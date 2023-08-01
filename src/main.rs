use bus::{
    active_connection::OrgFreedesktopNetworkManagerConnectionActive,
    devices::OrgFreedesktopNetworkManagerDevice, ip4config::OrgFreedesktopNetworkManagerIP4Config,
    network_manager::OrgFreedesktopNetworkManager,
};
use dbus::{blocking::Connection, Path};
use serde::Serialize;
use std::time::Duration;
mod bus;

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

fn get_interface(conn: &Connection, device_path: Path) -> Option<Interface> {
    let dev_proxy = conn.with_proxy(
        "org.freedesktop.NetworkManager",
        device_path,
        Duration::from_millis(5000),
    );

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
        let dev_name: Option<String> = match dev_proxy.interface() {
            Ok(s) => Some(s),
            Err(e) => {
                eprint!("Failed to get interface name: {}", e);
                None
            }
        };

        let dev_state: InterfaceState = match OrgFreedesktopNetworkManagerDevice::state(&dev_proxy)
        {
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
            match OrgFreedesktopNetworkManagerDevice::ip4_config(&dev_proxy) {
                Ok(ip_conf_path) => {
                    let ip_conf_proxy = conn.with_proxy(
                        "org.freedesktop.NetworkManager",
                        ip_conf_path,
                        Duration::from_millis(5000),
                    );
                    let addresses: Option<Vec<dbus::arg::PropMap>> =
                        match ip_conf_proxy.address_data() {
                            Ok(addr) => Some(addr),
                            Err(e) => {
                                eprintln!("Failed to get address data: {}", e);
                                None
                            }
                        };

                    // I could've put this is the match statement above but I already feel like I'm nesting too much
                    if let Some(addrs) = addresses {
                        if addrs.len() == 0 {
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
                match active_conn_proxy.id() {
                    Ok(n) => Some(n),
                    Err(_) => {
                        // Errors if connection is not connected
                        // also errors if it's not found but lets not worry about that right now
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to get active connection object: {}", e);
                None
            }
        };

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let conn = Connection::new_system()?;

    let mut interfaces: Vec<Interface> = vec![];

    let proxy = conn.with_proxy(
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        Duration::from_millis(5000),
    );

    let devices: Vec<dbus::Path<'static>> = proxy.get_devices()?;

    for device in devices {
        match get_interface(&conn, device) {
            Some(i) => interfaces.push(i),
            _ => {}
        }
    }

    println!("{}", serde_json::to_string(&interfaces).unwrap());
    Ok(())
}
