
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

use kernel::tcp::{EthFrame , canfd_ethpayload , TCP_Frame , Ipv4Frame , Ipv4Header , TcpHeader ,EtherType} ; 
use kernel::tcp::{serialize_canfd_ethpayload};
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
        
        /*let mut vec48:Vec<u8>=Vec::new() ; 
        for i in 0..52 {
            vec48.try_push(vec[i]-48) ; 
        }*/
        
       
        
        Ok(0)
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

async fn send_data_to_server(data: &[u8], server_addr: &str) -> Result<(), Error> {
    // Connect to the server
    let mut stream = TcpStream::connect(server_addr).await?;
  
    // Send the data
    stream.write_all(data).await?;
  
    // Optional: Receive response (if applicable for your server interaction)
    // let mut response_buf = [0u8; 1024];
    // let _ = stream.read(&mut response_buf).await?;
  
    Ok(())
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
            let canfd_result = canfd_ethpayload::deserialize_canfd_ethpayload(slice).unwrap();
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
        let canfd_result = canfd_ethpayload::deserialize_canfd_ethpayload(&data.contents.unwrap()).unwrap();
        pr_info!("------------------------------------") ; 
         // Print CAN frame information
        pr_info!("CAN Frame Information:");
        pr_info!("- CAN ID: 0x{:0x}", canfd_result.can_id);
        pr_info!("- Length: {}", canfd_result.len);
        pr_info!("- Flags: 0x{:0x}", canfd_result.flags);

    // Access and print Ethload data
        pr_info!("\n  - Ethernet Payload:");
        pr_info!("- IP Header:");
        pr_info!("- Version: {:?}", canfd_result.data_can.iphdr.version);
        pr_info!("- TCP Header:");
        pr_info!("- Source Port: {}", canfd_result.data_can.tcphdr.src_port);
        pr_info!("- Destination Port: {}", canfd_result.data_can.tcphdr.dst_port);
        pr_info!("- Sequence Number: {}", canfd_result.data_can.tcphdr.seq);
        pr_info!("- Acknowledgment Number: {}", canfd_result.data_can.tcphdr.ack);
          
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
            let canfd_result = canfd_ethpayload::deserialize_canfd_ethpayload(slice).unwrap();
            pr_info!("------------------------------------") ; 
    
            // Print CAN frame information
            pr_info!("CAN Frame Information:");
            pr_info!("- CAN ID: 0x{:0x}", canfd_result.can_id);
            pr_info!("- Length: {}", canfd_result.len);
            pr_info!("- Flags: 0x{:0x}", canfd_result.flags);
   
       // Access and print Ethload data
            pr_info!("\n  - Ethernet Payload:");
            pr_info!("- IP Header:");
            pr_info!("- Version: {:?}", canfd_result.data_can.iphdr.version);
            pr_info!("- TCP Header:");
            pr_info!("- Source Port: {}", canfd_result.data_can.tcphdr.src_port);
            pr_info!("- Destination Port: {}", canfd_result.data_can.tcphdr.dst_port);
            pr_info!("- Sequence Number: {}", canfd_result.data_can.tcphdr.seq);
            pr_info!("- Acknowledgment Number: {}", canfd_result.data_can.tcphdr.ack);

        // Improved output for TCP flags:
            pr_info!("- Flags: {:?}", canfd_result.data_can.tcphdr.flags); // Use Debug trait for detailed flag information

        // Additional TCP header fields (consider including only relevant ones):
    pr_info!("- Data Offset: {}", canfd_result.data_can.tcphdr.offset);
    pr_info!("- Window: {}", canfd_result.data_can.tcphdr.window);
    pr_info!("- Checksum: {:x}", canfd_result.data_can.tcphdr.checksum); // Hexadecimal format for checksum

    // Payload information:
    pr_info!("- Payload (first 16 bytes):");
    for i in 0..16 {
        if i < canfd_result.data_can.data_eth.len() {
            pr_info!("  - Byte {}: {:02X}", i, canfd_result.data_can.data_eth[i]);
        }   else {
            pr_info!("  - Byte {}: (no data)", i);
        }
    } */



    /*
    
        let canfd_result = canfd_ethpayload::deserialize_canfd_ethpayload(&vec48).unwrap();
 
        pr_info!("------------------------------------") ; 
         // Print CAN frame information
        pr_info!("CAN Frame Information:");
        pr_info!("- CAN ID: 0x{:0x}", canfd_result.can_id);
        pr_info!("- Length: {}", canfd_result.len);
        pr_info!("- Flags: 0x{:0x}", canfd_result.flags);

    // Access and print Ethload data
        pr_info!("\n  - Ethernet Payload:");
        pr_info!("- IP Header:");
        pr_info!("- Version: {:?}", canfd_result.data_can.iphdr.version);
        pr_info!("\n  - TCP Header:");
        pr_info!("- Source Port: {}", canfd_result.data_can.tcphdr.src_port);
        pr_info!("- Destination Port: {}", canfd_result.data_can.tcphdr.dst_port);
        pr_info!("- Sequence Number: {}", canfd_result.data_can.tcphdr.seq);
        pr_info!("- Acknowledgment Number: {}", canfd_result.data_can.tcphdr.ack);

        // Improved output for TCP flags:
        pr_info!("- Flags: {:?}", canfd_result.data_can.tcphdr.flags); // Use Debug trait for detailed flag information

        // Additional TCP header fields (consider including only relevant ones):
        pr_info!("- Data Offset: {}", canfd_result.data_can.tcphdr.offset);
        pr_info!("- Window: {}", canfd_result.data_can.tcphdr.window);
        pr_info!("- Checksum: {:x}", canfd_result.data_can.tcphdr.checksum); // Hexadecimal format for checksum 
*/

