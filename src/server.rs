// Derived from work by slowtec GmbH:
//  Copyright (c) 2017-2025 slowtec GmbH <post@slowtec.de>
//  SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(unused_imports)]
use anyhow::Result;
use std::{
    collections::HashMap,
    future,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tokio::net::TcpListener;

use tokio_modbus::{
    prelude::*,
    server::tcp::{Server, accept_tcp_connection},
};

pub struct CommandServer<F> {
    func: F,
}

impl<F: Fn(u16) -> Result<(), ExceptionCode>> tokio_modbus::server::Service for CommandServer<F> {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ready(self.call_sync(req))
    }
}

impl<F: Fn(u16) -> Result<(), ExceptionCode>> CommandServer<F> {
    fn call_sync(&self, req: Request<'static>) -> Result<Response, ExceptionCode> {
        match req {
            Request::ReadCoils(_addr, cnt) => {
                //  Return all zeros
                Ok(Response::ReadCoils(vec![false; cnt as usize]))
            }
            Request::WriteMultipleCoils(mut addr, values) => {
                for _ in 0..values.len() {
                    (self.func)(addr)?;
                    addr += 1;
                }
                Ok(Response::WriteMultipleCoils(addr, values.len() as u16))
            }
            Request::WriteSingleCoil(addr, value) => {
                (self.func)(addr)?;
                Ok(Response::WriteSingleCoil(addr, value))
            }
            _ => {
                println!(
                    "SERVER: Exception::IllegalFunction - Unimplemented function code in request: {req:?}"
                );
                Err(ExceptionCode::IllegalFunction)
            }
        }
    }
}
/*

/// Helper function implementing reading registers from a HashMap.
fn register_read(
    registers: &HashMap<u16, u16>,
    addr: u16,
    cnt: u16,
) -> Result<Vec<u16>, ExceptionCode> {
    let mut response_values = vec![0; cnt.into()];
    for i in 0..cnt {
        let reg_addr = addr + i;
        if let Some(r) = registers.get(&reg_addr) {
            response_values[i as usize] = *r;
        } else {
            println!("SERVER: Exception::IllegalDataAddress");
            return Err(ExceptionCode::IllegalDataAddress);
        }
    }

    Ok(response_values)
}

/// Write a holding register. Used by both the write single register
/// and write multiple registers requests.
fn register_write(
    registers: &mut HashMap<u16, u16>,
    addr: u16,
    values: &[u16],
) -> Result<(), ExceptionCode> {
    for (i, value) in values.iter().enumerate() {
        let reg_addr = addr + i as u16;
        if let Some(r) = registers.get_mut(&reg_addr) {
            *r = *value;
        } else {
            println!("SERVER: Exception::IllegalDataAddress");
            return Err(ExceptionCode::IllegalDataAddress);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = "127.0.0.1:5502".parse().unwrap();

    server_context(socket_addr).await?;

    Ok(())
}

async fn server_context(socket_addr: SocketAddr) -> anyhow::Result<()> {
    println!("Starting up server on {socket_addr}");
    let listener = TcpListener::bind(socket_addr).await?;
    let server = Server::new(listener);
    let new_service = |_socket_addr| Ok(Some(CommandServer::new()));
    let on_connected = |stream, socket_addr| async move {
        accept_tcp_connection(stream, socket_addr, new_service)
    };
    let on_process_error = |err| {
        eprintln!("{err}");
    };
    server.serve(&on_connected, on_process_error).await?;
    Ok(())
}
*/
