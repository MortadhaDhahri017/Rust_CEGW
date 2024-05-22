
use bindings::canid_t;
use core::fmt::Formatter;
use core::fmt::Display;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use crate::delay::coarse_sleep ; 
use core::time::Duration;



#[allow(warnings)]

#[derive(PartialEq)]

pub struct EtherType(pub u16);

impl EtherType {
    pub const IPV4: EtherType = Self(0x0800);
    pub const IPV6: EtherType = Self(0x86dd);
    pub const ARP: EtherType = Self(0x0806);
    pub const WAKE_ON_LAN: EtherType = Self(0x0842);
    pub const VLAN_TAGGED_FRAME: EtherType = Self(0x8100);
    pub const PROVIDER_BRIDGING: EtherType = Self(0x88A8);
    pub const VLAN_DOUBLE_TAGGED_FRAME: EtherType = Self(0x9100);
}

#[derive(PartialEq)]
pub struct TCP_Frame {
    pub header: TcpHeader,
    pub data: [u8; 4],
}
#[derive(PartialEq)]
pub struct Ipv4Frame {
    pub header: Ipv4Header,
    pub data_tcp: TCP_Frame,
}

 impl Ipv4Frame {
        pub fn from_bytes(buffer: &[u8]) -> Option<Self> {
            // Minimum buffer size check (adjusted based on struct definitions)
            
            // Extract individual field slices using nom (assuming all fields are present)
            let (iphdr_bytes, rest) = buffer.split_at(21);
            let (tcphdr_bytes, data) = buffer.split_at( 21);
            let (data,_)=buffer.split_at(4) ; 
    
            // Deserialize individual fields
            let iphdr = Ipv4Header::deserialize_ip_header(iphdr_bytes)?;
            let tcphdr = TcpHeader::deserialize_tcp_header(tcphdr_bytes)?; // Assuming deserialize_tcp_header exists
            let mut data_arr = [0; 4];
            data_arr.copy_from_slice(data);
    
            // Return Ipv4Frame with data copy
            Some(Self {
                header: iphdr,
                data_tcp: {
                    TCP_Frame {
                        header: tcphdr,
                        data: data_arr,
                    }
                },
            })
        }
    }
    
#[derive(PartialEq)]
pub struct EthFrame {
    pub dst: [u8; 6],
    pub src: [u8; 6],
    pub ethertype: EtherType,
    pub data_ipv4: Ipv4Frame,
    pub fsc: u32,
}


impl EthFrame {
    pub fn get_ip_header(&self) -> Option<&Ipv4Header> {
        
        if self.ethertype == EtherType::IPV4 {
            Some(&self.data_ipv4.header)
        } else {
            None
        }
    }

    pub fn get_tcp_header(&self) -> Option<&TcpHeader> {
        
        if self.ethertype == EtherType::IPV4 {
            Some(&self.data_ipv4.data_tcp.header)
        } else {
            None
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        bytes.try_extend_from_slice(&self.dst).unwrap(); // unwrap added for error handling
        bytes.try_extend_from_slice(&self.src).unwrap(); // unwrap added for error handling
        bytes.try_extend_from_slice(&self.ethertype.0.to_be_bytes()).unwrap(); // unwrap added for error handling
        
        // Serialize the IP header using the standalone function
        bytes.try_extend_from_slice(&serialize_ip_header(&self.data_ipv4.header));
        
        // Serialize the TCP header using the standalone function
        bytes.try_extend_from_slice(&serialize_tcp_header(&self.data_ipv4.data_tcp.header));
        
        bytes.try_extend_from_slice(&self.data_ipv4.data_tcp.data).unwrap(); // Assuming data is already in byte form
        bytes.try_extend_from_slice(&self.fsc.to_be_bytes()).unwrap(); // unwrap added for error handling
        
        bytes
    }

  
    
        pub fn deserialize_ethernet(buffer:&[u8]) -> Option<EthFrame> {
            let (dst, rest) = buffer.split_at(6);
            let (src, rest) = rest.split_at(6);
            let (ethertype, rest) = rest.split_at(2);
            let (iphdr_bytes, rest) = rest.split_at(21);
            let (tcphdr_bytes, rest) = rest.split_at(21);
            let (data_eth, rest) = rest.split_at(4);
            let (fsc, _) = rest.split_at(4);
    
    
            let mut ethertype_arr = [0; 2];
            let mut fsc_arr = [0; 4] ; 
            ethertype_arr.copy_from_slice(ethertype);
            fsc_arr.copy_from_slice(fsc);
    
    
    
            let src_vec = src.to_owned();
            let src_v1=vec_to_array6(&src_vec) ;  
            let dst_vec = dst.to_owned();
            let dst_v1=vec_to_array6(&dst_vec) ; 
            let ethertype = u16::from_be_bytes(ethertype_arr);
            let iphdr = Ipv4Header::deserialize_ip_header(iphdr_bytes)?;
            let tcphdr = TcpHeader::deserialize_tcp_header(tcphdr_bytes)?;
            let data_eth_vec = data_eth.to_owned();
            let data_eth_v1=vec_to_array4(&data_eth_vec) ; 
            let fsc = u32::from_be_bytes(fsc_arr);
            Some(EthFrame { 
                dst: dst_v1.unwrap(),
                src: src_v1.unwrap(),
                ethertype: EtherType(ethertype),
                data_ipv4: Ipv4Frame{
                    header: iphdr,
                    data_tcp: TCP_Frame{
                        header: tcphdr,
                        data: data_eth_v1.unwrap(),
                    },
                },
                fsc: fsc,
                 })
     }
}
  
    fn vec_to_array6(slice_data: &[u8]) -> Result<[u8; 6], &'static str> {
        // Check if the slice has exactly 4 elements
        if slice_data.len() != 6 {
            return Err("Error: Slice must contain exactly 6 elements");
        }
    
        // Safely convert the slice to a fixed-size array
        unsafe {
            // This is safe because we checked the slice length beforehand
            let array: [u8; 6] = *(slice_data.as_ptr() as *const [u8; 6]);
            Ok(array)
        }
    }

    fn vec_to_array4(slice_data: &[u8]) -> Result<[u8; 4], &'static str> {
        // Check if the slice has exactly 4 elements
        if slice_data.len() != 4 {
            return Err("Error: Slice must contain exactly 6 elements");
        }
    
        // Safely convert the slice to a fixed-size array
        unsafe {
            // This is safe because we checked the slice length beforehand
            let array: [u8; 4] = *(slice_data.as_ptr() as *const [u8; 4]);
            Ok(array)
        }
    }

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Ipv4Header {
    pub version: u8, // 0x04
    pub len: u8,
    pub ToS: u8,
    pub total_len: u16,
    pub id: u16, //
    pub flags: u8, //3b
    pub frag_offset: u8, //13b
    pub ttl: u8,  //
    pub protocol: u8, // 0x06,   tcp-->6 u8
    pub checksum: u16, //
    pub src: [u8; 4], //
    pub dst: [u8; 4], //
}

impl Ipv4Header {
    pub fn serialized_size(&self)->usize{
        let size=1+1+1+2+2+1+1+1+1+2+4+4;
        size
    }
    
}


  

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct TcpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub offset: u8, //4b
    pub reserved: u8, //4b
    pub flags: u8,
    pub window: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
}


impl TcpHeader {
  fn serialized_size(&self) -> usize {
    // Fixed-size fields
    let fixed_size = 2 + 2 + 4 + 4 + 1 + 1 + 1 + 2 + 2 + 2; // src_port, dst_port, seq, ack, offset, reserved, flags, window, checksum, urgent_ptr

    fixed_size
  }
}


#[derive(PartialEq)]
pub struct Ethload {
   pub iphdr : Ipv4Header , 
   pub tcphdr : TcpHeader , 
   pub data_eth : [u8;4]
}

impl Ethload {
    fn serialized_size(&self) -> usize {
      // Sizes of fixed-size fields
      let iphdr_size = self.iphdr.serialized_size();
      let tcphdr_size = self.tcphdr.serialized_size();
      let data_eth_size = 4;  // Size of data_eth (fixed-size array)
  
      // Total size is the sum of individual field sizes
      (iphdr_size + tcphdr_size + data_eth_size).into()
    }
  }
  

#[derive(PartialEq)]
pub struct canfd_ethpayload {
    pub can_id: canid_t,
    pub len: u8,
    pub flags: u8,
    pub data_can :Ethload 
}

impl canfd_ethpayload {
    pub fn from_eth_frame(frame: &EthFrame) -> Self {
      let ip_header = frame.data_ipv4.header ; 
      let tcp_header = frame.data_ipv4.data_tcp.header ; 
  
      // Assuming data.len() is at least the size of the copied data (here, 4 bytes)
      let mut data_slice = [0; 4]; // Initialize data with zeroes
      data_slice.copy_from_slice(&frame.data_ipv4.data_tcp.data[..4]); // Copy first 4 bytes of TCP data
  
      // Calculate the total length of the payload: IP header size + TCP header size + data size
      let payload_len = ip_header.len as u8 + tcp_header.offset * 4 + data_slice.len() as u8;
      


      canfd_ethpayload {
        can_id: 0, // Set default value for CAN ID
        len: payload_len,
        flags: 0, // Set default value for flags (you can define flags if needed)
        data_can: Ethload {
          iphdr: ip_header,
          tcphdr: tcp_header,
          data_eth: data_slice,
        },
      }
    }
    pub fn serialized_size(&self) -> usize {
        4 + 1 + 1 + self.data_can.serialized_size() // can_id, len, flags, data_can size
      }

}
impl core::fmt::Display for canfd_ethpayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
      write!(f, "can_id: 0x{:0x}\n", self.can_id)?;
      write!(f, "len: {}\n", self.len)?;
      write!(f, "flags: 0x{:0x}\n", self.flags)?;
      write!(f, "data:\n")?;
      write!(f, "  - IP header:\n")?;
      write!(f, "    - version: {:?}\n", self.data_can.iphdr.version)?;
      // ... Add formatting for other IP header fields
      write!(f, "  - TCP header:\n")?;
      write!(f, "    - src_port: {}\n", self.data_can.tcphdr.src_port)?;
      // ... Add formatting for other TCP header fields
      write!(f, "  - TCP data: {:?}\n", self.data_can.data_eth)?;
      Ok(())
    }
  }


pub fn serialize_canfd_ethpayload(payload: &canfd_ethpayload) -> Vec<u8> {
    let mut serialized_data = Vec::new();

    // Serialize can_id (u32)
    serialized_data.try_extend_from_slice(&payload.can_id.to_be_bytes());

    // Serialize len (u8)
    serialized_data.try_push(payload.len);

    // Serialize flags (u8)
    serialized_data.try_push(payload.flags);

    // Serialize data_can (Ethload)
    serialized_data.try_extend_from_slice(&serialize_ethload(&payload.data_can));

 


    serialized_data
}

fn serialize_ethload(ethload: &Ethload) -> Vec<u8> {
    let mut serialized_data = Vec::new();

    // Serialize iphdr (Ipv4Header)
    serialized_data.try_extend_from_slice(&serialize_ip_header(&ethload.iphdr));

    // Serialize tcphdr (TcpHeader)
    serialized_data.try_extend_from_slice(&serialize_tcp_header(&ethload.tcphdr));

    // Serialize data_eth (slice of u8)
    serialized_data.try_extend_from_slice(&ethload.data_eth);

  

    serialized_data
}

fn serialize_ip_header(ip_header: &Ipv4Header) -> Vec<u8> {
    let mut serialized_data = Vec::new();

    // Serialize each field of Ipv4Header (u8, u16, etc.) using to_be_bytes()
    serialized_data.try_push(ip_header.version);
    serialized_data.try_push(ip_header.len);
    serialized_data.try_push(ip_header.ToS);
    serialized_data.try_extend_from_slice(&ip_header.total_len.to_be_bytes());
    serialized_data.try_extend_from_slice(&ip_header.id.to_be_bytes());
    serialized_data.try_push(ip_header.flags);
    serialized_data.try_push(ip_header.frag_offset);
    serialized_data.try_push(ip_header.ttl);
    serialized_data.try_push(ip_header.protocol);
    serialized_data.try_extend_from_slice(&ip_header.checksum.to_be_bytes());
    serialized_data.try_extend_from_slice(&ip_header.src);
    serialized_data.try_extend_from_slice(&ip_header.dst);

  
    serialized_data
}

fn serialize_tcp_header(tcp_header: &TcpHeader) -> Vec<u8> {
    let mut serialized_data = Vec::new();

    // Serialize each field of TcpHeader (u16, u32, etc.) using to_be_bytes()
    serialized_data.try_extend_from_slice(&tcp_header.src_port.to_be_bytes());
    serialized_data.try_extend_from_slice(&tcp_header.dst_port.to_be_bytes());
    serialized_data.try_extend_from_slice(&tcp_header.seq.to_be_bytes());
    serialized_data.try_extend_from_slice(&tcp_header.ack.to_be_bytes());
    serialized_data.try_push(tcp_header.offset); // u8, no need for to_be_bytes()
    serialized_data.try_push(tcp_header.reserved); // u8, no need for to_be_bytes()
    serialized_data.try_push(tcp_header.flags); // u8, no need for to_be_bytes()
    serialized_data.try_extend_from_slice(&tcp_header.window.to_be_bytes());
    serialized_data.try_extend_from_slice(&tcp_header.checksum.to_be_bytes());
    serialized_data.try_extend_from_slice(&tcp_header.urgent_ptr.to_be_bytes());



    serialized_data
}

/*#[derive(PartialEq)]
pub struct canfd_ethpayload {
    pub can_id: canid_t,
    pub len: u8,
    pub flags: u8,
    pub data_can :Ethload 
} */

impl canfd_ethpayload {

    pub fn deserialize_canfd_ethpayload(buffer: &[u8]) -> Option<canfd_ethpayload> {
        // Check if the buffer has enough data for all fields
        /*if buffer.len() < mem::size_of::<Ipv4Header>() + mem::size_of::<TcpHeader>() + mem::size_of::<[u8; 4]>() {
            return None;
        }*/

        // Extract individual field slices
        let (can_id, rest) = buffer.split_at(4);
        let (len, rest) = rest.split_at(1);
        let (flags,rest)=rest.split_at(1) ; 
        let (data_can_bytes, _) = rest.split_at(46);

        let mut can_arr = [0; 4];
        can_arr.copy_from_slice(can_id);

        // Deserialize individual fields
        let data_can = Ethload::from_bytes(data_can_bytes)?;
     
        let can_id = u32::from_be_bytes(can_arr);

   

        Some(canfd_ethpayload {
            can_id ,
            len:len[0],
            flags:flags[0],
            data_can,
        })
    }
}

impl Ethload {
    pub fn from_bytes(buffer: &[u8]) -> Option<Ethload> {
        // Check if the buffer has enough data for all fields
        /*if buffer.len() < mem::size_of::<Ipv4Header>() + mem::size_of::<TcpHeader>() + mem::size_of::<[u8; 4]>() {
            return None;
        }*/

        let (iphdr_bytes, rest) = buffer.split_at(21);
        let (tcphdr_bytes, rest) = rest.split_at(21);
        let (data_eth, _) = rest.split_at(4);

        // Deserialize individual fields
        let iphdr = Ipv4Header::deserialize_ip_header(iphdr_bytes)?;
        let tcphdr = TcpHeader::deserialize_tcp_header(tcphdr_bytes)?;
        let data_eth_vec = data_eth.to_owned();
        let data_eth_v1=vec_to_array(data_eth_vec) ; 
        
  
        Some(Ethload {
            iphdr,
            tcphdr,
            data_eth:data_eth_v1.unwrap(),
        })
    }
}

impl Ipv4Header {
pub fn deserialize_ip_header(buffer: &[u8]) -> Option<Ipv4Header> {
        // Check if the buffer has enough data (expected size)
        /*if buffer.len() < mem::size_of::<Ipv4Header>() {
            return None;
        }*/
    
        // Extract individual field values
        let (version, rest) = buffer.split_at(1);
        let (len, rest) = rest.split_at(1);
        let (tos, rest) = rest.split_at(1);
        let (total_len, rest) = rest.split_at(2);
        let (id, rest) = rest.split_at(2);
        let (flags, rest) = rest.split_at(1);
        let (frag_offset, rest) = rest.split_at(1);
        let (ttl, rest) = rest.split_at(1);
        let (protocol, rest) = rest.split_at(1);
        let (checksum, rest) = rest.split_at(2);
        let (src,rest) = rest.split_at(4);
        let (dst,_)=rest.split_at(4) ; 

        let mut total_len_arr = [0; 2];
        total_len_arr.copy_from_slice(total_len);
        let mut id_arr = [0; 2];
        id_arr.copy_from_slice(id);
        let mut checksum_arr = [0; 2];
        checksum_arr.copy_from_slice(checksum);

        let src_vec = src.to_owned();
        let src_v1=vec_to_array(src_vec) ;  
        let dst_vec = dst.to_owned();
        let dst_v1=vec_to_array(dst_vec) ; 

        // Convert slices to fixed-size values (avoid unnecessary copies)
        let total_len = u16::from_be_bytes(total_len_arr);
        let checksum = u16::from_be_bytes(checksum_arr) ; 
        let id = u16::from_be_bytes(id_arr);
        let ttl = ttl[0]; // Single byte for TTL
   
    

        // Create the Ipv4Header struct
        Some(Ipv4Header {
            version: version[0],
            len: len[0],
            ToS: tos[0],
            total_len,
            id,
            flags: flags[0],
            frag_offset: frag_offset[0],
            ttl,
            protocol: protocol[0],
            checksum,
            src:src_v1.unwrap(),
            dst:dst_v1.unwrap(),
        })
    }
}
impl TcpHeader {
    pub fn deserialize_tcp_header(buffer: &[u8]) -> Option<TcpHeader> {

        // Extract individual field slices
        let (src_port, rest) = buffer.split_at(2);
        let (dst_port, rest) = rest.split_at(2);
        let (seq, rest) = rest.split_at(4);
        let (ack, rest) = rest.split_at(4);
        let (offset, rest) = rest.split_at(1);
        let (reserved,rest) =rest.split_at(1) ; 
        let (flags, rest) = rest.split_at(1);
        let (window, rest) = rest.split_at(2);
        let (checksum, rest) = rest.split_at(2);
        let (urgent_ptr, _) = rest.split_at(2);
    
        // Convert slices to fixed-size arrays before conversion
        let mut src_port_arr = [0; 2];
        src_port_arr.copy_from_slice(src_port);
        let mut dst_port_arr = [0; 2];
        dst_port_arr.copy_from_slice(dst_port);
        let mut seq_arr = [0; 4];
        seq_arr.copy_from_slice(seq);
        let mut ack_arr = [0; 4];
        ack_arr.copy_from_slice(ack);
        let mut window_arr = [0; 2];
        window_arr.copy_from_slice(window);
        let mut checksum_arr = [0; 2];
        checksum_arr.copy_from_slice(checksum);
        let mut urgent_ptr_arr = [0; 2];
        urgent_ptr_arr.copy_from_slice(urgent_ptr);
    
        // Extract individual fields and convert from network byte order
        let src_port = u16::from_be_bytes(src_port_arr);
        let dst_port = u16::from_be_bytes(dst_port_arr);
        let seq = u32::from_be_bytes(seq_arr);
        let ack = u32::from_be_bytes(ack_arr);
        let offset = (offset[0] >> 4) & 0x0F;
        let reserved = offset & 0x0F;
        let flags = flags[0];
        let window = u16::from_be_bytes(window_arr);
        let checksum = u16::from_be_bytes(checksum_arr);
        let urgent_ptr = u16::from_be_bytes(urgent_ptr_arr);
 
        // Create the TcpHeader struct
        Some(TcpHeader{
            src_port,
            dst_port,
            seq,
            ack,
            offset,
            reserved,
            flags,
            window,
            checksum,
            urgent_ptr,
        })
    }
}    


fn vec_to_array(slice_data: &[u8]) -> Result<[u8; 4], &'static str> {
    // Check if the slice has exactly 4 elements
    if slice_data.len() != 4 {
        return Err("Error: Slice must contain exactly 4 elements");
    }

    // Safely convert the slice to a fixed-size array
    unsafe {
        // This is safe because we checked the slice length beforehand
        let array: [u8; 4] = *(slice_data.as_ptr() as *const [u8; 4]);
        Ok(array)
    }
}
