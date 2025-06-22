use std::{
    fs, process,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, sleep},
    time::{Duration, Instant},
};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code, HotKey},
};
use mouse_rs::{Mouse, types::keys::Keys};
use clap::{Parser, command};
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
struct AppConfig {
    interval: Option<u64>,
    button: Option<MouseButton>,
    repeat: Option<i32>,
    toggle: Option<String>,
    quit: Option<String>,
}

#[derive(Debug, Serialize)]
struct MergedConfig {
    interval: u64,
    button: MouseButton,
    repeat: i32,
    toggle: String,
    quit: String,
}

#[derive(clap::ValueEnum, Clone, Debug, Serialize, Deserialize, PartialEq)]
enum MouseButton {
    Left,
    Right,
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::Left
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
enum Cli {
    /// Run the auto clicker using the saved config
    Run {},

    /// Update configuration settings (without running)
    Set(ConfigFlags),

    /// Display current configs
    ShowConfig {},
}

#[derive(Debug, Default, Parser)]
struct ConfigFlags {
    /// Click intervals in miliseconds
    #[arg(long)]
    interval: Option<u64>,

    /// Which mouse button to click
    #[arg(long, value_enum)]
    button: Option<MouseButton>,

    /// How many times to click per interval
    #[arg(long)]
    repeat: Option<i32>,

    /// Keybind to toggle clicker
    #[arg(long)]
    toggle: Option<String>,

    /// Keybind to quit program
    #[arg(long)]
    quit: Option<String>,
}

fn load_config() -> Option<AppConfig> {
    let path = ProjectDirs::from("com", "YourName", "autoclicker")
        .map(|d| d.config_dir().join("config.toml"))?;

    if !path.exists() {
        return None;
    }

    let contents = fs::read_to_string(path).ok()?;
    toml::from_str(&contents).ok()
}

fn parse_hotkey(input: &str) -> Option<HotKey> {
    use global_hotkey::hotkey::{HotKey, Modifiers};

    let mut modifiers = Modifiers::empty();
    let mut code = None;

    for part in input.split_whitespace() {
        match part.to_ascii_lowercase().as_str() {
            "ctrl" => modifiers |= Modifiers::CONTROL,
            "alt" => modifiers |= Modifiers::ALT,
            "shift" => modifiers |= Modifiers::SHIFT,
            "meta" | "cmd" | "super" => modifiers |= Modifiers::META,
            other => {
                code = parse_code(other);
            }
        }
    }

    code.map(|c| HotKey::new(Some(modifiers), c))
}

fn parse_code(key: &str) -> Option<Code> {
    use Code::*;

    Some(match key.to_ascii_uppercase().as_str() {
        "A" => KeyA,
        "B" => KeyB,
        "C" => KeyC,
        "D" => KeyD,
        "E" => KeyE,
        "F" => KeyF,
        "G" => KeyG,
        "H" => KeyH,
        "I" => KeyI,
        "K" => KeyK,
        "J" => KeyJ,
        "L" => KeyL,
        "M" => KeyM,
        "N" => KeyN,
        "O" => KeyO,
        "P" => KeyP,
        "Q" => KeyQ,
        "R" => KeyR,
        "S" => KeyS,
        "T" => KeyT,
        "U" => KeyU,
        "V" => KeyV,
        "X" => KeyX,
        "Y" => KeyY,
        "Z" => KeyZ,
        "F1" => F1,
        "F2" => F2,
        "F3" => F3,
        "F4" => F4,
        "F5" => F5,
        "F6" => F6,
        "F7" => F7,
        "ESC" | "ESCAPE" => Escape,
        "SPACE" => Space,
        _ => return None,
    })
}

fn run_clicker(
    interval: u64,
    button: MouseButton,
    repeat: i32,
    toggle_key: HotKey,
    quit_key: HotKey,
) {
    // shared active flag
    let active = Arc::new(AtomicBool::new(false));
    let active_clone = Arc::clone(&active);

    // start clicking thread
    thread::spawn(move || {
        let mouse = Mouse::new();
        let button = match button {
            MouseButton::Left => Keys::LEFT,
            MouseButton::Right => Keys::RIGHT,
        };

        loop {
            if active_clone.load(Ordering::Relaxed) {
                for _ in 0..=repeat {
                    mouse.press(&button).expect("can't press");
                    mouse.release(&button).expect("can't release");
                }
                sleep(Duration::from_millis(interval)); // click interval
            } else {
                sleep(Duration::from_millis(100));
            }
        }
    });

    let mut last_toggle = Instant::now();
    let manager = GlobalHotKeyManager::new().unwrap();

    let hotkeys = [toggle_key, quit_key];

    manager
        .register_all(&hotkeys)
        .expect("unable to register hotkeys");

    println!("starting auto clicker");

    let toggle_id = toggle_key.id();
    let quit_id = quit_key.id();

    // hotkeys handling
    loop {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            let now = Instant::now();
            match event.id() {
                e if e == toggle_id => {
                    if now.duration_since(last_toggle) >= Duration::from_millis(300) {
                        last_toggle = now;
                        let new_state = !active.load(Ordering::Relaxed);
                        active.store(new_state, Ordering::Relaxed);
                        println!("auto clicker state: {}", new_state);
                    }
                }
                e if e == quit_id => {
                    println!("quitting...");
                    break;
                }
                _ => {}
            }
        }
        sleep(Duration::from_millis(200)); // expect slight delay :3
    }
}

fn main() {
    let cli = Cli::parse();
    let path = ProjectDirs::from("com", "YourName", "autoclicker")
        .expect("Cannot determine config directory")
        .config_dir()
        .join("config.toml");

    match cli {
        Cli::Run {} => {
            let file_config: AppConfig = {
                let contents = fs::read_to_string(&path).expect("Config file not found.");
                toml::from_str(&contents).expect("Invalid config file")
            };

            let merged = MergedConfig {
                interval: file_config.interval.unwrap_or(50),
                button: file_config.button.unwrap_or(MouseButton::Left),
                repeat: file_config.repeat.unwrap_or(1),
                toggle: file_config.toggle.unwrap_or("Alt T".to_string()),
                quit: file_config.quit.unwrap_or("Alt Q".to_string()),
            };

            let toggle_key = parse_hotkey(&merged.toggle).unwrap_or_else(|| {
                eprintln!("Invalid toggle hotkey: {}", merged.toggle);
                process::exit(1);
            });

            let quit_key = parse_hotkey(&merged.quit).unwrap_or_else(|| {
                eprintln!("Invalid quit hotkey: {}", merged.quit);
                process::exit(1);
            });

            run_clicker(
                merged.interval,
                merged.button,
                merged.repeat,
                toggle_key,
                quit_key,
            );
        }

        Cli::Set(flags) => {
            let mut config = load_config().unwrap_or_default();

            if let Some(val) = flags.interval {config.interval = Some(val)};
            if let Some(val) = flags.button {config.button = Some(val)};
            if let Some(val) = flags.repeat {config.repeat = Some(val)};
            if let Some(val) = flags.toggle {config.toggle = Some(val)};
            if let Some(val) = flags.quit {config.quit = Some(val)};

            fs::create_dir_all(path.parent().unwrap()).unwrap();
            let toml = toml::to_string_pretty(&config).unwrap();
            fs::write(&path, toml).unwrap();
            println!("✅ Configuration updated.");
        }

        Cli::ShowConfig {} => {
            let file_config: AppConfig = {
                let contents = fs::read_to_string(&path).expect("Config file not found.");
                toml::from_str(&contents).expect("Invalid config file")
            };

            let merged = MergedConfig {
                interval: file_config.interval.unwrap_or(50),
                button: file_config.button.unwrap_or(MouseButton::Left),
                repeat: file_config.repeat.unwrap_or(1),
                toggle: file_config.toggle.unwrap_or("Alt T".to_string()),
                quit: file_config.quit.unwrap_or("Alt Q".to_string()),
            };

            let toml = toml::to_string_pretty(&merged).unwrap();
            println!("📄 Current Configuration:\n{toml}");
        }
    }
}
