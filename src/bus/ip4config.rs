// This code was autogenerated with `dbus-codegen-rust -d org.freedesktop.NetworkManager -p /org/freedesktop/NetworkManager/IP4Config/1 -o ip4config.rs --system-bus`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusProperties {
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {

    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>, )| Ok(r.0, ))
    }

    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (arg::PropMap, )| Ok(r.0, ))
    }

    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, value, ))
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopNetworkManagerIP4Config {
    fn addresses(&self) -> Result<Vec<Vec<u32>>, dbus::Error>;
    fn address_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error>;
    fn gateway(&self) -> Result<String, dbus::Error>;
    fn routes(&self) -> Result<Vec<Vec<u32>>, dbus::Error>;
    fn route_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error>;
    fn nameserver_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error>;
    fn nameservers(&self) -> Result<Vec<u32>, dbus::Error>;
    fn domains(&self) -> Result<Vec<String>, dbus::Error>;
    fn searches(&self) -> Result<Vec<String>, dbus::Error>;
    fn dns_options(&self) -> Result<Vec<String>, dbus::Error>;
    fn dns_priority(&self) -> Result<i32, dbus::Error>;
    fn wins_server_data(&self) -> Result<Vec<String>, dbus::Error>;
    fn wins_servers(&self) -> Result<Vec<u32>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopNetworkManagerIP4Config for blocking::Proxy<'a, C> {

    fn addresses(&self) -> Result<Vec<Vec<u32>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Addresses")
    }

    fn address_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "AddressData")
    }

    fn gateway(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Gateway")
    }

    fn routes(&self) -> Result<Vec<Vec<u32>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Routes")
    }

    fn route_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "RouteData")
    }

    fn nameserver_data(&self) -> Result<Vec<arg::PropMap>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "NameserverData")
    }

    fn nameservers(&self) -> Result<Vec<u32>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Nameservers")
    }

    fn domains(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Domains")
    }

    fn searches(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "Searches")
    }

    fn dns_options(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "DnsOptions")
    }

    fn dns_priority(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "DnsPriority")
    }

    fn wins_server_data(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "WinsServerData")
    }

    fn wins_servers(&self) -> Result<Vec<u32>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.NetworkManager.IP4Config", "WinsServers")
    }
}
