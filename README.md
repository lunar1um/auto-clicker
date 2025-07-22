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
curl -sSL https://raw.githubusercontent.com/lunar1um/auto-clicker/master/install/install.sh | bash
```

Or manually:

```bash
wget -q https://github.com/lunar1um/auto-clicker/releases/latest/download/auto-clicker-x86_64-unknown-linux-gnu.zip
unzip -q auto-clicker-x86_64-unknown-linux-gnu.zip
mv auto-clicker-x86_64-unknown-linux-gnu auto-clicker
chmod +x auto-clicker
sudo mv auto-clicker /usr/local/bin/
rm auto-clicker-x86_64-unknown-linux-gnu.zip
```

Verify:
```bash
auto-clicker --help
```

### 🪟 Windows (using `Powershell`)

⚠️ Make sure you are running Powershell with Admin permission:

```powershell
powershell start-process powershell -verb runas
```

Install:

```powershell
Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/lunar1um/auto-clicker/master/install/install.ps1").Content
```

Or manually:

1. Go to [Releases](https://github.com/Lunarr199/auto-clicker/releases)
2. Download the latest `.zip` for Windows
3. Extract it and run `auto-clicker.exe`

### 🛠 Developers (Install via `Cargo`)

```bash
cargo install --git https://github.com/Lunarr199/auto-clicker
```

### 🔑 Hotkeys (default):
* `Alt + T` → Toggle clicker
* `Alt + Q` → Quit

## ⚙️ Usage

```bash
# Run the auto clicker program with the current config
auto-clicker run

# Change and save config
auto-clicker set <ARGUMENT1> <ARGUMENT2> <...>

# Display current config
auto-clicker show-config
```

### 🔧 Arguments:

| Flag               | Description                         | Example                 |
|--------------------|-------------------------------------|-------------------------|
| `--interval`       | Time between clicks (ms)            | `--interval 50`         |
| `--button`         | Mouse button to click               | `--button right`        |
| `--toggle`         | Custom toggle keybind (optional)    | `--toggle "ctrl alt t"` |
| `--quit`         | Custom quit keybind (optional)    | `--toggle "ctrl alt q"` |
| `--repeat`| Number of clicks per interval       | `--repeat 2`   |

For more information about arguments, please use the command:

```bash
auto-clicker set --help
```

#### ⚠️ Config will be stored in: 
* Linux: `~/.config/autoclicker`
* Windows: `C:\Users\<Username>\AppData\Roaming\YourName\autoclicker`
* macOS: `/Users/<Username>/Library/ApplicationSupport/com.YourName.autoclicker`

## 🧱 Building from Source

```bash
git clone https://github.com/lunar1um/auto-clicker
cd auto-clicker
cargo build --release
```

Binaries will be in `target/release/`.

### 📋 Planned Features
* Auto updater
* Hold mouse support
