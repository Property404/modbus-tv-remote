use anyhow::Result;
use mouse_keyboard_input::key_codes;
use raw_tty::IntoRawMode;
use std::io::Read;
use std::net::SocketAddr;
use terminal_keycode::{Decoder, KeyCode};
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

fn translate(key: KeyCode) -> Option<u16> {
    match key {
        KeyCode::ArrowUp => Some(key_codes::KEY_UP),
        KeyCode::ArrowDown => Some(key_codes::KEY_DOWN),
        KeyCode::ArrowLeft => Some(key_codes::KEY_LEFT),
        KeyCode::ArrowRight => Some(key_codes::KEY_RIGHT),
        KeyCode::Enter => Some(key_codes::KEY_ENTER),
        KeyCode::Space => Some(key_codes::KEY_SPACE),
        KeyCode::Backspace => Some(key_codes::KEY_BACKSPACE),
        KeyCode::Escape => Some(key_codes::KEY_ESC),
        KeyCode::Char('x') => Some(key_codes::KEY_X),
        KeyCode::Char('c') => Some(key_codes::KEY_C),
        _ => None,
    }
}

async fn run_client_inner(client: &mut ModbusClient) -> Result<()> {
    let mut stdin = std::io::stdin().into_raw_mode().unwrap();
    let mut buf = vec![0];
    let mut decoder = Decoder::new();
    loop {
        stdin.read_exact(&mut buf).unwrap();
        for keycode in decoder.write(buf[0]) {
            print![
                "code={:?} bytes={:?} printable={:?}\r\n",
                keycode,
                keycode.bytes(),
                keycode.printable()
            ];
            if keycode == KeyCode::CtrlC {
                return Ok(());
            }
            if let Some(addr) = translate(keycode) {
                client.send_command(addr).await?;
            }
        }
    }
}

pub async fn run_client(socket_addr: SocketAddr) -> Result<()> {
    let mut client = ModbusClient::new(socket_addr).await?;
    run_client_inner(&mut client).await?;
    client.cleanup().await?;
    Ok(())
}
