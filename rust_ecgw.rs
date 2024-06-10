// SPDX-License-Identifier: GPL-2.0

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
/* 
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
// SPDX-License-Identifier: GPL-2.0

//! Rust echo server sample.

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


async fn echo_server(stream: TcpStream) -> Result {
    let mut buff = [0u8; 52 ];
        let n = stream.read(&mut buff).await?;
        
   
        if n == 0 {
            return Ok(());
        }
        pr_info!("RECEIVING CANFD FRAME FROM THE NETLINK CLIENT ! ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("Starting the conversion from CANFD to Ethernet \n");
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
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 7000)) ;
        pr_info!("---INFO--- CREATED GATEWAY SOCKET SUCCCESSFULLY !") ;
        coarse_sleep(Duration::from_secs(1)) ; 
        let stream1 = connect(&remote_addr)?;
        // Example number to send 
        let buf1=serialize_eth_canfd_payload(&payload_data) ; 
        pr_info!("DONE SERIALIZING THE ETHERNET FRAME , SENDING TO THE ETHERNET DEVICE") ;
        coarse_sleep(Duration::from_secs(1)) ;  
        send_data(&stream1, buf1)? ; //test
        pr_info!("---INFO--- SEND DATA FUNCTION IS BEING CALLED !") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("---INFO--- SENDING TO THE ETHERNET DEVICE ! ") ;
        coarse_sleep(Duration::from_secs(1)) ; 

        
        Ok(())
   
    //stream.write_all(&buff[..n]).await?;
    }



pub fn send_data(stream: &net::TcpStream, data: Vec<u8>) -> Result<usize> {
    // Ensure the data vector has exactly 52 elements
    let mut buffer = [0u8; 102 ];
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
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 7070));
    let listener = TcpListener::try_new(net::init_ns(), &addr)?;
    pr_info!(" listening") ;
    spawn_task!(ex, accept_loop(listener, ex.into()))?;
    Ok(())
}


pub fn connect(address: &SocketAddr) -> Result<net::TcpStream> {
    let socket = Socket::new(AddressFamily::Inet, SockType::Stream, IpProtocol::Tcp)?;
    socket.connect(address, 0)?; 
    Ok(net::TcpStream {sock:unsafe{socket.as_inner()}})
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
    name: "RUST_CAN_ETHERNET_MODULE",
    author: "Rust for Linux Contributors",
    description: "Rust gateway",
    license: "GPL",
}

