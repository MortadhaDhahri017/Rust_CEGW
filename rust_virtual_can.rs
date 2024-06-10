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
    let mut buf = [0u8; 52];
        let n = stream.read(&mut buf).await?;
        coarse_sleep(Duration::from_secs(1));
        pr_info!("RECEIVING DATA FROM THE GATEWAY MODULE : MODE ETHERNET->CANFD") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("DESERIALIZING THE CONVERTED CANFD FRAME") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        let canfd = CanfdEthpayload::deserialize_CanfdEthpayload(&buf).unwrap();
        pr_info!("CAN Frame Information:");
            coarse_sleep(Duration::from_millis(500));  
            pr_info!("- CAN ID: 0x{:0x}", canfd.can_id);
            coarse_sleep(Duration::from_millis(500));  
            pr_info!("- Length: {}", canfd.len);
            coarse_sleep(Duration::from_millis(200));  
            pr_info!("- Flags: 0x{:0x}", canfd.flags);
            coarse_sleep(Duration::from_millis(200));  
   
       // Access and print Ethload data
            pr_info!("\n  - Ethernet Payload:");
            coarse_sleep(Duration::from_millis(500));  
            pr_info!("- IP Header:");
            coarse_sleep(Duration::from_millis(500));  
            pr_info!("- Version: {:?}", canfd.data_can.iphdr.version);
            coarse_sleep(Duration::from_millis(500));
            pr_info!("- Source IP: {}.{}.{}.{}",
            canfd.data_can.iphdr.src[0], canfd.data_can.iphdr.src[1], canfd.data_can.iphdr.src[2], canfd.data_can.iphdr.src[3]);
            pr_info!("- Destination IP: {}.{}.{}.{}",
            canfd.data_can.iphdr.dst[0], canfd.data_can.iphdr.dst[1],canfd.data_can.iphdr.dst[2], canfd.data_can.iphdr.dst[3]); 
            pr_info!("- TCP Header:");
            coarse_sleep(Duration::from_millis(500));  
            pr_info!("- Source Port: 0x{:0x}", canfd.data_can.tcphdr.src_port);
            coarse_sleep(Duration::from_millis(500));  
            pr_info!("- Destination Port: 0x{:0x}", canfd.data_can.tcphdr.dst_port);
            coarse_sleep(Duration::from_millis(500));  
            pr_info!("- Sequence Number: {}", canfd.data_can.tcphdr.seq);
            coarse_sleep(Duration::from_millis(500));  
            pr_info!("- Acknowledgment Number: {}", canfd.data_can.tcphdr.ack);
            coarse_sleep(Duration::from_millis(500));  

        // Improved output for TCP flags:
            pr_info!("- Flags: {:?}", canfd.data_can.tcphdr.flags); // Use Debug trait for detailed flag information
            coarse_sleep(Duration::from_millis(500)); 
        // Additional TCP header fields (consider including only relevant ones):
            pr_info!("- Data Offset: {}", canfd.data_can.tcphdr.offset);
            coarse_sleep(Duration::from_millis(500)); 
            pr_info!("- Window: {}", canfd.data_can.tcphdr.window);
            coarse_sleep(Duration::from_millis(500)); 
            pr_info!("- Checksum: {:x}", canfd.data_can.tcphdr.checksum); // Hexadecimal format for checksum
            coarse_sleep(Duration::from_millis(500)); 
    // Payload information:


            pr_info!("- Payload");
            for i in 0..14 {
                if i < canfd.data_can.data_eth.len() {
                pr_info!("  - Byte {}: {:02X}", i, canfd.data_can.data_eth[i]);
                coarse_sleep(Duration::from_millis(200)); 
                }   else {
                pr_info!("  - Byte {}: 00", i);
                }
            }    
         
             /*let first_six_bytes = &buf[0..6];
    
                // Extract the last 4 bytes
                let last_four_bytes = &buf[48..52];

                // Create a new buffer to hold the concatenated result
                let mut concatenated_buffer = Vec::try_with_capacity(10)?;

                // Append the first 6 bytes
                concatenated_buffer.try_extend_from_slice(first_six_bytes)?;
                
                // Append the last 4 bytes
                concatenated_buffer.try_extend_from_slice(last_four_bytes)?;*/
              

                let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192,168,75,128),5550)) ;
                pr_info!("Sending Data to IP Address 192.168.75.128 For Diagnostics") ;
                coarse_sleep(Duration::from_secs(1)) ; 
                let stream1 = connect(&remote_addr)?;
                pr_info!("Initializing the CANFD Chronogram ... ") ; 
                send_data(&stream1, array_to_vec(&buf))? ; 

            pr_info!("--------------------------------------------------------------") ; 
            
        return Ok(());

        //stream.write_all(&buf[..n]).await?;
        

}


/* 
fn correct_bit_stuffing_error(mut can_frame: [u8; 52]) -> [u8; 52] {
    let mut last_bit = can_frame[0] & 0x01;
    let mut count = 1;

    for byte_idx in 0..can_frame.len() {
        let byte = can_frame[byte_idx];
        for bit_pos in 0..8 {
            let current_bit = (byte >> bit_pos) & 0x01;
            if current_bit == last_bit {
                count += 1;
                if count > 5 {
                    // Correct the bit stuffing error by inserting a 0
                    can_frame[byte_idx] &= !(0x01 << (7 - bit_pos));
                    pr_info!("Fixed the bit stuffing error at byte {} and bit position {}", byte_idx, bit_pos);
                    coarse_sleep(Duration::from_millis(500)); 
                    count = 1;
                }
            } else {
                last_bit = current_bit;
                count = 1;
            }
        }
    }

    can_frame
}*/




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
        if i >=52{
            break; // Prevent index out of bounds
        }
        buffer[i] = item;
    }
    // Write the data vector to the stream
    stream.write(&buffer,true)

    // Return the number of bytes written
   
}



async fn accept_loop(listener: TcpListener, executor: Arc<impl Executor>) {
    loop {
        pr_info!("Waiting for a Connection on Port 8000.");
        coarse_sleep(Duration::from_millis(500)) ;  
        if let Ok(stream) = listener.accept().await {
            pr_info!("Connexion Established on Port 8000.");
            coarse_sleep(Duration::from_millis(500)) ; 
            let _ = spawn_task!(executor.as_arc_borrow(), echo_server(stream));
        }
    }
}

fn start_listener(ex: ArcBorrow<'_, impl Executor + Send + Sync + 'static>) -> Result {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8000));
    let listener = TcpListener::try_new(net::init_ns(), &addr)?;
    spawn_task!(ex, accept_loop(listener, ex.into()))?;
    Ok(())
}

struct RustEchoServer2 {
    _handle: AutoStopHandle<dyn Executor>,
}

impl kernel::Module for RustEchoServer2 {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Virtual CAN Module Init Function Has been Called") ;
        let handle = WqExecutor::try_new(kernel::workqueue::system())?;
        start_listener(handle.executor())?; 
        Ok(Self {
            _handle: handle.into(),
        })
    }
}

fn array_to_vec(arr: &[u8; 52]) -> Vec<u8> {
    let mut vec = Vec::new();
    for &item in arr.iter() {
        vec.try_push(item);
    }
    vec
}

 

module! {
    type: RustEchoServer2,
    name: "RUST_VIRTUAL_CAN_DEVICE",
    author: "Rust for Linux Contributors",
    description: "Rust tcp echo sample",
    license: "GPL v2",
}






/* // SPDX-License-Identifier: GPL-2.0

//! Rust semaphore sample.
//!
//! A counting semaphore that can be used by userspace.
//!
//! The count is incremented by writes to the device. A write of `n` bytes results in an increment
//! of `n`. It is decremented by reads; each read results in the count being decremented by 1. If
//! the count is already zero, a read will block until another write increments it.
//!
//! This can be used in user space from the shell for example  as follows (assuming a node called
//! `semaphore`): `cat semaphore` decrements the count by 1 (waiting for it to become non-zero
//! before decrementing); `echo -n 123 > semaphore` increments the semaphore by 3, potentially
//! unblocking up to 3 blocked readers.

use core::sync::atomic::{AtomicU64, Ordering};
use kernel::{
    condvar_init,
    file::{self, File, IoctlCommand, IoctlHandler},
    io_buffer::{IoBufferReader, IoBufferWriter},
    miscdev::Registration,
    mutex_init,
    prelude::*,
    sync::{Arc, CondVar, Mutex, UniqueArc},
    user_ptr::{UserSlicePtrReader, UserSlicePtrWriter},
};

module! {
    type: RustSemaphore,
    name: "rust_semaphore",
    author: "Rust for Linux Contributors",
    description: "Rust semaphore sample",
    license: "GPL",
}

struct SemaphoreInner {
    count: usize,
    max_seen: usize,
}

struct Semaphore {
    changed: CondVar,
    inner: Mutex<SemaphoreInner>,
}

struct FileState {
    read_count: AtomicU64,
    shared: Arc<Semaphore>,
}

impl FileState {
    fn consume(&self) -> Result {
        let mut inner = self.shared.inner.lock();
        while inner.count == 0 {
            if self.shared.changed.wait(&mut inner) {
                return Err(EINTR);
            }
        }
        inner.count -= 1;
        Ok(())
    }
}

#[vtable]
impl file::Operations for FileState {
    type Data = Box<Self>;
    type OpenData = Arc<Semaphore>;

    fn open(shared: &Arc<Semaphore>, _file: &File) -> Result<Box<Self>> {
        Ok(Box::try_new(Self {
            read_count: AtomicU64::new(0),
            shared: shared.clone(),
        })?)
    }

    fn read(this: &Self, _: &File, data: &mut impl IoBufferWriter, offset: u64) -> Result<usize> {
        if data.is_empty() || offset > 0 {
            return Ok(0);
        }
        this.consume()?;
        data.write_slice(&[0u8; 1])?;
        this.read_count.fetch_add(1, Ordering::Relaxed);
        Ok(1)
    }

    fn write(this: &Self, _: &File, data: &mut impl IoBufferReader, _offs: u64) -> Result<usize> {
        {
            let mut inner = this.shared.inner.lock();
            inner.count = inner.count.saturating_add(data.len());
            if inner.count > inner.max_seen {
                inner.max_seen = inner.count;
            }
        }

        this.shared.changed.notify_all();
        Ok(data.len())
    }

    fn ioctl(this: &Self, file: &File, cmd: &mut IoctlCommand) -> Result<i32> {
        cmd.dispatch::<Self>(this, file)
    }
}

struct RustSemaphore {
    _dev: Pin<Box<Registration<FileState>>>,
}

impl kernel::Module for RustSemaphore {
    fn init(name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust semaphore sample (init)\n");

        let mut sema = Pin::from(UniqueArc::try_new(Semaphore {
            // SAFETY: `condvar_init!` is called below.
            changed: unsafe { CondVar::new() },

            // SAFETY: `mutex_init!` is called below.
            inner: unsafe {
                Mutex::new(SemaphoreInner {
                    count: 0,
                    max_seen: 0,
                })
            },
        })?);

        // SAFETY: `changed` is pinned when `sema` is.
        let pinned = unsafe { sema.as_mut().map_unchecked_mut(|s| &mut s.changed) };
        condvar_init!(pinned, "Semaphore::changed");

        // SAFETY: `inner` is pinned when `sema` is.
        let pinned = unsafe { sema.as_mut().map_unchecked_mut(|s| &mut s.inner) };
        mutex_init!(pinned, "Semaphore::inner");

        Ok(Self {
            _dev: Registration::new_pinned(fmt!("{name}"), sema.into())?,
        })
    }
}

impl Drop for RustSemaphore {
    fn drop(&mut self) {
        pr_info!("Rust semaphore sample (exit)\n");
    }
}

const IOCTL_GET_READ_COUNT: u32 = 0x80086301;
const IOCTL_SET_READ_COUNT: u32 = 0x40086301;

impl IoctlHandler for FileState {
    type Target<'a> = &'a Self;

    fn read(this: &Self, _: &File, cmd: u32, writer: &mut UserSlicePtrWriter) -> Result<i32> {
        match cmd {
            IOCTL_GET_READ_COUNT => {
                writer.write(&this.read_count.load(Ordering::Relaxed))?;
                Ok(0)
            }
            _ => Err(EINVAL),
        }
    }

    fn write(this: &Self, _: &File, cmd: u32, reader: &mut UserSlicePtrReader) -> Result<i32> {
        match cmd {
            IOCTL_SET_READ_COUNT => {
                this.read_count.store(reader.read()?, Ordering::Relaxed);
                Ok(0)
            }
            _ => Err(EINVAL),
        }
    }
}
*/