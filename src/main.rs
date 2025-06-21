use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, sleep},
    time::{Duration, Instant},
};

use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code, HotKey},
};

use mouse_rs::{Mouse, types::keys::Keys};

use clap::{Parser, command};

#[derive(clap::ValueEnum, Clone, Debug)]
enum MouseButton {
    Left,
    Right,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_flag = true, disable_version_flag = true)]
struct Cli {
    /// show help information
    #[arg(long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// show version information
    #[arg(long = "version", action = clap::ArgAction::Version)]
    version: Option<bool>,

    /// click intervals in miliseconds
    #[arg(long, default_value_t = 50)]
    interval: u64,

    /// which mouse button to click
    #[arg(long, default_value = "left", value_enum)]
    button: MouseButton,

    /// how many times to click per interval 
    #[arg(long, default_value_t = 1)]
    repeat: i32,

    /// keybind to toggle clicker
    #[arg(long, default_value = "Alt T")]
    toggle: String,

    /// keybind to quit program
    #[arg(long, default_value = "Alt Q")]
    quit: String,
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
        "Q" => KeyQ,
        "T" => KeyT,
        "F1" => F1,
        "F2" => F2,
        "F3" => F3,
        "ESC" | "ESCAPE" => Escape,
        "SPACE" => Space,
        _ => return None,
    }) // only these codes for now
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

    if cli.help.is_some() || cli.version.is_some() {
        return;
    }

    let toggle = parse_hotkey(&cli.toggle).unwrap_or_else(|| {
        eprintln!("Invalid toggle hotkey: {}", cli.toggle);
        std::process::exit(1);
    });

    let quit = parse_hotkey(&cli.quit).unwrap_or_else(|| {
        eprintln!("Invalid quit hotkey: {}", cli.quit);
        std::process::exit(1);
    });

    run_clicker(cli.interval, cli.button, cli.repeat, toggle, quit);
}
