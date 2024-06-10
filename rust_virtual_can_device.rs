// SPDX-License-Identifier: GPL-2.0

//! Rust stack probing sample.


/* 
use kernel::prelude::*;

module! {
    type: RustStackProbing,
    name: "rust_stack_probing",
    author: "Rust for Linux Contributors",
    description: "Rust stack probing sample",
    license: "GPL",
}

struct RustStackProbing;

impl kernel::Module for RustStackProbing {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust stack probing sample (init)\n");

        // Including this large variable on the stack will trigger
        // stack probing on the supported archs.
        // This will verify that stack probing does not lead to
        // any errors if we need to link `__rust_probestack`.
        let x: [u64; 514] = core::hint::black_box([5; 514]);
        pr_info!("Large array has length: {}\n", x.len());

        Ok(RustStackProbing)
    }
}

impl Drop for RustStackProbing {
    fn drop(&mut self) {
        pr_info!("Rust stack probing sample (exit)\n");
    }
}*/

// SPDX-License-Identifier: GPL-2.0
/* 
use kernel::{
    file::{flags, File, Operations},
    io_buffer::{IoBufferReader, IoBufferWriter},
    miscdev,
    prelude::*,
    sync::{smutex::Mutex, Arc, ArcBorrow},
    Module,
    net::*,
    str::CString,
    delay::coarse_sleep,
};
use core::time::Duration;

module! {
    type: RustClientTest,
    name: "RUST_ETHERNET_CLIENT",
    license: "GPL",
}


pub fn connect(address: &SocketAddr) -> Result<TcpStream> {
    let socket = Socket::new(AddressFamily::Inet, SockType::Stream, IpProtocol::Tcp)?;
    socket.connect(address, 0)?; 
    pr_info!("RUST_NETLINK CONNECT FUNCTION IS BEING CALLED "); 
    coarse_sleep(Duration::from_secs(1)) ;
    Ok(TcpStream {sock:unsafe{socket.as_inner()}})
}

pub struct RustClientTest {
    stream: TcpStream,
}


impl kernel::Module for RustClientTest {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8080)) ;
        pr_info!("---INFO--- CREATED SOCKET SUCCESSFULLY !") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        
        let stream = connect(&remote_addr)?;
        pr_info!("---INFO--- CONNECTED TO PORT 8080 SUCCESSFULLY , BROADCAST MODE ENABLED ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        // Example number to send 
        
        let raw_data:[u8;52]= [
            //can_id
            0x00, 0x00, 0x22 , 0x33,
            //can_len
            0x44,
            //flags
            0x11,
            //data
            0xAA, 0x00, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 
            0x33, 0x44, 0x55, 0x66, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x00,0x22, 0x33, 0x44, 0x55, 0x00,  

        ];   
        //let hex_string = &raw_data;
        //pr_info!("Hex string: {:?}", hex_string);  


        let data_vec=raw_data; 
       
        pr_info!("---INFO--- PUSHING DATA (CANFD FRAME) TO THE GATEWAY ! ") ;
        coarse_sleep(Duration::from_secs(1)) ;
        send_data(&stream, data_vec)? ; 

        Ok(Self { stream })
    }
}



pub fn send_data(stream: &TcpStream, data: [u8; 52]) -> Result<usize> {
    // Ensure the data vector has exactly 13 elements
    
    
    let mut buffer = [0u8; 52];
    for (i, &item) in data.iter().enumerate() {
        if i >= 52 {
            break; // Prevent index out of bounds
        }
        buffer[i] = item;
    }

    pr_info!("RUST_CLIENT : SEND_DATA FUNCTION IS BEING CALLED") ; 
    coarse_sleep(Duration::from_secs(1)) ;
    // Write the data vector to the stream
    stream.write(&buffer,true)

    // Return the number of bytes written
   
}
*/

use kernel::{
    kasync::executor::{workqueue::Executor as WqExecutor, AutoStopHandle, Executor},
    kasync::net::{TcpListener, TcpStream},
    net::{self, Ipv4Addr, SocketAddr, SocketAddrV4},
    prelude::*,
    spawn_task,
    sync::{Arc, ArcBorrow},
    cegwfdtcp::* ,
    //eth_can_payload ::* ,
    delay::coarse_sleep ,
    
};
use core::time::Duration;
use kernel::net::*;
use ::alloc::format ; 
use core::fmt::Write ; 
use core::fmt ;


async fn echo_server(stream: TcpStream) -> Result {
    let mut buff = [0u8; 52 ];
        let n = stream.read(&mut buff).await?;
        coarse_sleep(Duration::from_secs(1)) ; 
        pr_info!(" Connection Established with Netlink Kernel Server ... ") ;
        coarse_sleep(Duration::from_secs(1)) ; 
        pr_info!(" Displaying Incoming CANFD Frame ...") ;
 
        coarse_sleep(Duration::from_secs(1)) ;
        if n == 0 {
            return Ok(());
        }
        let frame=canfdFrame::deserialize_canfd_payload(&buff).unwrap() ; 
        pr_info!("Deserializing CANFD frame in the Virtual CAN module") ;
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("Displaying CANFD frame Data ... ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  can_id: {:08x}", frame.can_id);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  len: {:02x}", frame.len);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  flags: {:02x}", frame.flags);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  data:");
        for chunk in frame.data.chunks(5) {
            let mut line = ArrayString::<46>::new();
            for byte in chunk {
                write!(&mut line, "{:02X} ", byte).unwrap();
            }
            pr_info!("{}", line);
        }
        /*for (i, byte) in frame.data.iter().enumerate() {
            if i % 10 == 0 && i != 0 {
                pr_info!("\n");
            }
            pr_info!("{:08x} ", byte);
            coarse_sleep(Duration::from_secs(1));
        }*/
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 7070)) ;
        let stream1 = connect(&remote_addr)?;
        pr_info!("PREPARING TO PUSH DATA (CANFD FRAME) TO THE GATEWAY MODULE . ") ;

        coarse_sleep(Duration::from_secs(1)) ;
        let stream1 = connect(&remote_addr)?;
        send_data(&stream1,array_to_vec(&buff))?;
        
        Ok(())
        //pr_info!("{:?}" ,frame.data) ;
        
        
       

        /*pr_info!("-------------------------------") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("start the conversion from CANFD to Ethernet \n");
        coarse_sleep(Duration::from_secs(1)) ;
        //pr_info!("buffer is {:?}", buff );
        //coarse_sleep(Duration::from_secs(1)) ;
        //pr_info!("-------------------------------") ; 
        //coarse_sleep(Duration::from_secs(1)) ;
        let canfd = canfdFrame ::deserialize_canfd_payload(&buff).unwrap();
        pr_info!("DONE DESERIALIZING THE CANFD FRAME ")  ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("PREPARING FOR A CONVERSION ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        let payload_data = EthCanfdpayLoad::to_eth_frame(&canfd) ; 
        pr_info!("DONE THE CONVERSION FROM CANFD FRAME INTO ETHERNET FRAME ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        //pr_info!("-------------------------------") ; 
        //coarse_sleep(Duration::from_secs(1)) ;
        //pr_info!("-------------------------------") ; 
        //coarse_sleep(Duration::from_secs(1)) ;
       // pr_info!("-------------------------------") ;
        pr_info!("PREPARING TO SEND TO THE ETHERNET DEVICE ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 7070)) ;
        pr_info!("---INFO--- CREATED GATEWAY SOCKET SUCCCESSFULLY !") ;
        coarse_sleep(Duration::from_secs(1)) ; 
        let stream1 = connect(&remote_addr)?;
        // Example number to send 
        let buf1=serialize_eth_canfd_payload(&payload_data) ; 
        pr_info!("len  {:}", buf1.len());

        pr_info!("DONE SERIALIZING THE ETHERNET FRAME , SENDING TO THE ETHERNET DEVICE") ;
        coarse_sleep(Duration::from_secs(1)) ;  
        send_data(&stream1, buf1)? ; //test
        pr_info!("---INFO--- SEND DATA FUNCTION IS BEING CALLED !") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("---INFO--- SENDING TO THE ETHERNET DEVICE ! ") ;
        coarse_sleep(Duration::from_secs(1)) ; */

        
    
   
    //stream.write_all(&buff[..n]).await?;
    }
    struct ArrayString<const N: usize> {
        buffer: [u8; N],
        length: usize,
    }
    
    impl<const N: usize> ArrayString<N> {
        fn new() -> Self {
            Self {
                buffer: [0; N],
                length: 0,
            }
        }
    
        fn as_str(&self) -> &str {
            core::str::from_utf8(&self.buffer[..self.length]).unwrap()
        }
    }
    

    impl<const N: usize> Write for ArrayString<N> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            let bytes = s.as_bytes();
            let new_length = self.length + bytes.len();
            if new_length > N {
                return Err(fmt::Error);
            }
            self.buffer[self.length..new_length].copy_from_slice(bytes);
            self.length = new_length;
            Ok(())
        }
    }
    
    impl<const N: usize> core::fmt::Display for ArrayString<N> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    


pub fn send_data(stream: &net::TcpStream, data: Vec<u8>) -> Result<usize> {
    // Ensure the data vector has exactly 52 elements
    let mut buffer = [0u8; 52 ];
    for (i, &item) in data.iter().enumerate() {
        buffer[i] = item;
    }
    // Write the data vector to the stream
    stream.write(&buffer,true)
    // Return the number of bytes written
   
}



async fn accept_loop(listener: TcpListener, executor: Arc<impl Executor>) {
    loop {
        if let Ok(stream) = listener.accept().await {
            let _ = spawn_task!(executor.as_arc_borrow(), echo_server(stream));
        }
    }
}

fn start_listener(ex: ArcBorrow<'_, impl Executor + Send + Sync + 'static>) -> Result {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8050));
    let listener = TcpListener::try_new(net::init_ns(), &addr)?;
    pr_info!(" Listening to Incoming Connexion ") ;
    spawn_task!(ex, accept_loop(listener, ex.into()))?;
    Ok(())
}


pub fn connect(address: &SocketAddr) -> Result<net::TcpStream> {
    let socket = Socket::new(AddressFamily::Inet, SockType::Stream, IpProtocol::Tcp)?;
    socket.connect(address, 0)?; 
    Ok(net::TcpStream {sock:unsafe{socket.as_inner()}})
}

fn array_to_vec(arr: &[u8; 52]) -> Vec<u8> {
    let mut vec = Vec::new();
    for &item in arr.iter() {
        vec.try_push(item);
    }
    vec
}


struct RustEchoServer {
    _handle: AutoStopHandle<dyn Executor>,
}

impl kernel::Module for RustEchoServer {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let handle = WqExecutor::try_new(kernel::workqueue::system())?;
        
        start_listener(handle.executor())?;
        //echo_server(stream);
        Ok(Self {
            _handle: handle.into(),
        })
    }
}

module! {
    type: RustEchoServer,
    name: "RUST_VIRTUAL_CAN_DEVICE",
    author: "Rust for Linux Contributors",
    description: "Rust gateway",
    license: "GPL",
}



