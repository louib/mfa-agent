# Contributing

## Logging

You can set the log level using the `MFA_AGENT_LOG_LEVEL` environment variable.
See [the log documentation](https://docs.rs/log/0.4.11/log/enum.LevelFilter.html) for the available log levels.
The default level is `Info`.

## Debugging
The `MFA_AGENT_IS_DEV` and `MFA_AGENT_LOG_LEVEL` environment variables can be used during
development:

```
MFA_AGENT_IS_PROXY=true MFA_AGENT_IS_DEV=true MFA_AGENT_LOG_LEVEL=debug ./target/debug/mfa-agent
```

## References
* https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/#structs
* https://gtk-rs.org/gtk4-rs/stable/latest/book/
* https://docs.gtk.org/gtk4/class.Widget.html
* https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.1/
* https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/
* https://docs.rs/crate/async-std/latest
* https://github.com/gtk-rs/gtk3-rs/tree/master/examples
* https://github.com/bluez/bluer/tree/master/bluer/examples
* https://docs.rs/bluer/0.15.0/bluer/
* https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/
* https://github.com/diwic/dbus-rs#readme
* https://developer.mozilla.org/en-US/docs/Web/CSS/Syntax
* The Comparison of Rust Keepass Libraries at https://crates.io/crates/kdbx-rs
