use anyhow::anyhow;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::server::CommandFunctionError;
use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;

pub static VD: LazyLock<Mutex<VirtualDevice>> = LazyLock::new(|| {
    println!("Initializing");
    Mutex::new(VirtualDevice::default().unwrap())
});

pub fn key_commander_inner(val: u16) -> anyhow::Result<()> {
    let key = match val {
        0 => KEY_LEFT,
        1 => KEY_UP,
        2 => KEY_RIGHT,
        3 => KEY_DOWN,
        4 => KEY_SPACE,
        5 => KEY_ENTER,
        _ => {
            return Ok(());
        }
    };
    println!("{val} => {key}");

    let mut vd = VD.lock().expect("Poisoned lock");
    vd.click(key)
        .map_err(|err| anyhow!("Failed to click: {err}"))?;
    Ok(())
}

pub fn key_commander(val: u16) -> Result<(), CommandFunctionError> {
    if let Err(err) = key_commander_inner(val) {
        eprintln!("error: {err}");
        // Custom error because nothing else really fits
        return Err(CommandFunctionError::new(100));
    }
    Ok(())
}
