// SPDX-License-Identifier: GPL-2.0

//! Rust echo server sample.

use kernel::{
    kasync::executor::{workqueue::Executor as WqExecutor, AutoStopHandle, Executor},
    kasync::net::{TcpListener, TcpStream},
    net::{self, Ipv4Addr, SocketAddr, SocketAddrV4},
    prelude::*,
    spawn_task,
    sync::{Arc, ArcBorrow},
    cegwtcp::*
};

async fn echo_server(stream: TcpStream) -> Result {
    let mut buf = [0u8; 52];
    loop {
        let n = stream.read(&mut buf).await?;
        pr_info!("echoserverread") ;
        pr_info!("buffer is{:?}", buf) ; 
        let canfd_result = CanFdEthload::deserialize_CanFdEthload(&buf).unwrap();
        pr_info!("------------------------------------") ; 
   
            pr_info!("can_id: 0x{:0x}", canfd_result.can_id);
            pr_info!("len: {}", canfd_result.len);
            pr_info!("flags: 0x{:0x}", canfd_result.flags);
            pr_info!("data:");
            pr_info!("  - IP header:");
            pr_info!("    - version: {:?}", canfd_result.data_can.iphdr.version);
            // ... Add formatting for other IP header fields
            pr_info!("  - TCP header:");
            pr_info!("    - src_port: {}", canfd_result.data_can.tcphdr.src_port);
            // ... Add formatting for other TCP header fields
            //pr_info!("  - TCP data: {:?}", canfd_result.data_can.data_eth);
        if n == 0 {
            return Ok(());
        }

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
    pr_info!("LIstening") ;
    spawn_task!(ex, accept_loop(listener, ex.into()))?;
    Ok(())
}

struct RustEchoServer {
    _handle: AutoStopHandle<dyn Executor>,
}

impl kernel::Module for RustEchoServer {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let handle = WqExecutor::try_new(kernel::workqueue::system())?;
        start_listener(handle.executor())?;
        pr_info!("----------------------") ; 
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



