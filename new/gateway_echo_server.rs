// SPDX-License-Identifier: GPL-2.0

//! Rust echo server sample.

use kernel::{
    kasync::executor::{workqueue::Executor as WqExecutor, AutoStopHandle, Executor},
    kasync::net::{TcpListener, TcpStream},
    net::{self, Ipv4Addr, SocketAddr, SocketAddrV4},
    prelude::*,
    spawn_task,
    sync::{Arc, ArcBorrow},
    cegwtcp::*, 
};


use kernel::net::*;
use kernel::error::*;
use core::*;
use kernel::bindings ;


async fn echo_server(stream: TcpStream) -> Result {
    let mut buf = [0u8; 52];
    loop {
        let n = stream.read(&mut buf).await?;
       
        if n == 0 {
            return Ok(());
        }

        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8000)) ;
        let stream1 = connect(&remote_addr)?;
        // Example number to send 
        let buf1=buf.try_to_vec()?; 
        send_data(&stream1, buf1)? ; 

        stream.write_all(&buf[..n]).await?;
        



    }
}

async fn accept_loop(listener: TcpListener, executor: Arc<impl Executor>) {
    loop {
        if let Ok(stream) = listener.accept().await {
            let _ = spawn_task!(executor.as_arc_borrow(), echo_server(stream));
        }
    }
}

fn start_listener(ex: ArcBorrow<'_, impl Executor + Send + Sync + 'static>) -> Result {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8080));
    let listener = TcpListener::try_new(net::init_ns(), &addr)?;
    spawn_task!(ex, accept_loop(listener, ex.into()))?;
    Ok(())
}


pub fn connect(address: &SocketAddr) -> Result<net::TcpStream> {
    let socket = Socket::new(AddressFamily::Inet, SockType::Stream, IpProtocol::Tcp)?;
    socket.connect(address, 0)?; 
    Ok(net::TcpStream {sock:unsafe{socket.as_inner()}})
}
/* 
pub fn send_number(stream: &TcpStream, number: u32) -> Result<usize> {
    let number_bytes = number.to_le_bytes();
    stream.write(&number_bytes, true)
    
}*/

pub fn send_data(stream: &net::TcpStream, data: Vec<u8>) -> Result<usize> {
    // Ensure the data vector has exactly 52 elements
    
    
    let mut buffer = [0u8; 52];
    for (i, &item) in data.iter().enumerate() {
        if i >= 52 {
            break; // Prevent index out of bounds
        }
        buffer[i] = item;
    }
    // Write the data vector to the stream
    stream.write(&buffer,true)

    // Return the number of bytes written
   
}


struct RustEchoServer {
    _handle: AutoStopHandle<dyn Executor>,
}

impl kernel::Module for RustEchoServer {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let handle = WqExecutor::try_new(kernel::workqueue::system())?;
        pr_info!("Listening") ; 
        start_listener(handle.executor())?;
        Ok(Self {
            _handle: handle.into(),
        })
    }
}

module! {
    type: RustEchoServer,
    name: "rust_echo_server",
    author: "Rust for Linux Contributors",
    description: "Rust tcp echo sample",
    license: "GPL v2",
}
