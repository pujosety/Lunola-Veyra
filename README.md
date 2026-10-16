<p align="center">
  <a href="https://lunola.com/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/brand/artcraft-logo-white.svg">
      <img alt="Lunola" src="docs/brand/artcraft-logo.svg" width="200">
    </picture>
  </a>
</p>

<h1 align="center">Lunola Veyra</h1>

<p align="center">
  <b>Presentations and slide shows; an open-source, clean-room reimplementation of Microsoft PowerPoint, rebuilt in pure Rust.</b>
</p>

<p align="center">
  Build decks with themes, layouts, shapes, pictures, tables, charts, transitions and animations,
  open and save PowerPoint files, and present them full screen. Native on macOS, Windows, Linux
  and FreeBSD, and in the browser. Every button is also a command that the CLI and AI agents
  (through MCP) can drive.
</p>

<p align="center">
  <img alt="Status: early development" src="https://img.shields.io/badge/status-early%20development-f26b1d?style=flat-square">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-b7410e?style=flat-square">
  <img alt="Platforms" src="https://img.shields.io/badge/platforms-macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20BSD%20%C2%B7%20Web-555?style=flat-square">
</p>

<p align="center">
  <a href="https://discord.gg/lunola"><img alt="Join the Lunola community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<p align="center">
  <a href="https://lunola.com/apps/lunola-veyra"><b>Lunola Veyra on lunola.com</b></a> ·
  <a href="https://lunola.com/">Lunola</a> ·
  <a href="https://lunola.com/apps">All Crafting Apps</a>
</p>

> [!NOTE]
> **Lunola is a community of artists from all walks of life.** Painters, photographers,
> filmmakers, illustrators, designers, animators, hobbyists, and people who picked up a pencil
> last week. If you make things, you're one of us. **[Come say hi on Discord](https://discord.gg/lunola).**

<p align="center">
  <img alt="Lunola Veyra editing a slide: ribbon, slide thumbnails with sections, a process diagram on the canvas, notes and status bar" src="docs/images/editor.png" width="900">
</p>

<p align="center">
  <a href="#what-works-today">What works</a> ·
  <a href="#getting-started">Getting started</a> ·
  <a href="#agents-cli-and-mcp">Agents, CLI and MCP</a> ·
  <a href="#roadmap">Roadmap</a> ·
  <a href="#downloads">Downloads</a> ·
  <a href="#the-crafting-apps">The Crafting Apps</a> ·
  <a href="#license-and-credits">License</a>
</p>

<p align="center">
  <a href="CHANGELOG.md">Changelog</a> ·
  <a href="KNOWN_ISSUES.md">Known Issues</a> ·
  <a href="ROADMAP.md">Roadmap</a> ·
  <a href="DELIVERY.md">Delivery</a>
</p>

## What works today

Lunola Veyra is in alpha, heading for beta (see the [roadmap](ROADMAP.md)).
Today it can:

- **Make decks**: slide masters and eleven layouts with placeholders that inherit position and
  formatting, eight original themes with colour and font schemes, sections, notes, comments,
  headers and footers, custom slide sizes.
- **Draw**: 150+ preset shapes with adjust handles, connectors that stay glued to shapes, freeform,
  curve and scribble tools, merge shapes (union, combine, fragment, intersect, subtract), gradient,
  picture and pattern fills, outlines with dashes and arrowheads, shadows, glow, soft edges and
  reflections.
- **Write**: in-place text editing with full font and paragraph formatting, bullets and numbering,
  columns, autofit, vertical text, fields and hyperlinks.
- **Add content**: pictures with crop and corrections, tables with styles, charts, SmartArt basics,
  audio and video (MP3, AAC, FLAC, ALAC, Ogg, Opus, WAV, AIFF; H.264, HEVC, VP9, AV1).
- **Present**: transitions (including Morph), entrance/emphasis/exit and motion-path animations,
  by-paragraph builds, full-screen slide show, presenter view, pen, rehearse timings, media playback.
- **Exchange files**: open and save PowerPoint `.pptx`, export PDF (slides, notes pages, handouts
  with selectable text), PNG/JPEG, an animated GIF of the deck and outlines. Unsaved work is kept by AutoRecover.
- **Automate**: 200+ commands with undo, all scriptable from the CLI, a JSON control channel and an
  MCP server for AI agents.

<table>
  <tr>
    <td width="50%"><img alt="A selected shape with the Shape Format tab" src="docs/images/shape-format.png"><br><sub>Shape Format contextual tab and selection handles</sub></td>
    <td width="50%"><img alt="The Format Shape pane editing a gradient fill" src="docs/images/format-pane.png"><br><sub>Format Shape pane: gradient type, direction and stops</sub></td>
  </tr>
  <tr>
    <td width="50%"><img alt="Slide Sorter view with transitions under each slide" src="docs/images/slide-sorter.png"><br><sub>Slide Sorter with each slide's transition</sub></td>
    <td width="50%"><img alt="Lunola Veyra in dark mode showing a chart slide" src="docs/images/dark-mode.png"><br><sub>Dark mode, with a native chart</sub></td>
  </tr>
</table>

<p align="center">
  <img alt="All eight slides of the Northern Lights sample deck" src="docs/images/sample-deck.png" width="900"><br>
  <sub>The built-in sample deck, rendered by <code>deckcraft-cli render --sample --all</code></sub>
</p>

## Getting started

PDF export uses the shared `craft-pdf` writer with import features disabled. See
[`crates/pdf/README.md`](crates/pdf/README.md) for the dependency and validation requirements.

```sh
git clone https://github.com/lunola/lunola-veyra
cd deckcraft
cargo run --release -p deckcraft-cli -- render --sample --all --scale 1 out/
```

On Linux, the build also needs the ALSA headers for slide-show sound: `alsa-lib-devel` on Fedora,
`libasound2-dev` on Debian and Ubuntu, `alsa-lib` on Arch. Without them it stops at `alsa-sys`.

Fonts come from [craft-fonts](https://github.com/lunola/craft-fonts): clone it next to this repo
(`../craft-fonts`) and local builds pick it up automatically; without it Lunola Veyra uses your system
fonts.

Each [GitHub release](https://github.com/lunola/lunola-veyra/releases) has ready-made builds, on Linux as
an AppImage, a `.deb`, an `.rpm` and a tarball. On Gentoo, the community [::snakebyte
overlay](https://github.com/switch87/snakebyte-overlay) packages the Linux release as
`app-office/deckcraft-bin` (not maintained by the Lunola Veyra team):

```sh
eselect repository add snakebyte git https://github.com/switch87/snakebyte-overlay.git
emaint sync -r snakebyte
echo 'app-office/deckcraft-bin ~amd64' >> /etc/portage/package.accept_keywords/deckcraft
emerge --ask app-office/deckcraft-bin
```

### Logs

The desktop app writes its `log` records to standard error and to `logs/deckcraft.log` in the
settings directory, beside `ui.json` and `prefs.json` (Linux `~/.config/deckcraft/logs/`, or
`$XDG_CONFIG_HOME/deckcraft/logs/`; macOS `~/Library/Application Support/Lunola Veyra/logs/`; Windows
`%APPDATA%\Lunola Veyra\logs\`). A start from a desktop menu or the Dock has no terminal, so this file
is what to attach to a bug report: the crash guard's panic report, a failed crash-recovery save,
PowerPoint parts skipped on import and audio output problems land there. Each launch moves the
previous log to `deckcraft.1.log` (and that one to `deckcraft.2.log`), so the log of a run that
crashed survives the next start. The file stops growing at 16 MiB. `--version` writes no file.

| Variable | Effect |
|---|---|
| `RUST_LOG` | Log levels for standard error and the log file. Default: `info` for Lunola Veyra's own crates, `warn` for everything else. env_logger-style directives replace that, e.g. `RUST_LOG=debug`, `RUST_LOG=warn,deckcraft_pptx=trace` or `RUST_LOG=info,wgpu_core=warn`; a directive ending in `*` covers every target starting with it (`deckcraft*=debug`). |

The logger is `apps/deckcraft/src/logging.rs`; the realtime audio callback does not log. The web
app logs to the browser console instead.

## Agents, CLI and MCP

Every action in Lunola Veyra is a command with a stable id, so people, scripts and AI agents use the
same verbs:

```sh
deckcraft-cli commands format.          # list commands
deckcraft-cli run --sample --cmd 'slide.new={"layout":"titleOnly","title":"Hello"}' --save hello.deckcraft
deckcraft-cli mcp                       # MCP server over stdio (headless)
deckcraft-cli mcp --connect 7990        # drive the running app (deckcraft --control 7990)
deckcraft-cli mcp --automation-read-root DIR --automation-write-root DIR   # confine the headless agent to folders
```

## Roadmap

Lunola Veyra is in **alpha**. It covers **84% of PowerPoint's features by breadth** (92% of the core
ones, measured) and is about **55% of the way to replacing PowerPoint for real work** (estimated).
See [ROADMAP.md](ROADMAP.md) for the stage, numbers by dimension and what's next,
[docs/target-app-parity.md](docs/target-app-parity.md) for the full assessment,
[docs/gaps.md](docs/gaps.md) for every known shortfall and
[docs/parity-checklist.md](docs/parity-checklist.md) for the feature-by-feature checklist.

## Downloads

**Download Lunola Veyra** from GitHub: the [latest release](https://github.com/lunola/lunola-veyra/releases/latest) has every build listed below, and [all releases](https://github.com/lunola/lunola-veyra/releases) has earlier versions and their notes. `<ver>` in the file names is the version number, and `SHA256SUMS.txt` lists a checksum for every file.

### Windows

| Build | Installer | Portable |
|---|---|---|
| x64 (64-bit Intel/AMD) | `deckcraft-<ver>-windows-x64.msi` | `deckcraft-<ver>-windows-x64-portable.zip` |
| arm64 (Snapdragon and other ARM PCs) | `deckcraft-<ver>-windows-arm64.msi` | `deckcraft-<ver>-windows-arm64-portable.zip` |
| x86 (32-bit) | `deckcraft-<ver>-windows-x86.msi` | `deckcraft-<ver>-windows-x86-portable.zip` |

Installers and executables are code-signed.

**If the app doesn't open on Windows:** the desktop app initializes only DirectX 12 by default.
Letting wgpu also create an OpenGL instance can crash some graphics drivers (AMD's
`atio6axx.dll`) before the window appears, so the app would flash in Task Manager and quit.
`WGPU_BACKEND` overrides the default for troubleshooting (for example `dx12` or `vulkan`). In
PowerShell, from the folder containing the executable:

```powershell
$env:WGPU_BACKEND = "vulkan"
& .\deckcraft.exe
Remove-Item Env:WGPU_BACKEND                     # restore the default for later launches
```

An explicit `gl` override can bring the driver crash back on affected systems. The macOS, Linux
and web backend defaults are unchanged.

DirectX 12 shaders are compiled with FXC, which is part of Windows, so no `dxcompiler.dll` or
`dxil.dll` is needed ("Parent device is lost" at start, #153). `$env:WGPU_DX12_COMPILER = "dxc"`
(or `"auto"`) selects wgpu's DXC handling again for troubleshooting.

### macOS

| Build | File | Notes |
|---|---|---|
| App, universal (Apple silicon + Intel) | `deckcraft-<ver>-macos-universal.dmg` | Signed and notarized |
| Command-line tool, universal | `deckcraft-cli-<ver>-macos-universal.zip` | Signed and notarized |

### Linux

| Format | x86_64 | aarch64 (ARM64) | Notes |
|---|---|---|---|
| AppImage | `deckcraft-<ver>-linux-x86_64.AppImage` | `deckcraft-<ver>-linux-aarch64.AppImage` | Runs anywhere; updates itself with [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate) (`.zsync` files) |
| Flatpak | `deckcraft-<ver>-linux-x86_64.flatpak` | `deckcraft-<ver>-linux-aarch64.flatpak` | Sandboxed; `flatpak install --user <file>` |
| Debian/Ubuntu | `deckcraft-<ver>-linux-x86_64.deb` | `deckcraft-<ver>-linux-aarch64.deb` | |
| Fedora/RHEL/openSUSE | `deckcraft-<ver>-linux-x86_64.rpm` | `deckcraft-<ver>-linux-aarch64.rpm` | |
| Tarball | `deckcraft-<ver>-linux-x86_64.tar.gz` | `deckcraft-<ver>-linux-aarch64.tar.gz` | Unpack anywhere |

RISC-V 64 (riscv64): `deckcraft-<ver>-linux-riscv64.tar.gz`, a tarball only (cross-compiled; needs glibc 2.39+, e.g. Ubuntu 24.04+).

### FreeBSD

| Build | File |
|---|---|
| x86_64 | `deckcraft-<ver>-freebsd-x86_64.tar.gz` |

### Web (WebAssembly)

| Build | File | Notes |
|---|---|---|
| Static site | `deckcraft-web-<ver>.zip` | Runs in a modern browser; host it on any static server |

## The Crafting Apps

Lunola Veyra is one of the **Crafting Apps**: free, open-source creative tools from the
[Lunola](https://lunola.com/) team, each written from scratch in Rust and each able to
stand on its own.

| | App | What it's for | Code | Learn more |
|:-:|---|---|---|---|
| <img src="https://raw.githubusercontent.com/lunola/photocraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.photocraft.png" alt="" width="32" height="32"> | **PhotoCraft** | Image editing: layers, masks, type and real PSD files | [GitHub](https://github.com/lunola/photocraft) | [Website](https://lunola.com/apps/photocraft) |
| <img src="https://raw.githubusercontent.com/lunola/vectorcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.vectorcraft.png" alt="" width="32" height="32"> | **VectorCraft** | Vector illustration | [GitHub](https://github.com/lunola/vectorcraft) | [Website](https://lunola.com/apps/vectorcraft) |
| <img src="https://raw.githubusercontent.com/lunola/filmcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.filmcraft.png" alt="" width="32" height="32"> | **FilmCraft** | Video editing, color and sound | [GitHub](https://github.com/lunola/filmcraft) | [Website](https://lunola.com/apps/filmcraft) |
| <img src="https://raw.githubusercontent.com/lunola/lightcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.lightcraft.png" alt="" width="32" height="32"> | **LightCraft** | Photo library and raw development | [GitHub](https://github.com/lunola/lightcraft) | [Website](https://lunola.com/apps/lightcraft) |
| <img src="https://raw.githubusercontent.com/lunola/pdfcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.pdfcraft.png" alt="" width="32" height="32"> | **PdfCraft** | Reading, organizing and protecting PDFs | [GitHub](https://github.com/lunola/pdfcraft) | [Website](https://lunola.com/apps/pdfcraft) |
| <img src="https://raw.githubusercontent.com/lunola/effectcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.effectcraft.png" alt="" width="32" height="32"> | **EffectCraft** | Motion graphics and visual effects | [GitHub](https://github.com/lunola/effectcraft) | [Website](https://lunola.com/apps/effectcraft) |
| <img src="https://raw.githubusercontent.com/lunola/designcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.designcraft.png" alt="" width="32" height="32"> | **DesignCraft** | Page layout and publishing | [GitHub](https://github.com/lunola/designcraft) | [Website](https://lunola.com/apps/designcraft) |
| <img src="https://raw.githubusercontent.com/lunola/deckcraft/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.deckcraft.png" alt="" width="32" height="32"> | **Lunola Veyra** | **Presentations and slide shows · you are here** | [GitHub](https://github.com/lunola/deckcraft) | [Website](https://lunola.com/apps/deckcraft) |

And [**Lunola**](https://lunola.com/) itself, our AI image and video studio for artists who want real control.

<br>

<p align="center">
  <a href="https://discord.gg/lunola"><img alt="Join the Lunola community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<h3 align="center">Come make things with us</h3>

<p align="center">
  Our Discord is where artists of every kind hang out: people who paint, shoot, draw, cut film,
  set type, and people still figuring out what they like to make. Share what you're working on,
  ask for help, tell us what's broken, or tell us what you wish these tools could do.
  Whatever your medium and however long you've been at it, you're welcome here.
</p>

<p align="center">
  <a href="https://discord.gg/lunola"><b>discord.gg/lunola</b></a> ·
  <a href="https://lunola.com/">lunola.com</a> ·
  <a href="https://lunola.com/apps">The Crafting Apps</a> ·
  <a href="https://lunola.com/apps/lunola-veyra">Lunola Veyra</a>
</p>

## License and credits

Lunola Veyra is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 Lunola Team and the Lunola Veyra contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licenses; each one is listed
with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md).

Lunola Veyra's themes, layouts, shape geometry, icons and sample decks are original work, drawn or
generated in code; no Microsoft artwork, fonts, themes or templates are used.

The Lunola name, wordmark and logos in [`docs/brand/`](docs/brand/) are trademarks of the
Lunola Team and are not covered by this license. They may be used only unmodified, and only as
part of this repository and Lunola Veyra, under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).
Forks and modified versions must remove them.

<sub>Microsoft and PowerPoint are trademarks of the Microsoft group of companies. Adobe, Photoshop, Illustrator, Premiere Pro, Lightroom, Acrobat, After Effects and InDesign are trademarks or registered trademarks of Adobe Inc. in the United States and/or other countries. Lunola Veyra is an independent, open-source project and is not affiliated with, sponsored by or endorsed by Microsoft Corporation or Adobe Inc.; these names are used only to describe the workflows it is compatible with.</sub>

<p align="center">
  <a href="https://lunola.com/"><img alt="Lunola" src="docs/brand/artcraft-mark.svg" width="28"></a><br>
  <sub>Made by the <a href="https://lunola.com/">Lunola</a> team and community.</sub>
</p>
