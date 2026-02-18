use anyhow::Result;
use device_query::{DeviceQuery, DeviceState, Keycode};
use mouse_keyboard_input::key_codes;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use tokio_modbus::prelude::*;

struct ModbusClient {
    ctx: tokio_modbus::client::Context,
}

impl ModbusClient {
    async fn new(socket_addr: SocketAddr) -> Result<Self> {
        let ctx = tcp::connect(socket_addr).await?;
        Ok(ModbusClient { ctx })
    }

    // This should be in an async drop
    async fn send_command(&mut self, addr: u16) -> Result<()> {
        self.ctx.write_single_coil(addr, true).await??;
        Ok(())
    }

    // This should be in an async drop
    async fn cleanup(&mut self) -> Result<()> {
        println!("Disconnecting");
        self.ctx.disconnect().await?;
        Ok(())
    }
}

fn translate(key: Keycode) -> Option<u16> {
    match key {
        Keycode::Up => Some(key_codes::KEY_UP),
        Keycode::Down => Some(key_codes::KEY_DOWN),
        Keycode::Left => Some(key_codes::KEY_LEFT),
        Keycode::Right => Some(key_codes::KEY_RIGHT),
        Keycode::Enter => Some(key_codes::KEY_ENTER),
        Keycode::Space => Some(key_codes::KEY_SPACE),
        _ => None,
    }
}

async fn run_client_inner(client: &mut ModbusClient) -> Result<()> {
    let device_state = DeviceState::new();

    let mut states = HashMap::<Keycode, bool>::new();
    let mut previous = HashSet::<Keycode>::new();
    loop {
        let current = device_state.get_keys();
        let current: HashSet<Keycode> = HashSet::from_iter(current.into_iter());
        let pressed = current.symmetric_difference(&previous);

        for key in pressed {
            if !states.contains_key(key) {
                states.insert(*key, false);
            } else {
                // Flip
                states.insert(*key, !states[key]);
            }
            if !states[key] {
                println!("[{key:?}]");
                if let Some(addr) = translate(*key) {
                    client.send_command(addr).await?;
                }
            }
        }

        previous = current;
    }
}

pub async fn run_client(socket_addr: SocketAddr) -> Result<()> {
    let mut client = ModbusClient::new(socket_addr).await?;
    run_client_inner(&mut client).await?;
    client.cleanup().await?;
    Ok(())
}
