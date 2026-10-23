# uwp-reactor

`uwp-reactor` generates its own Rust projection of `Windows.UI.Xaml` from the WinRT RDL metadata
in `reference/windows-rs`, then builds a small component runtime on top of those bindings. It does
not depend on `windows-reactor`. The UWP application entry point follows the
`Application::Start` / `IApplicationOverrides::OnLaunched` lifecycle.

The root repository pins `reference/windows-rs` to a specific shallow commit. Initialize or refresh it with:

```sh
git submodule update --init --remote --depth 1
```

Build and run the gallery on Windows with:

```sh
cargo run -p demo-gallery --target x86_64-pc-windows-msvc
```

The example is an unpackaged UWP-style application entry point. Package deployment and app
manifests are intentionally left to `winapp`.
