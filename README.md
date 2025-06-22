# Auto Clicker

A simple, fast, and customizable auto-clicker built in Rust. Cross-platform support with hotkey toggling and configurable intervals.

---

## ✨ Features

- ✅ Global hotkey support (toggle & quit)
- ⏱️ Customizable click intervals
- 🖱️ Left or right mouse button selection
- 💻 Supports Windows, Linux, and macOS*

---

## 📦 Installation

### ✅ Linux / MacOS (using `install.sh`)

```bash
bash <(curl -sSL https://raw.githubusercontent.com/Lunarr199/auto-clicker/master/install/install.sh)
```

Or manually:

```bash
wget https://github.com/Lunarr199/auto-clicker/releases/latest/download/auto-clicker-x86_64-unknown-linux-gnu.zip
unzip auto-clicker*.zip
chmod +x auto-clicker
sudo mv auto-clicker /usr/local/bin/
```

Verify:
```bash
auto-clicker --help
```

### 🪟 Windows (using `Powershell`)

```powershell
irm https://raw.githubusercontent.com/Lunarr199/auto-clicker/master/install/install.ps1 | iex
```

Or manually:

1. Go to [Releases](https://github.com/Lunarr199/auto-clicker/releases)
2. Download the latest `.zip` for Windows
3. Extract it and run `auto-clicker.exe`

### 🛠 Developers (Install via `Cargo`)

```bash
cargo install --git https://github.com/Lunarr199/auto-clicker
```

## ⚙️ Usage

```bash
auto-clicker <ARGUMENTS>
```

### 🔑 Hotkeys (default):
* `Alt + T` → Toggle clicker
* `Alt + Q` → Quit

### 🔧 Arguments:

| Flag               | Description                         | Example                 |
|--------------------|-------------------------------------|-------------------------|
| `--interval`       | Time between clicks (ms)            | `--interval 50`         |
| `--button`         | Mouse button to click               | `--button right`        |
| `--toggle`         | Custom toggle keybind (optional)    | `--toggle "ctrl alt t"` |
| `--repeat`| Number of clicks per interval       | `--repeat 2`   |

For more information, please enter:

```bash
auto-clicker --help
```

### 📦 Example:

```bash
auto-clicker --interval 80 --button left --repeat 2
```

## 🧱 Building from Source

```bash
git clone https://github.com/Lunarr199/auto-clicker
cd auto-clicker
cargo build --release
```

Binaries will be in `target/release/`.

### 📋 Planned Features
* Auto updater
* Hold mouse support