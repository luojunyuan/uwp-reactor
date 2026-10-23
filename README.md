# uwp-reactor

`uwp-reactor` is a small, typed entry point for declarative Windows applications. It follows the
application, component, declaration, and window boundaries used by the current Windows Reactor,
and delegates native WinUI rendering and component reconciliation to that runtime.

The runtime is pinned as the `reference/windows-rs` submodule. Initialize it before building:

```sh
git submodule update --init --depth 1
cargo run -p demo-gallery --target x86_64-pc-windows-msvc
```

The current Windows Reactor targets WinUI 3 and the Windows App SDK. The gallery can be packaged
later with `winapp`; this repository does not include a package manifest or installer configuration.

## Demo gallery

The gallery example demonstrates typed component state, message-driven updates, native controls,
and a small multi-page application:

```sh
cargo run -p demo-gallery --target x86_64-pc-windows-msvc
```

The Windows App Runtime expected by the pinned Windows Reactor must be installed on the machine
that runs the example.
