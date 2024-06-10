/* // SPDX-License-Identifier: GPL-2.0

//! Rust platform device driver sample.

use kernel::{module_platform_driver, of, platform, prelude::*};

module_platform_driver! {
    type: Driver,
    name: "rust_platform",
    license: "GPL",
}

struct Driver;
impl platform::Driver for Driver {
    kernel::define_of_id_table! {(), [
        (of::DeviceId::Compatible(b"rust,sample"), None),
    ]}

    fn probe(_dev: &mut platform::Device, _id_info: Option<&Self::IdInfo>) -> Result {
        Ok(())
    }
}
 */

// SPDX-License-Identifier: GPL-2.0

//! Rust echo server sample.

use kernel::{
    cegwtcp::*,
    kasync::{executor::{workqueue::Executor as WqExecutor, AutoStopHandle, Executor},
    net::{TcpListener, TcpStream}},
    net::{self, Ipv4Addr, SocketAddr, SocketAddrV4},
    prelude::*,
    spawn_task,
    sync::{Arc, ArcBorrow} 
};


use kernel::net::*;
use core::*;
use kernel::delay::coarse_sleep ; 
use core::time::Duration;


pub const DEFAULT : &str = "\x1b[0m";
pub const HIGHLIGHT : &str = "\x1b[1m";
pub const UNDERLINE : &str = "\x1b[4m";
pub const BLINK : &str = "\x1b[5m";
pub const RED : &str = "\x1b[31m";
pub const GREEN : &str = "\x1b[32m";
pub const YELLOW : &str = "\x1b[33m";
pub const BLUE : &str = "\x1b[34m";
pub const MAGENTA : &str = "\x1b[35m";
pub const CYAN : &str = "\x1b[36m";
pub const RESET : &str = "\x1b[0m";
pub const WHITE : &str = "\x1b[37m";


pub const BROWN : &str = "\x1b[38;2;165;42;42m"; // RGB color for brown

async fn echo_server(stream: TcpStream) -> Result {
        let mut buf = [0u8; 64];
        let n = stream.read(&mut buf).await?;

        pr_info!(" Receiving Ethernet Frame From Ethernet Module !") ; 
        coarse_sleep(Duration::from_millis(700)) ;

        pr_info!("Preparing to Deserialize the Ethernet Frame");
        let eth=EthFrame::deserialize_ethernet(&buf).unwrap() ; 
        pr_info!("buff {:?}",buf);
        
        pr_info!("Finished Deserializing Ethernet Frame ")  ; 
        coarse_sleep(Duration::from_millis(700)) ;
        pr_info!("PREPARING FOR A CONVERSION. ") ; 
        coarse_sleep(Duration::from_millis(700)) ;
        pr_info!("Updating the MAC Address ...") ; 
        coarse_sleep(Duration::from_millis(700)) ;
        pr_info!("Updating the IP Header ...") ;
        coarse_sleep(Duration::from_millis(700)) ;
        pr_info!("Updating the TCP Header") ; 
        coarse_sleep(Duration::from_millis(700)) ;
        pr_info!("Populating the Payload ...") ;
        let canFDeth=CanfdEthpayload::from_eth_frame(&eth)  ; 

        pr_info!("Done Converting the Ethernet Frame to a CANFD Frame ") ; 
        coarse_sleep(Duration::from_millis(700)) ;
        pr_info!("Preparing to Send Data to the Virtual CAN Device ") ; 
        coarse_sleep(Duration::from_millis(700)) ;
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8000)) ;
        pr_info!("Created Gateway Socket Successfully") ;
        coarse_sleep(Duration::from_millis(700)) ; 
        let stream1 = connect(&remote_addr)?;
        // Example number to send 
        let buf1=serialize_CanfdEthpayload(&canFDeth) ; 
        pr_info!("Done Serializing the CAN FD Frame , Sending to The Virtual CAN Device") ;
        coarse_sleep(Duration::from_millis(700)) ;  
        send_data(&stream1, buf1)? ; 
        pr_info!("SEND DATA Function Has been Called .") ; 
        coarse_sleep(Duration::from_millis(700)) ;
        pr_info!("Sending to The Virtual CAN Device ") ;
        coarse_sleep(Duration::from_millis(700)) ; 

        Ok(()) }
        //stream.write_all(&buf[..n]).await?;
    
    

         

async fn accept_loop(listener: TcpListener, executor: Arc<impl Executor>) {
    loop {
        pr_info!("Waiting for a Connexion on Port 8080");
        coarse_sleep(Duration::from_millis(500)); 
        if let Ok(stream) = listener.accept().await {
            pr_info!("Connexion Established On Port 8080");
            coarse_sleep(Duration::from_millis(500)); 
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
        {
        pr_info!("Gateway Init Function has been called , Ethernet to CAN Mode Enabled") ; 
        start_listener(handle.executor())?;
        Ok(Self {
            _handle: handle.into(),
        })
    }
}
}

module! {
    type: RustEchoServer,
    name: "RUST_ETHERNET_CAN_GATEWAY",
    author: "Rust for Linux Contributors",
    description: "Rust tcp echo sample",
    license: "GPL v2",
}