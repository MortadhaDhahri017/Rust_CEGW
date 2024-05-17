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


use kernel::net::*;
use kernel::error::*;
use core::*;
use kernel::bindings ;


pub fn connect(address: &SocketAddr) -> Result<TcpStream> {
    let socket = Socket::new(AddressFamily::Inet, SockType::Stream, IpProtocol::Tcp)?;
    socket.connect(address, 0)?; 
    pr_info!("RUST_NETLINK CONNECT FUNCTION IS BEING CALLED "); 
    coarse_sleep(Duration::from_secs(1)) ;
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
    coarse_sleep(Duration::from_secs(1)) ;
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
        coarse_sleep(Duration::from_secs(1)) ;
        
        let stream = connect(&remote_addr)?;
        pr_info!("---INFO--- CONNECTED TO PORT 8080 SUCCESSFULLY , BROADCAST MODE ENABLED ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        // Example number to send 
        
        let raw_data:[u8;62]= [
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
            0x00, 0x00, // Checksum (placeholder, needs calculation)
            192, 168, 1,  1, // Source IP address
            192, 168, 1,  2, // Destination IP address
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
        ];           // TCP data (4 bytes)
       
        let data_vec=raw_data.try_to_vec()? ; 
       
        pr_info!("---INFO--- PUSHING DATA (ETHERNET FRAME) TO THE GATEWAY ! ") ;
        coarse_sleep(Duration::from_secs(1)) ;
        send_data(&stream, data_vec)? ; 

        Ok(Self { stream })
    }
}

/* 
//! Virtual Device Module

//use core::intrinsics::offset;

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
use core::str::* ; 
//use kernel::tcp::{EthFrame , canfd_ethpayload , TCP_Frame , Ipv4Frame , Ipv4Header , TcpHeader ,EtherType} ; 
use kernel::cegwtcp::*;
use alloc::vec::*; 


module! {
    type: VDev,
    name: "vdev",
    license: "GPL",
    params: {
        devices: u32 {
            default: 1,
            permissions: 0o644,
            description: "Number of virtual devices",
        },
    },
}
struct Device {
    number: usize,
    contents: Mutex<Vec<u8>>,
}

struct VDev {
    _devs: Vec<Pin<Box<miscdev::Registration<VDev>>>>,
}

#[vtable]
impl Operations for VDev {
    type OpenData = Arc<Device>;
    type Data = Arc<Device>;

    fn open(context: &Arc<Device>, file: &File) -> Result<Arc<Device>> {
        pr_info!("File for device {} was opened\n", context.number);
        if file.flags() & flags::O_ACCMODE == flags::O_WRONLY {
            context.contents.lock().clear();
        }
        Ok(context.clone())
    }

    fn read(
        data: ArcBorrow<'_, Device>,
        _file: &File,
        writer: &mut impl IoBufferWriter ,
        offset: u64,
        ) -> Result<usize> {
        pr_info!("File for device {} was read\n", data.number);
        let offset = offset.try_into()?;
        let vec = data.contents.lock();
        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(offset));

    
    
        if  len != 53 {
            pr_err!("DATA LENGTH IS NOT ADEQUATE , PLEASE INSERT VALID DATA !") ; 
            coarse_sleep(Duration::from_secs(1)) ; 
            pr_err!("RUST_GW ERROR 101 , ABORTING CONVERSION ") ; 
            coarse_sleep(Duration::from_secs(1)) ;
            pr_info!("------------------------------") ; 
            return  Ok(0) ; 
        }
        else {
            pr_warn!(" WARNING : PLEASE DON'T INTERRUPT THE CONVERSION , DON'T PRESS ANY KEY !") ; 
            coarse_sleep(Duration::from_secs(1)) ;

            let canfd=canfd_ethpayload::deserialize_canfd_ethpayload(&vec).unwrap() ; 
            
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
            pr_info!("- TCP Header:");
            coarse_sleep(Duration::from_millis(500));  
            pr_info!("- Source Port: {}", canfd.data_can.tcphdr.src_port);
            coarse_sleep(Duration::from_millis(500));  
            pr_info!("- Destination Port: {}", canfd.data_can.tcphdr.dst_port);
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
            for i in 0..5 {
                if i < canfd.data_can.data_eth.len() {
                pr_info!("  - Byte {}: {:02X}", i, canfd.data_can.data_eth[i]);
                coarse_sleep(Duration::from_millis(200)); 
                }   else {
                pr_info!("  - Byte {}: (no data)", i);
                }
            } 
            
            return Ok(0) ;
        }   
    } 
        
    

        fn write(
            data: ArcBorrow<'_, Device>,
            _file: &File,
            reader: &mut impl IoBufferReader,
            offset: u64,
            ) -> Result<usize> {
            pr_info!("File for device {} was written\n", data.number);
            let offset = offset.try_into()?;
            let len = reader.len();
            let new_len = len.checked_add(offset).ok_or(EINVAL)?;
            let mut vec = data.contents.lock();
            if new_len > vec.len() {
            vec.try_resize(new_len, 0)?;
            }
            reader.read_slice(&mut vec[offset..][..len])?;

           
        
        Ok(len)
            
}
}



impl Module for VDev {
    fn init(_name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        let count = {
            let lock = module.kernel_param_lock();
            (*devices.read(&lock)).try_into()?
        };
        pr_info!("-----------------------\n");
        pr_info!("starting {} vdevices!\n", count);
        pr_info!("watching for changes...\n");
        pr_info!("-----------------------\n");

        
        
        let mut devs = Vec::try_with_capacity(count)?;
        for i in 0..count {
            let dev = Arc::try_new(Device {
                number: i,
                contents: Mutex::new(Vec::new()),
            })?;
            let reg = miscdev::Registration::new_pinned(fmt!("vdev{i}"), dev)?;
            devs.try_push(reg)?;
        }
        Ok(Self { _devs: devs })
    }
}

impl Drop for VDev{
    fn drop(&mut self) {
        pr_info!("Rust CAN sample (exit)\n");
    }
}
*/
/* This is for testing !

    let mock_frame = EthFrame {
                dst: [0x00, 0x0C, 0x29, 0x70, 0xAC, 0x1E],  // Destination MAC address (example)
                src: [0x00, 0x5E, 0x70, 0x01, 0x02, 0x03],  // Source MAC address (example)
                ethertype: EtherType::IPV4,
                data_ipv4: Ipv4Frame {
                    header: Ipv4Header {
                        version: 4,
                        len: 20,
                        ToS: 0,
                        total_len: 100,
                        id: 10,
                        flags: 0,
                        fragoffset: 0,
                        ttl: 64,
                        protocol: 6,  // TCP protocol
                        checksum: 0xABCD,
                        src: [192, 168, 1, 10],  // Source IP address (example)
                        dst: [10, 0, 0, 1],       // Destination IP address (example)
                    },
                    data_tcp: TCP_Frame {
                        header: TcpHeader {
                            src_port: 80,  // Example port number
                            dst_port: 443, // Example port number
                            seq: 0,
                            ack: 0,
                            offset: 5,    // Header length in 32-bit words (20 bytes)
                            reserved: 0,
                            flags: 0,
                            window: 1024,
                            checksum: 0x1234,
                            urgent_ptr: 0,
                        },
                        data: [0xCA, 0xFE, 0xBA, 0xBE , 0xAC , 0xEF], // Example TCP data (first 4 bytes)
                    },
                },
                fsc: 0xDEADBEEF,
            } ; 
            
            let can_eth_frame=canfd_ethpayload::from_eth_frame(&mock_frame) ; 
            let serialized_vec=serialize_canfd_ethpayload(&can_eth_frame) ; 
            let slice = serialized_vec.as_ref() ; 
            let canfd = canfd_ethpayload::deserialize_canfd_ethpayload(slice).unwrap();
            pr_info!("------------------------------------") ; 
       
                pr_info!("can_id: 0x{:0x}", canfd.can_id);
                pr_info!("len: {}", canfd.len);
                pr_info!("flags: 0x{:0x}", canfd.flags);
                pr_info!("data:");
                pr_info!("  - IP header:");
                pr_info!("    - version: {:?}", canfd.data_can.iphdr.version);
                // ... Add formatting for other IP header fields
                pr_info!("  - TCP header:");
                pr_info!("    - src_port: {}", canfd.data_can.tcphdr.src_port);
                // ... Add formatting for other TCP header fields
                //pr_info!("  - TCP data: {:?}", canfd.data_can.data_eth);
           
           Ok(len) 


*/

/*

    fn read(
        data: ArcBorrow<'_, Device>,
        _file: &File,
        writer: &mut impl IoBufferWriter,
        offset: u64,
        ) -> Result<usize> {
        pr_info!("File for device {} was read\n", data.number);
        let offset = offset.try_into()?;
        let vec = data.contents.lock();
        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(offset));
        writer.write_slice(&vec[offset..][..len])?;
        let canfd = canfd_ethpayload::deserialize_canfd_ethpayload(&data.contents.unwrap()).unwrap();
        pr_info!("------------------------------------") ; 
         // Print CAN frame information
        pr_info!("CAN Frame Information:");
        pr_info!("- CAN ID: 0x{:0x}", canfd.can_id);
        pr_info!("- Length: {}", canfd.len);
        pr_info!("- Flags: 0x{:0x}", canfd.flags);

    // Access and print Ethload data
        pr_info!("\n  - Ethernet Payload:");
        pr_info!("- IP Header:");
        pr_info!("- Version: {:?}", canfd.data_can.iphdr.version);
        pr_info!("- TCP Header:");
        pr_info!("- Source Port: {}", canfd.data_can.tcphdr.src_port);
        pr_info!("- Destination Port: {}", canfd.data_can.tcphdr.dst_port);
        pr_info!("- Sequence Number: {}", canfd.data_can.tcphdr.seq);
        pr_info!("- Acknowledgment Number: {}", canfd.data_can.tcphdr.ack);
          
        Ok(len)
        } */

    /*
     let mock_frame = EthFrame {
                dst: [0x00, 0x0C, 0x29, 0x70, 0xAC, 0x1E],  // Destination MAC address (example)
                src: [0x00, 0x5E, 0x70, 0x01, 0x02, 0x03],  // Source MAC address (example)
                ethertype: EtherType::IPV4,
                data_ipv4: Ipv4Frame {
                    header: Ipv4Header {
                        version: 4,
                        len: 20,
                        ToS: 0,
                        total_len: 100,
                        id: 10,
                        flags: 0,
                        fragoffset: 0,
                        ttl: 64,
                        protocol: 6,  // TCP protocol
                        checksum: 0xABCD,
                        src: [192, 168, 1, 10],  // Source IP address (example)
                        dst: [10, 0, 0, 1],       // Destination IP address (example)
                    },
                    data_tcp: TCP_Frame {
                        header: TcpHeader {
                            src_port: 80,  // Example port number
                            dst_port: 443, // Example port number
                            seq: 0,
                            ack: 0,
                            offset: 5,    // Header length in 32-bit words (20 bytes)
                            reserved: 0,
                            flags: 0,
                            window: 1024,
                            checksum: 0x1234,
                            urgent_ptr: 0,
                        },
                        data: [0xCA, 0xFE, 0xBA, 0xBE , 0xAC , 0xEF], // Example TCP data (first 4 bytes)
                    },
                },
                fsc: 0xDEADBEEF,
            } ; 
            
            let can_eth_frame=canfd_ethpayload::from_eth_frame(&mock_frame) ; 
            let serialized_vec=serialize_canfd_ethpayload(&can_eth_frame) ; 
            let slice = serialized_vec.as_ref() ; 
            let canfd = canfd_ethpayload::deserialize_canfd_ethpayload(slice).unwrap();
            pr_info!("------------------------------------") ; 
    
            // Print CAN frame information
            pr_info!("CAN Frame Information:");
            pr_info!("- CAN ID: 0x{:0x}", canfd.can_id);
            pr_info!("- Length: {}", canfd.len);
            pr_info!("- Flags: 0x{:0x}", canfd.flags);
   
       // Access and print Ethload data
            pr_info!("\n  - Ethernet Payload:");
            pr_info!("- IP Header:");
            pr_info!("- Version: {:?}", canfd.data_can.iphdr.version);
            pr_info!("- TCP Header:");
            pr_info!("- Source Port: {}", canfd.data_can.tcphdr.src_port);
            pr_info!("- Destination Port: {}", canfd.data_can.tcphdr.dst_port);
            pr_info!("- Sequence Number: {}", canfd.data_can.tcphdr.seq);
            pr_info!("- Acknowledgment Number: {}", canfd.data_can.tcphdr.ack);

        // Improved output for TCP flags:
            pr_info!("- Flags: {:?}", canfd.data_can.tcphdr.flags); // Use Debug trait for detailed flag information

        // Additional TCP header fields (consider including only relevant ones):
    pr_info!("- Data Offset: {}", canfd.data_can.tcphdr.offset);
    pr_info!("- Window: {}", canfd.data_can.tcphdr.window);
    pr_info!("- Checksum: {:x}", canfd.data_can.tcphdr.checksum); // Hexadecimal format for checksum

    // Payload information:
    pr_info!("- Payload (first 16 bytes):");
    for i in 0..16 {
        if i < canfd.data_can.data_eth.len() {
            pr_info!("  - Byte {}: {:02X}", i, canfd.data_can.data_eth[i]);
        }   else {
            pr_info!("  - Byte {}: (no data)", i);
        }
    } */



    /*
    
        let canfd = canfd_ethpayload::deserialize_canfd_ethpayload(&vec48).unwrap();
 
        pr_info!("------------------------------------") ; 
         // Print CAN frame information
        pr_info!("CAN Frame Information:");
        pr_info!("- CAN ID: 0x{:0x}", canfd.can_id);
        pr_info!("- Length: {}", canfd.len);
        pr_info!("- Flags: 0x{:0x}", canfd.flags);

    // Access and print Ethload data
        pr_info!("\n  - Ethernet Payload:");
        pr_info!("- IP Header:");
        pr_info!("- Version: {:?}", canfd.data_can.iphdr.version);
        pr_info!("\n  - TCP Header:");
        pr_info!("- Source Port: {}", canfd.data_can.tcphdr.src_port);
        pr_info!("- Destination Port: {}", canfd.data_can.tcphdr.dst_port);
        pr_info!("- Sequence Number: {}", canfd.data_can.tcphdr.seq);
        pr_info!("- Acknowledgment Number: {}", canfd.data_can.tcphdr.ack);

        // Improved output for TCP flags:
        pr_info!("- Flags: {:?}", canfd.data_can.tcphdr.flags); // Use Debug trait for detailed flag information

        // Additional TCP header fields (consider including only relevant ones):
        pr_info!("- Data Offset: {}", canfd.data_can.tcphdr.offset);
        pr_info!("- Window: {}", canfd.data_can.tcphdr.window);
        pr_info!("- Checksum: {:x}", canfd.data_can.tcphdr.checksum); // Hexadecimal format for checksum 
*/

/*
    let mock = EthCanfdLoad {
            dst_mac: [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE],
            src_mac: [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB],
            ethertype: EtherType::IPV4,
            data: Ethload {
                iphdr: Ipv4Header {
                    version: 4,
                    len: 20, // Assuming minimum header length
                    ToS: 1, // ToS (Type of Service) field example
                    total_len: 50, // Assuming total length (including data)
                    id: 100, // Example ID
                    flags: 0, // Flags field example (no flags set)
                    frag_offset: 0, // Fragment offset (no fragmentation)
                    ttl: 64, // Time to live example
                    protocol: 6, // TCP protocol
                    checksum: 0xABCD, // Mock checksum value
                    src: [192, 168, 1, 10],
                    dst: [10, 0, 0, 1],
                },
                tcphdr: TcpHeader {
                    src_port: 8080,
                    dst_port: 4433,
                    seq: 1234567890,
                    ack: 987654321,
                    offset: 5, // Assuming header length (5 words)
                    reserved: 0, // Reserved bits (set to 0)
                    flags: 0, // Flags field example (no flags set)
                    window: 1024, // Example window size
                    checksum: 0x1234, // Mock checksum value
                    urgent_ptr: 0, // Urgent pointer (not used here)
                },
                data: [0xCA, 0xFE, 0xBA, 0xBE], // Example data
            }

    }; 
   /*     
    let payload = to_eth_frame(mock) ; 
    println!("-- Eth Can Frame --");
    println!("  Destination MAC: {:?}", payload.dst);
    println!("  Source MAC: {:?}", payload.src);
   // println!("  Ethertype: {:?}", payload.ethertype);

    // Print IP Header details
    println!("-- IP Header --");
    println!("  Version: {}", payload.data.header.version);
    println!("  Header Length: {} bytes", payload.data.header.len * 4);
    println!("  Type of Service: {}", payload.data.header.to_s);
    println!("  Total Length: {} bytes", payload.data.header.total_len);
    println!("  Identification: {}", payload.data.header.id);
    println!("  Flags: {:x}", payload.data.header.flags);
    println!("  Fragment Offset: {} bytes", payload.data.header.frag_offset & 0x1f);
    println!("  Time to Live: {} hops", payload.data.header.ttl);
    */
    /*let vec=serialize_eth_canfd_payload(&mock) ; 
    pr_info!("le vec {:?}",vec) ;  */
