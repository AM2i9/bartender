use crate::bus::bat_disp::{
    OrgFreedesktopDBusPropertiesPropertiesChanged, OrgFreedesktopUPowerDevice,
};
use dbus::{
    blocking::{Connection, Proxy},
    Message,
};
use serde::Serialize;
use std::{io::Write, time::Duration};
use notify_rust::{Notification, Urgency};

#[derive(Serialize, Debug)]
enum BatteryState {
    Unknown,
    Charging,
    Discharging,
    Empty,
    FullyCharged,
    PendingCharge,
    PendingDischarge,
}

#[derive(Serialize, Debug)]
struct Battery {
    state: BatteryState,
    charge: f64,
    time_to: i64,
}

fn fetch_battery(bat_proxy: &Proxy<&Connection>) -> Battery {
    let state = match bat_proxy.state() {
        Ok(1) => BatteryState::Charging,
        Ok(2) => BatteryState::Discharging,
        Ok(3) => BatteryState::Empty,
        Ok(4) => BatteryState::FullyCharged,
        Ok(5) => BatteryState::PendingCharge,
        Ok(6) => BatteryState::PendingDischarge,
        Err(e) => {
            eprintln!("Failed to get battery state: {}", e);
            BatteryState::Unknown
        }
        _ => BatteryState::Unknown,
    };

    let charge: f64 = bat_proxy.percentage().unwrap_or_else(|e| {
        eprintln!("Failed to get charge percentage: {}", e);
        0.0
    });

    let time_to = match state {
        BatteryState::Charging => bat_proxy.time_to_full().unwrap_or_else(|e| {
            eprintln!("Failed to get time to full: {}", e);
            0
        }),
        _ => bat_proxy.time_to_empty().unwrap_or_else(|e| {
            eprintln!("Failed to get time to empty: {}", e);
            0
        }),
    };

    Battery {
        state,
        charge,
        time_to,
    }
}

fn check_n_dump_battery(conn: &Connection) {
    let mut stdout = std::io::stdout().lock();

    let bat_proxy = conn.with_proxy(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower/devices/DisplayDevice",
        Duration::from_millis(5000),
    );

    let battery = &fetch_battery(&bat_proxy);

    if battery.charge <= 5.0 && matches!(battery.state, BatteryState::Discharging) {
        let _ = Notification::new()
            .summary("Battery critically low!")
            .body(&format!("{}% left. Computer may shut down soon.", battery.charge))
            .urgency(Urgency::Critical)
            .show();
    }

    match serde_json::to_string(battery) {
        Ok(out) => {
            let _ = stdout.write_all(&[out.as_bytes(), b"\n"].concat());
            let _ = stdout.flush();
        }
        Err(e) => {
            eprintln!("Failed to serialize output: {}", e);
        }
    };
}

pub fn batwatcher() {
    match Connection::new_system() {
        Ok(conn) => {
            check_n_dump_battery(&conn);

            let bat_proxy = conn.with_proxy(
                "org.freedesktop.UPower",
                "/org/freedesktop/UPower/devices/DisplayDevice",
                Duration::from_millis(5000),
            );

            let _ = bat_proxy.match_signal(
                |_: OrgFreedesktopDBusPropertiesPropertiesChanged, c: &Connection, _: &Message| {
                    check_n_dump_battery(c);
                    true
                },
            );

            loop {
                if let Err(e) = conn.process(Duration::from_millis(1000)) {
                    eprintln!("Failed to process incomming messages: {}", e)
                }
            }
        }
        Err(e) => eprintln!("Failed to connect to system dbus: {}", e),
    };
}
