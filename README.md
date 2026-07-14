<p align="center">
<pre>
██████╗       ██████╗  █████╗ ███████╗████████╗███████╗██████╗ ███╗   ███╗
██╔══██╗      ██╔══██╗██╔══██╗██╔════╝╚══██╔══╝██╔════╝██╔══██╗████╗ ████║
██████╔╝█████╗██████╔╝███████║███████╗   ██║   █████╗  ██████╔╝██╔████╔██║
██╔══██╗╚════╝██╔══██╗██╔══██║╚════██║   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║
██████╔╝      ██║  ██║██║  ██║███████║   ██║   ███████╗██║  ██║██║ ╚═╝ ██║
╚═════╝       ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝
</pre>
</p>

<p align="center">
  <strong>🎨 Color configurator for Cava audio visualizer</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue" alt="Version">
  <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
  <img src="https://img.shields.io/badge/language-Rust-orange" alt="Rust">
</p>

---

## Features

- **Solid color mode** — single color for all bars
- **Gradient mode** — 2-5 colors, bottom to top
- **Live preview** — see colors before applying
- **Bilingual** — Spanish / English interface
- **Auto-reload** — sends `SIGUSR2` to Cava after config change
- **Minimal** — no dependencies, just Cava

## Install

### From source

```bash
git clone https://github.com/Ricardo-Castro-Gastelum/BlooCava.git
cd BlooCava
cargo build --release
sudo cp target/release/bloocava /usr/local/bin/
```

### With Cargo

```bash
cargo install --git https://github.com/Ricardo-Castro-Gastelum/BlooCava.git
```

## Usage

```bash
# Interactive mode (default)
bloocava

# Help
bloocava -h

# Version
bloocava -v
```

## How it works

1. Run `bloocava`
2. Select language (ES/EN)
3. Choose **Solid** or **Gradient** mode
4. Enter hex colors (e.g. `#ff0000`)
5. See live preview
6. Press **Enter** to apply

BlooCava writes to `~/.config/cava/config` and sends `SIGUSR2` to reload Cava automatically.

## Controls

| Key | Action |
|-----|--------|
| `Up/Down` | Navigate menu |
| `Enter` | Select / Apply |
| `Esc` | Back / Exit |
| `Tab` | Next color (gradient) |
| `1` / `2` | Switch language |

## Requirements

- [Cava](https://github.com/karlstav/cava) installed and configured
- Terminal with color support

## License

MIT License - see [LICENSE](LICENSE) for details.
