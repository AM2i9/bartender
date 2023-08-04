use std::{
    io::Write,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use dbus::{
    arg::{self, PropMap},
    blocking::Connection,
    Message,
};
use serde::Serialize;

use crate::bus::{
    mpris::{OrgMprisMediaPlayer2, OrgMprisMediaPlayer2Player, OrgMprisMediaPlayer2PlayerSeeked},
    session::{OrgFreedesktopDBusNameOwnerChanged, OrgFreedesktopDBusPropertiesPropertiesChanged, OrgFreedesktopDBus},
};

#[derive(Serialize, Debug)]
enum PlayerStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Serialize, Debug)]
struct PlayerState {
    bus_name: Option<String>,
    player_name: Option<String>,
    status: Option<PlayerStatus>,
    song_name: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    position: Option<i64>,
    length: Option<u64>,
    playback_rate: Option<f64>,
}

impl PlayerState {
    pub fn clear(&mut self) {
        self.bus_name = None;
        self.player_name = None;
        self.status = None;
        self.song_name = None;
        self.artist = None;
        self.album = None;
        self.position = None;
        self.length = None;
        self.playback_rate = None;
    }

    pub fn dump_info(&self) {
        let mut stdout = std::io::stdout();
        match serde_json::to_string(&self) {
            Ok(out) => {
                let _ = stdout.write_all(&[out.as_bytes(), b"\n"].concat());
                let _ = stdout.flush();
            }
            Err(e) => {
                eprintln!("Failed to serialize output: {}", e);
            }
        };
    }

    pub fn fill_info(&mut self, conn: &Connection) {
        if let Some(bus) = &self.bus_name {
            let player =
                conn.with_proxy(bus, "/org/mpris/MediaPlayer2", Duration::from_millis(5000));

            self.status = player.playback_status().ok().map(|s| match s.as_str() {
                "Playing" => PlayerStatus::Playing,
                "Paused" => PlayerStatus::Paused,
                _ => PlayerStatus::Stopped,
            });

            self.player_name = player.identity().ok();
            let meta: Option<PropMap> = player.metadata().ok();
            self.position = player.position().ok();
            self.playback_rate = player.rate().ok();

            if let Some(m) = meta {
                let artist_list: Option<&Vec<String>> = arg::prop_cast(&m, "xesam:artist");
                self.artist = artist_list.map(|artists| artists.join(", "));

                self.song_name = arg::prop_cast(&m, "xesam:title").cloned();
                self.album = arg::prop_cast(&m, "xesam:album").cloned();
                self.length = arg::prop_cast(&m, "mpris:length").cloned();
            }
        }
    }

    pub fn bind_signals(&self, state: &Arc<Mutex<PlayerState>>, conn: &Connection) {
        let player = conn.with_proxy(
            self.bus_name.as_ref().unwrap(),
            "/org/mpris/MediaPlayer2",
            Duration::from_millis(5000),
        );
        {
            // Property Change signal
            let state = state.clone();
            let _ = player.match_signal(
                move |_sig: OrgFreedesktopDBusPropertiesPropertiesChanged,
                      conn: &Connection,
                      _: &Message| {
                    let mut state = state.lock().unwrap();
                    // I'm "lazy"
                    state.fill_info(conn);
                    state.dump_info();
                    true
                },
            );
        }
        {
            // Seek signal
            let state = state.clone();
            let _ = player.match_signal(
                move |sig: OrgMprisMediaPlayer2PlayerSeeked, _: &Connection, _: &Message| {
                    let mut state = state.lock().unwrap();
                    state.position = Some(sig.position);
                    state.dump_info();
                    true
                },
            );
        }
    }
}

fn find_player(conn: &Connection) -> Option<String> {
    let proxy = conn.with_proxy(
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        Duration::from_millis(5000),
    );
    let names = proxy.list_names().unwrap();

    if let Some(player_name) = names.iter().filter(|n| n.starts_with("org.mpris.MediaPlayer2.")).rev().last() {
        return Some(player_name.clone())
    }
    None
}

// I CAN SEE SOUNDS
pub fn musicwatcher() {
    let player_state = Arc::new(Mutex::new(PlayerState {
        bus_name: None,
        player_name: None,
        status: None,
        song_name: None,
        artist: None,
        album: None,
        position: None,
        length: None,
        playback_rate: None,
    }));

    // Position counter thread
    {
        let player_state = player_state.clone();
        thread::spawn(move || {
            let mut rate: f64;
            loop {
                {
                    // put into it's own block so mutex can be unlocked
                    let mut state = player_state.lock().unwrap();
                    if state.status.is_some()
                        && matches!(state.status.as_ref().unwrap(), PlayerStatus::Playing)
                        && state.length.is_some()
                    {
                        // There is a media player and it is playing and it has a

                        rate = state.playback_rate.unwrap_or(1.0);
                        state.position = state.position.map(|p| p + (rate * 1000000.0) as i64);

                        state.dump_info();
                    } else {
                        rate = 0.2;
                    }
                }
                thread::sleep(Duration::from_secs_f64(rate));
            }
        });
    }

    match Connection::new_session() {
        Ok(conn) => {
            let proxy = conn.with_proxy(
                "org.freedesktop.DBus",
                "/org/freedesktop/DBus",
                Duration::from_millis(5000),
            );

            player_state.lock().unwrap().bus_name = find_player(&conn);
            player_state.lock().unwrap().dump_info();

            {
                let _ = proxy.match_signal(
                    move |sig: OrgFreedesktopDBusNameOwnerChanged,
                          conn: &Connection,
                          _: &Message| {
                        let mut state = player_state.lock().unwrap();

                        if sig.arg0.starts_with("org.mpris.MediaPlayer2.") {
                            if sig.arg1.is_empty()
                                && !sig.arg2.is_empty()
                                && state.bus_name.is_none()
                            {
                                // Media player opened
                                state.bus_name = Some(sig.arg0);
                                state.fill_info(conn);
                                state.bind_signals(&player_state, conn);
                                state.dump_info();
                            } else if !sig.arg1.is_empty()
                                && sig.arg2.is_empty()
                                && state.bus_name.is_some()
                                && *state.bus_name.as_ref().unwrap() == sig.arg0
                            {
                                // open media player closed
                                state.clear();

                                // Look for another open player
                                if let Some(player) = find_player(conn) {
                                    state.bus_name = Some(player);
                                    state.fill_info(conn);
                                    state.bind_signals(&player_state, conn);
                                };

                                state.dump_info();
                            }
                        }
                        true
                    },
                );
            }

            loop {
                if let Err(e) = conn.process(Duration::from_millis(1000)) {
                    eprintln!("Failed to process incomming messages: {}", e);
                }
            }
        }
        Err(e) => eprintln!("Failed to connect to system dbus: {}", e),
    };
}
