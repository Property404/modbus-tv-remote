use anyhow::anyhow;
use std::sync::LazyLock;
use std::sync::Mutex;

use crate::server::CommandFunctionError;
use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;

pub static VD: LazyLock<Mutex<VirtualDevice>> = LazyLock::new(|| {
    println!("Initializing");
    Mutex::new(VirtualDevice::default().unwrap())
});

pub fn key_commander_inner(val: u16) -> anyhow::Result<()> {
    if ![KEY_LEFT, KEY_UP, KEY_RIGHT, KEY_DOWN, KEY_SPACE, KEY_ENTER].contains(&val) {
        return Ok(());
    }
    println!("Received {val}");

    let mut vd = VD.lock().expect("Poisoned lock");
    vd.click(val)
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
