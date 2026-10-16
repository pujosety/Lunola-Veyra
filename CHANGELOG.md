# Changelog

All notable changes to Lunola Veyra are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-10-16

### Rebranding: DeckCraft → Lunola Veyra

This release marks the official rebranding of DeckCraft to **Lunola Veyra**, a core
application in the [Lunola](https://lunola.com/) ecosystem of open-source creative tools.
The product name, visual identity, and ecosystem integration are now aligned with the Lunola
brand. The tagline is **"Make Ideas Shine."**

### Changed

- **Product identity**: DeckCraft is now Lunola Veyra throughout the codebase, UI, CLI, and
  documentation. The CLI binary remains `deckcraft-cli` for backward compatibility.
- **Visual identity**: New accent color `#F18A49` (Lunola orange) applied across the UI theme,
  replacing the previous DeckCraft branding.
- **Brand assets**: Added new icons, logo, and splash screen under `assets/brand/` and
  `assets/app-icon/`. All icon paths updated to use the new Lunola Veyra brand assets.
- **Ecosystem integration**: Lunola Veyra is now presented as one of the Crafting Apps
  alongside PhotoCraft, VectorCraft, FilmCraft, LightCraft, PdfCraft, EffectCraft, and
  DesignCraft.
- **UI modernization**: Updated theme, dialogs, icons, and show/presenter views to use the
  new Lunola Veyra visual language.
- **Documentation**: README, ROADMAP, and all product documentation updated to reflect the
  new name, branding, and ecosystem positioning.

### Added

- `CHANGELOG.md` — this file.
- `KNOWN_ISSUES.md` — known limitations and build requirements.
- `DELIVERY.md` — delivery summary for the Lunola Veyra rebranding project.
- Brand assets: `assets/brand/deckcraft-icon.svg`, `deckcraft-logo.svg`, `deckcraft-logo.png`,
  `deckcraft-splash.png`.
- App icons in all standard sizes (16–1024 px, `.ico`, `.icns`, `.png`, `.svg`).

### Migration

- File format IDs and crate names are unchanged. Existing `.deckcraft` files, PPTX files,
  and all project data remain fully compatible.
- The CLI tool `deckcraft-cli` retains its name; no command-line changes are required.
- Configuration directories on Linux (`~/.config/deckcraft/`), macOS
  (`~/Library/Application Support/Lunola Veyra/`), and Windows
  (`%APPDATA%\Lunola Veyra\`) now use the new product name where applicable.

[1.0.0]: https://github.com/lunola/deckcraft/releases/tag/v1.0.0
