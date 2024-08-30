//! # D-Bus interface proxy for: `org.kde.StatusNotifierWatcher`
//!
//! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
//! Source: `dbus_status_notifier_watcher.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::proxy;
#[proxy(
    default_service = "org.kde.StatusNotifierWatcher",
    interface = "org.kde.StatusNotifierWatcher",
    default_path = "/StatusNotifierWatcher"
)]
trait StatusNotifierWatcher {
    /// RegisterStatusNotifierHost method
    fn register_status_notifier_host(&self, service: &str) -> zbus::Result<()>;

    /// RegisterStatusNotifierItem method
    fn register_status_notifier_item(&self, service: &str) -> zbus::Result<()>;

    /// StatusNotifierHostRegistered signal
    #[zbus(signal)]
    fn status_notifier_host_registered(&self) -> zbus::Result<()>;

    /// StatusNotifierHostUnregistered signal
    #[zbus(signal)]
    fn status_notifier_host_unregistered(&self) -> zbus::Result<()>;

    /// StatusNotifierItemRegistered signal
    #[zbus(signal)]
    fn status_notifier_item_registered(&self, service: &str) -> zbus::Result<()>;

    /// StatusNotifierItemUnregistered signal
    #[zbus(signal)]
    fn status_notifier_item_unregistered(&self, service: &str) -> zbus::Result<()>;

    /// IsStatusNotifierHostRegistered property
    #[zbus(property)]
    fn is_status_notifier_host_registered(&self) -> zbus::Result<bool>;

    /// ProtocolVersion property
    #[zbus(property)]
    fn protocol_version(&self) -> zbus::Result<i32>;

    /// RegisteredStatusNotifierItems property
    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> zbus::Result<Vec<String>>;
}
