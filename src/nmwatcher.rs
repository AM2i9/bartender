use crate::bus::{
    active_connection::OrgFreedesktopNetworkManagerConnectionActive,
    devices::{self, OrgFreedesktopNetworkManagerDevice},
    ip4config::OrgFreedesktopNetworkManagerIP4Config,
    network_manager::{OrgFreedesktopNetworkManager, OrgFreedesktopNetworkManagerDeviceAdded},
};
use dbus::{
    blocking::{Connection, Proxy},
    Message,
};
use serde::Serialize;
use std::{io::Write, time::Duration};

#[derive(Serialize, Debug)]
enum InterfaceState {
    Connected,
    Connecting,
    Disconnected,
    Unavailable,
}

#[derive(Serialize, Debug)]
enum InterfaceType {
    Wired,
    Wireless,
    Other,
}

#[derive(Serialize, Debug)]
struct Interface {
    name: Option<String>,
    conn_type: InterfaceType,
    connection_name: Option<String>,
    ip: Option<(String, u64)>,
    state: InterfaceState,
}

fn make_interface(
    conn: &Connection,
    dev_proxy: &Proxy<&Connection>,
    init: bool,
) -> Option<Interface> {
    let dev_type: InterfaceType = match dev_proxy.device_type() {
        Ok(1) => InterfaceType::Wired,
        Ok(2) => InterfaceType::Wireless,
        Err(e) => {
            eprintln!("Failed to get device type: {}", e);
            InterfaceType::Other
        }
        _ => InterfaceType::Other,
    };

    if !matches!(dev_type, InterfaceType::Other) {
        let dev_name: Option<String> = dev_proxy.interface().ok();

        let dev_state: InterfaceState = match OrgFreedesktopNetworkManagerDevice::state(dev_proxy) {
            Ok(30) => InterfaceState::Disconnected,
            Ok(40..=90) => InterfaceState::Connecting,
            Ok(100) => InterfaceState::Connected,
            Err(e) => {
                eprintln!("Failed to get device state: {}", e);
                InterfaceState::Unavailable
            }
            _ => InterfaceState::Unavailable,
        };

        let ip_info: Option<(String, u64)> =
            match OrgFreedesktopNetworkManagerDevice::ip4_config(dev_proxy) {
                Ok(ip_conf_path) => {
                    let ip_conf_proxy = conn.with_proxy(
                        "org.freedesktop.NetworkManager",
                        ip_conf_path,
                        Duration::from_millis(5000),
                    );

                    let addresses: Option<Vec<dbus::arg::PropMap>> =
                        ip_conf_proxy.address_data().ok();

                    // I could've put this is the match statement above but I already feel like I'm nesting too much
                    if let Some(addrs) = addresses {
                        if addrs.is_empty() {
                            None
                        } else {
                            let address = &addrs[0].get("address").unwrap().0;
                            let prefix = &addrs[0].get("prefix").unwrap().0;
                            Some((
                                String::from((*address).as_str().unwrap()),
                                (*prefix).as_u64().unwrap(),
                            ))
                        }
                    } else {
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Failed to get IP4Config: {}", e);
                    None
                }
            };

        let conn_name: Option<String> = match dev_proxy.active_connection() {
            Ok(active_conn_path) => {
                let active_conn_proxy = conn.with_proxy(
                    "org.freedesktop.NetworkManager",
                    active_conn_path,
                    Duration::from_millis(5000),
                );
                active_conn_proxy.id().ok()
            }
            Err(e) => {
                eprintln!("Failed to get active connection object: {}", e);
                None
            }
        };

        // Start listening for events when device is first detected
        if init {
            add_statechange_listener(dev_proxy)
        }

        Some(Interface {
            name: dev_name,
            conn_type: dev_type,
            connection_name: conn_name,
            ip: ip_info,
            state: dev_state,
        })
    } else {
        None
    }
}

fn add_statechange_listener(dev_proxy: &Proxy<&Connection>) {
    let _ = dev_proxy.match_signal(
        |sig: devices::OrgFreedesktopNetworkManagerDeviceStateChanged,
         conn: &Connection,
         _: &Message| {
            let _ = make_n_dump_devices(conn, false);
            sig.reason != 36
        },
    );
}

fn make_n_dump_devices(conn: &Connection, init: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout().lock();

    let mut interfaces: Vec<Interface> = vec![];

    let proxy = conn.with_proxy(
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        Duration::from_millis(5000),
    );

    let devices: Vec<dbus::Path<'static>> = proxy.get_devices()?;

    for device in devices {
        let dev_proxy = conn.with_proxy(
            "org.freedesktop.NetworkManager",
            device,
            Duration::from_millis(5000),
        );

        if let Some(i) = make_interface(conn, &dev_proxy, init) {
            interfaces.push(i);
        }
    }

    if init {
        // Device add event
        let _ = proxy.match_signal(
            |sig: OrgFreedesktopNetworkManagerDeviceAdded, conn: &Connection, _: &Message| {
                let dev_proxy = conn.with_proxy(
                    "org.freedesktop.NetworkManager",
                    sig.device_path,
                    Duration::from_millis(5000),
                );

                add_statechange_listener(&dev_proxy);
                true
            },
        );
    }

    match serde_json::to_string(&interfaces) {
        Ok(out) => {
            let _ = stdout.write_all(&[out.as_bytes(), b"\n"].concat());
            let _ = stdout.flush();
        }
        Err(e) => {
            eprintln!("Failed to serialize output: {}", e);
        }
    };

    Ok(())
}

pub fn nmwatcher() {
    match Connection::new_system() {
        Ok(conn) => {
            if let Err(e) = make_n_dump_devices(&conn, true) {
                eprintln!("Failed to display devices: {}", e);
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
