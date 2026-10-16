# Known Issues

This document lists known limitations, build requirements, and outstanding issues for
Lunola Veyra 1.0.0.

## Build Requirements

### Rust toolchain

Lunola Veyra requires **Rust 1.95 or later**. The workspace `Cargo.toml` specifies
`rust-version = "1.95"`. Verified with **rustc 1.99.1 stable** — `cargo check -p deckcraft-ui-egui` passes.

### Krilla PDF dependency

The project depends on [Krilla](https://github.com/LaurenzV/krilla) 0.8.2 through the
`craft-pdf` crate. Krilla is vendored in `craft-libs` and is **not available as a git
dependency** in all environments. If you encounter build failures related to Krilla or
`craft-pdf`, ensure that:

1. The `craft-libs` submodule or vendored copy is present.
2. You are building from the repository root, not from a partial clone.

### System dependencies

- **Linux**: ALSA headers are required for slide-show audio. Install `alsa-lib-devel`
  (Fedora), `libasound2-dev` (Debian/Ubuntu), or `alsa-lib` (Arch).
- **Fonts**: For the best experience, clone
  [craft-fonts](https://github.com/lunola/craft-fonts) next to this repo
  (`../craft-fonts`). Without it, Lunola Veyra falls back to system fonts.

## Testing status

### Automated tests

**`cargo check -p deckcraft-ui-egui`**: ✅ PASS (rustc 1.99.1)

Full `cargo test --workspace`: NOT RUN. The existing test suite (unit tests, integration tests, and parity checks) should be run
before publishing a release:

```sh
cargo test --workspace
cargo xtask parity
```

### GUI verification

**No GUI verification was performed** as part of this delivery. The desktop application
should be manually tested on all target platforms (macOS, Windows, Linux, FreeBSD) before
release. Key areas to verify:

- Theme and accent color rendering (`#F18A49`)
- Icon and logo display in the title bar, About window, and system integration
- Splash screen appearance
- Dark mode consistency with the new branding

## Known limitations

### Alpha-stage product

Lunola Veyra is in **alpha**. While all six of PowerPoint's core workflows pass the alpha
gate (build, open/save `.pptx`, present, add tables/charts, share as PDF, review), there
are known gaps in feature depth, fidelity, and polish. See [ROADMAP.md](ROADMAP.md) and
`docs/gaps.md` for details.

### PPTX round-trip fidelity

Opening and saving PowerPoint `.pptx` files is supported, but round-trip fidelity on
real-world decks has not been fully tested. Some advanced features (EMF/SVG pictures,
MicroType Express font subsets, certain SmartArt and OLE constructs) may not survive a
round trip perfectly.

### Localization

The UI is available in **English only**. No string catalog exists yet. See
`docs/localization-parity.md` for the localization roadmap.

### Platform-specific issues

- **Windows**: The desktop app initializes DirectX 12 by default. Some AMD graphics
  drivers may crash on launch; set `WGPU_BACKEND=vulkan` as a workaround.
- **Web**: The WebAssembly build runs in modern browsers but has not been tested on all
  browser/OS combinations.

### Performance

Performance has not been systematically measured. Large decks with many high-resolution
images or complex animations may exhibit slowdowns. GPU rasterization and hardware video
decode are not yet implemented.

## Reporting issues

Please report issues on [GitHub](https://github.com/lunola/deckcraft/issues). Include:

- Your operating system and Rust version (`rustc --version`)
- The contents of `logs/deckcraft.log` (or the platform equivalent)
- Steps to reproduce the issue
