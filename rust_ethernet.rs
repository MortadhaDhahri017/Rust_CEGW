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

        let n = stream.read(&mut buf).await?;

        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8080)) ;

        coarse_sleep(Duration::from_millis(500)) ; 
        pr_info!("Connection Established with Netlink Kernel Server ... ") ;
        coarse_sleep(Duration::from_millis(500)) ; 
        pr_info!("Displaying Incoming Ethernet Frame ...") ;

        coarse_sleep(Duration::from_millis(500)) ; 
        let eth = EthFrame::deserialize_ethernet(&buf).unwrap();
        pr_info!("EthFrame:");
        pr_info!("  dst: {:02x?}", eth.dst);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("  src: {:02x?}", eth.src);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("  ethertype: {:04x}", eth.ethertype.0);
        coarse_sleep(Duration::from_millis(100)); 
        
        pr_info!("  Ipv4Frame:");
        pr_info!("    version: {:02x}", eth.data_ipv4.header.version);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    len: {:02x}", eth.data_ipv4.header.len);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    ToS: {:02x}", eth.data_ipv4.header.ToS);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    total_len: {:04x}", eth.data_ipv4.header.total_len);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    id: {:04x}", eth.data_ipv4.header.id);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    flags: {:02x}", eth.data_ipv4.header.flags);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    frag_offset: {:02x}", eth.data_ipv4.header.frag_offset);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    ttl: {:02x}", eth.data_ipv4.header.ttl);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    protocol: {:02x}", eth.data_ipv4.header.protocol);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    checksum: {:04x}", eth.data_ipv4.header.checksum);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    src: {:02x?}", eth.data_ipv4.header.src);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    dst: {:02x?}", eth.data_ipv4.header.dst);
        coarse_sleep(Duration::from_millis(100)); 
        
        pr_info!("    TcpHeader:");
        pr_info!("      src_port: {:04x}", eth.data_ipv4.data_tcp.header.src_port);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("      dst_port: {:04x}", eth.data_ipv4.data_tcp.header.dst_port);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("      seq: {:08x}", eth.data_ipv4.data_tcp.header.seq);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("      ack: {:08x}", eth.data_ipv4.data_tcp.header.ack);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("      offset: {:02x}", eth.data_ipv4.data_tcp.header.offset);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("      reserved: {:02x}", eth.data_ipv4.data_tcp.header.reserved);    
        coarse_sleep(Duration::from_millis(100));     
        pr_info!("      flags: {:02x}", eth.data_ipv4.data_tcp.header.flags);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("      window: {:04x}", eth.data_ipv4.data_tcp.header.window);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("      checksum: {:04x}", eth.data_ipv4.data_tcp.header.checksum);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("      urgent_ptr: {:04x}", eth.data_ipv4.data_tcp.header.urgent_ptr);
        coarse_sleep(Duration::from_millis(100)); 
        pr_info!("    data: {:02x?}", eth.data_ipv4.data_tcp.data);
        coarse_sleep(Duration::from_millis(100)); 
        
        pr_info!(" Preparing to push Ethernet Frame to Gateway kernel module .") ;
        
        coarse_sleep(Duration::from_millis(500)) ;
        let stream1 = connect(&remote_addr)?;
        pr_info!("Establishing Connexion on Port 8080 .");
        send_data(&stream1,array_to_vec(&buf))?;
        Ok(())
}

async fn accept_loop(listener: TcpListener, executor: Arc<impl Executor>) {
        
        pr_info!("Listening on Port 8050.");
        coarse_sleep(Duration::from_millis(200)); 
        if let Ok(stream) = listener.accept().await {
            pr_info!("Connexion Established with Netlink UserSpace Module.");
            coarse_sleep(Duration::from_millis(200));  
            let _ = spawn_task!(executor.as_arc_borrow(), echo_server(stream));
        }
    
}

fn start_listener(ex: ArcBorrow<'_, impl Executor + Send + Sync + 'static>) -> Result {
    let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY,8050)) ;
    let listener = TcpListener::try_new(net::init_ns(), &remote_addr)?;
    spawn_task!(ex, accept_loop(listener, ex.into()))?;
    Ok(())
}

struct RustEchoServer {
    _handle: AutoStopHandle<dyn Executor>,
}

impl kernel::Module for RustEchoServer {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Ethernet module Init Function Has been called .") ; 
        coarse_sleep(Duration::from_millis(200)); 
        let handle = WqExecutor::try_new(kernel::workqueue::system())?;
        start_listener(handle.executor())?;

        Ok(Self {
            _handle: handle.into(),
        })
    }
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



module! {
    type: RustEchoServer,
    name: "RUST_ETHERNET_MODULE",
    author: "Rust for Linux Contributors",
    description: "Rust tcp echo sample",
    license: "GPL v2",
}

fn array_to_vec(arr: &[u8; 64]) -> Vec<u8> {
    let mut vec = Vec::new();
    for &item in arr.iter() {
        vec.try_push(item);
    }
    vec
}

 
/* 
use kernel::{
    file::{flags, File, Operations},
    io_buffer::{IoBufferReader, IoBufferWriter},
    miscdev,
    prelude::*,
    sync::{smutex::Mutex, Arc, ArcBorrow},
    Module,
};
use kernel::str::CString;
use kernel::delay::coarse_sleep ; 
use core::time::Duration;

/* 
use kernel::tcp::{EthFrame , canfd_ethpayload , TCP_Frame , Ipv4Frame , Ipv4Header , TcpHeader ,EtherType} ; 
use kernel::tcp::{serialize_canfd_ethpayload};
use alloc::vec::*; 
*/

module! {
    type: RustClientTest,
    name: "RUST_ETHERNET",
    license: "GPL",
    params: {
        devices: u32 {
            default: 1,
            permissions: 0o644,
            description: "Number of virtual devices",
        },
    },
}

use kernel::cegwtcp::*; 
use kernel::net::*;
use kernel::error::*;
use core::*;
use kernel::bindings ;


pub fn connect(address: &SocketAddr) -> Result<TcpStream> {
    let socket = Socket::new(AddressFamily::Inet, SockType::Stream, IpProtocol::Tcp)?;
    socket.connect(address, 0)?; 
    pr_info!("RUST_NETLINK CONNECT FUNCTION IS BEING CALLED "); 
    coarse_sleep(Duration::from_millis(500)) ;
    Ok(TcpStream {sock:unsafe{socket.as_inner()}})
}
/* 
pub fn send_number(stream: &TcpStream, number: u32) -> Result<usize> {
    let number_bytes = number.to_le_bytes();
    stream.write(&number_bytes, true)
    
}*/

pub fn send_data(stream: &TcpStream, data: Vec<u8>) -> Result<usize> {
    // Ensure the data vector has exactly 52 elements
    
    
    let mut buffer = [0u8; 64];
    for (i, &item) in data.iter().enumerate() {
        if i >= 64 {
            break; // Prevent index out of bounds
        }
        buffer[i] = item;
    }

    pr_info!("RUST_CLIENT : SEND_DATA FUNCTION IS BEING CALLED") ; 
    coarse_sleep(Duration::from_millis(500)) ;
    // Write the data vector to the stream
    stream.write(&buffer,true)

    // Return the number of bytes written
   
}

pub struct RustClientTest {
    stream: TcpStream,
}

impl kernel::Module for RustClientTest {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8080)) ;
        pr_info!("---INFO--- CREATED SOCKET SUCCCESSFULLY !") ; 
        coarse_sleep(Duration::from_millis(500)) ;
        
       let stream = connect(&remote_addr)?;
        pr_info!("---INFO--- CONNECTED TO PORT 8080 SUCCESSFULLY , BROADCAST MODE ENABLED ") ; 
        coarse_sleep(Duration::from_millis(500)) ;
        // Example number to send 
        
        let raw_data:[u8;64]= [
            // Destination MAC address (replace with real values)
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            // Source MAC address (replace with real values)
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
            // EtherType (IPv4)
            0x08, 0x00,
            // IP header data (replace with actual values) - Assuming minimal header size (20 bytes)
            0x45, 0x00, 0x00, 0x2C, // Version, header length, ToS, total length
            0xAB, 0xCD, // Identification
            0x00, 0x00, // Flags, fragment offset
            0x40, // TTL
            0x06, // Protocol (TCP)
            0x00, 0x00,0x00 ,0x00, // Checksum (placeholder, needs calculation)
            0x01, 0x02, 0x03,  0x04, // Source IP address
            0x05, 0x06, 0x07,  0x08, // Destination IP address
            // TCP header data (replace with actual values) - Assuming minimal header size (20 bytes)
            0x20, 0x08, // Source port
            0x01, 0xBB, // Destination port
            0x49, 0x79, 0xBE, 0x6F, // Sequence number
            0x00, 0x00, 0x00, 0x00, // Acknowledgment number (0 for SYN)
            0x05, 0x00, // Offset, reserved, flags (SYN flag set)
            0xFF, 0xFF, 0x00, 0x00, // Window size
            0x00, 0x00, // Checksum (placeholder, needs calculation)
            0x00, 0x00, 0x00, 0x00, // Urgent pointer
            // TCP data (first 4 bytes, replace with actual data)
            0xDE, 0xAD, 0xBE, 0xEF,
        ]; 
                 // TCP data (4 bytes)
        

        let eth=EthFrame::deserialize_ethernet(&raw_data).unwrap() ; 
        pr_info!("EthFrame:");
        pr_info!("  dst: {:02x?}", eth.dst);
        pr_info!("  src: {:02x?}", eth.src);
        pr_info!("  ethertype: {:04x}", eth.ethertype.0);
        
        pr_info!("  Ipv4Frame:");
        pr_info!("    version: {}", eth.data_ipv4.header.version);
        pr_info!("    len: {}", eth.data_ipv4.header.len);
        pr_info!("    ToS: {}", eth.data_ipv4.header.ToS);
        pr_info!("    total_len: {}", eth.data_ipv4.header.total_len);
        pr_info!("    id: {}", eth.data_ipv4.header.id);
        pr_info!("    flags: {}", eth.data_ipv4.header.flags);
        pr_info!("    frag_offset: {}", eth.data_ipv4.header.frag_offset);
        pr_info!("    ttl: {}", eth.data_ipv4.header.ttl);
        pr_info!("    protocol: {}", eth.data_ipv4.header.protocol);
        pr_info!("    checksum: {}", eth.data_ipv4.header.checksum);
        pr_info!("    src: {:02x?}", eth.data_ipv4.header.src);
        pr_info!("    dst: {:02x?}", eth.data_ipv4.header.dst);
        
        pr_info!("    TcpHeader:");
        pr_info!("      src_port: {}", eth.data_ipv4.data_tcp.header.src_port);
        pr_info!("      dst_port: {}", eth.data_ipv4.data_tcp.header.dst_port);
        pr_info!("      seq: {}", eth.data_ipv4.data_tcp.header.seq);
        pr_info!("      ack: {}", eth.data_ipv4.data_tcp.header.ack);
        pr_info!("      offset: {}", eth.data_ipv4.data_tcp.header.offset);
        pr_info!("      reserved: {}", eth.data_ipv4.data_tcp.header.reserved);
        pr_info!("      flags: {}", eth.data_ipv4.data_tcp.header.flags);
        pr_info!("      window: {}", eth.data_ipv4.data_tcp.header.window);
        pr_info!("      checksum: {}", eth.data_ipv4.data_tcp.header.checksum);
        pr_info!("      urgent_ptr: {}", eth.data_ipv4.data_tcp.header.urgent_ptr);
        
        pr_info!("    data: {:02x?}", eth.data_ipv4.data_tcp.data);


       
        pr_info!("---INFO--- PUSHING DATA (ETHERNET FRAME) TO THE GATEWAY ! ") ;
        coarse_sleep(Duration::from_millis(500)) ;

    
        send_data(&stream, array_to_vec(&raw_data))? ; 

        Ok(Self { stream })
    }
}
fn array_to_vec(arr: &[u8; 64]) -> Vec<u8> {
    let mut vec = Vec::new();
    for &item in arr.iter() {
        vec.try_push(item);
    }
    vec
}
*/