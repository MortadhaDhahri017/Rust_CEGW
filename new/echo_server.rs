// SPDX-License-Identifier: GPL-2.0

//! Rust echo server sample.

use kernel::{
    cegwtcp::{self, *}, kasync::{executor::{workqueue::Executor as WqExecutor, AutoStopHandle, Executor}, net::{TcpListener, TcpStream}}, net::{self, Ipv4Addr, SocketAddr, SocketAddrV4}, prelude::*, spawn_task, sync::{Arc, ArcBorrow} 
};


use kernel::net::*;
use kernel::error::*;
use core::*;
use kernel::bindings ;
use kernel::cegwtcp::*; 
use kernel::delay::coarse_sleep ; 
use core::time::Duration;


async fn echo_server(stream: TcpStream) -> Result {
    let mut buf = [0u8; 64];
    loop {
        let n = stream.read(&mut buf).await?;

        pr_info!("RECEIVING ETHERNET FRAME FROM THE NETLINK CLIENT ! ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
       
        if n == 0 {
            return Ok(());
        }


        pr_info!("CALLING FROM_ETH FUNCTION ") ;
        coarse_sleep(Duration::from_secs(1)) ; 

        let ethernet=EthFrame::deserialize_ethernet(&buf).unwrap() ; 

        pr_info!("DONE DESERIALIZING THE ETHERNET FRAME ")  ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("PREPARING FOR A CONVERSION ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        let canFDeth=canfd_ethpayload::from_eth_frame(&ethernet)  ; 

        pr_info!("DONE CONVERTING THE ETHERNET FRAME INTO A CANFD FRAME ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("PREPARING TO SEND TO THE VIRTUAL CAN DEVICE ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8000)) ;
        pr_info!("---INFO--- CREATED GATEWAY SOCKET SUCCCESSFULLY !") ;
        coarse_sleep(Duration::from_secs(1)) ; 
        let stream1 = connect(&remote_addr)?;
        // Example number to send 
        let buf1=serialize_canfd_ethpayload(&canFDeth) ; 
        pr_info!("DONE SERIALIZING THE CANFD FRAME , SENDING TO THE VIRTUAL CAN DEVICE") ;
        coarse_sleep(Duration::from_secs(1)) ;  
        send_data(&stream1, buf1)? ; 
        pr_info!("---INFO--- SEND DATA FUNCTION IS BEING CALLED !") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("---INFO--- SENDING TO THE VIRTUAL CAN DEVICE ! ") ;
        coarse_sleep(Duration::from_secs(1)) ; 

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
    
    
    let mut buffer = [0u8; 64];
    for (i, &item) in data.iter().enumerate() {
        if i >= 64 {
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
    name: "RUST_ETHERNET_CAN_GATEWAY",
    author: "Rust for Linux Contributors",
    description: "Rust tcp echo sample",
    license: "GPL v2",
}
