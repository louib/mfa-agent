# Contributing

## Logging

You can set the log level using the `MFA_AGENT_LOG_LEVEL` environment variable.
See [the log documentation](https://docs.rs/log/0.4.11/log/enum.LevelFilter.html) for the available log levels.
The default level is `Info`.

## Debugging
The `MFA_AGENT_IS_DEV` and `MFA_AGENT_LOG_LEVEL` environment variables can be used during
development:

```
MFA_AGENT_IS_DEV=true MFA_AGENT_LOG_LEVEL=debug ./target/debug/mfa-agent
```

## References
* https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/#structs
* https://gtk-rs.org/gtk4-rs/stable/latest/book/
* https://docs.gtk.org/gtk4/class.Widget.html
* https://docs.gtk.org/gtk4/css-properties.html#gtk-css-properties
* https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes
* https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.1/
* https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.1/visual-index.html
* https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Builder.html#gtkbuilder-ui-definitions
* https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/
* https://docs.rs/crate/async-std/latest
* https://developer.gnome.org/hig/index.html
* https://github.com/gtk-rs/gtk3-rs/tree/master/examples
* https://github.com/bluez/bluer/tree/master/bluer/examples
* https://docs.rs/bluer/0.15.0/bluer/
* https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/
* https://github.com/diwic/dbus-rs#readme
* https://fidoalliance.org/specs/u2f-specs-master/fido-u2f-hid-protocol.html
* https://developers.yubico.com/U2F/Libraries/Client_error_codes.html
* https://developer.mozilla.org/en-US/docs/Web/CSS/Syntax
* The Comparison of Rust Keepass Libraries at https://crates.io/crates/kdbx-rs
