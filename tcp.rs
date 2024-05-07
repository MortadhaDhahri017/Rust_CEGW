
use bindings::canid_t;
use core::fmt::Formatter;
use core::fmt::Display;
use alloc::vec::Vec;

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
    pub data: [u8; 6],
}
#[derive(PartialEq)]
pub struct Ipv4Frame {
    pub header: Ipv4Header,
    pub data_tcp: TCP_Frame,
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
    fn serialized_size(&self) -> usize {
      // Fixed-size fields
      let fixed_size = 1 + 3 + 2 + 2 + 1 + 1 + 1 + 1 + 2 + 8; // Version, len, ToS, total_len, id, flags, frag_offset, ttl, protocol, checksum, src, dst
  
      // Header length calculation (assuming len includes options)
      let header_length = (self.len & 0xF) * 4; // Extract lower 4 bits of len (assuming len includes options)
  
      (fixed_size + header_length).into()
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

    // Variable size based on options (data offset field)
    let options_size = match (self.offset & 0xF) << 2 {
      0 => 0, // No options
      x => x, // Option length is encoded in the lower 4 bits of offset, multiplied by 4
    };

    (fixed_size + options_size).into()
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
      let ip_header = frame.get_ip_header().unwrap_or_else(|| panic!("Frame does not contain an IPv4 header"));
      let tcp_header = frame.get_tcp_header().unwrap_or_else(|| panic!("Frame does not contain a TCP header"));
  
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
          iphdr: *ip_header,
          tcphdr: *tcp_header,
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

/* 
  fn main() {
    let mock_frame = EthFrame {
    dst: [0x00, 0x0C, 0x29, 0x70, 0xAC, 0x1E],  // Destination MAC address (example)
    src: [0x00, 0x5E, 0x70, 0x01, 0x02, 0x03],  // Source MAC address (example)
    ethertype: EtherType::IPV4,
    data: Ipv4Frame {
        header: Ipv4Header {
            version: 4,
            len: 20,
            ToS: 0,
            total_len: 100,
            id: 10,
            flags: 0,
            frag_offset: 0,
            ttl: 64,
            protocol: 6,  // TCP protocol
            checksum: 0xABCD,
            src: [192, 168, 1, 10],  // Source IP address (example)
            dst: [10, 0, 0, 1],       // Destination IP address (example)
        },
        data: TCP_Frame {
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
            data: [0xCA, 0xFE, 0xBA, 0xBE], // Example TCP data (first 4 bytes)
        },
    },
    fsc: 0xDEADBEEF,
};
// Test fmt function
  let can_payload = canfd_ethpayload::from_eth_frame(&mock_frame);
  println!("{}", can_payload);

  // Test from_eth_frame function
  let another_payload = canfd_ethpayload::from_eth_frame(&mock_frame);
  assert_eq!(another_payload.can_id, 0);
  assert_eq!(another_payload.len, 54); // 20 (IP header) + 20 (TCP header) + 4 (TCP data)
  assert_eq!(another_payload.flags, 0);

}*/
fn serialize_canfd_ethpayload(payload: &canfd_ethpayload) -> Vec<u8> {
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
/*  
fn deserialize_canfd_ethpayload(data: &[u8]) -> Result<canfd_ethpayload, &'static str> {
    if data.len() < 13 {
        return Err("Byte stream too short for canfd_ethpayload");
    }

    let mut cursor = 0;

    // Deserialize can_id (u32)
    let can_id = u32::from_be_bytes(&data[cursor..cursor + 4]);
    cursor += 4;

    // Deserialize len (u8)
    let len = data[cursor];
    cursor += 1;

    // Deserialize flags (u8)
    let flags = data[cursor];
    cursor += 1;

    // Deserialize data_can (Ethload)
    let ethload = deserialize_ethload(&data[cursor..])?;
    cursor += ethload.serialized_size(); // Update cursor after deserializing Ethload

    Ok(canfd_ethpayload {
        can_id,
        len,
        flags,
        data_can: ethload, // Use data_can instead of data
    })
}
*/

fn deserialize_ethload(data: &[u8]) -> Result<Ethload, &'static str> {
    if data.len() < 20 {
        return Err("Byte stream too short for Ethload");
    }

    let mut cursor = 0;

    let iphdr = deserialize_ip_header(&data[cursor..])?;
    cursor += iphdr.serialized_size(); // Assuming serialized_size is implemented

    let tcphdr = deserialize_tcp_header(&data[cursor..])?;
    cursor += tcphdr.serialized_size(); // Assuming serialized_size is implemented

    let data_eth_mock = &data[cursor..cursor + 4];
    cursor += 4;
    let data_eth = if data_eth_mock.len() >= 4 {
        [data_eth_mock[0], data_eth_mock[1], data_eth_mock[2], data_eth_mock[3]]
      } else {
        [0; 4] // Create an array with 4 zeroes if data_eth_mock is less than 4 bytes
      };

    Ok(Ethload {
        iphdr,
        tcphdr,
        data_eth:data_eth,
    })
}


fn deserialize_ip_header(data: &[u8]) -> Result<Ipv4Header, &'static str> {
    if data.len() < 20 {
        return Err("Byte stream too short for Ipv4Header");
    }

    let mut cursor = 0;

    let version = data[cursor];
    cursor += 1;

    let len = data[cursor];
    cursor += 1;

    let ToS = data[cursor];
    cursor += 1;

    // Fix for total_len, id, checksum:
    let total_len_arr = slice_to_array_2(&data[cursor..cursor + 2])?;
    let total_len = u16::from_be_bytes(total_len_arr);
    cursor += 2;

    let id_arr = slice_to_array_2(&data[cursor..cursor + 2])?;
    let id = u16::from_be_bytes(id_arr);
    cursor += 2;

    let checksum_arr = slice_to_array_2(&data[cursor..cursor + 2])?;
    let checksum = u16::from_be_bytes(checksum_arr);
    cursor += 2;

    let flags = data[cursor];
    cursor += 1;

    let frag_offset = data[cursor] & 0b01111111; // Mask to get only fragment offset bits
    cursor += 1;

    let ttl = data[cursor];
    cursor += 1;

    let protocol = data[cursor];
    cursor += 1;

    let src = slice_to_array_4(&data[cursor..cursor + 4])?;
    cursor += 4;

    let dst = slice_to_array_4(&data[cursor..cursor + 4])?;
    cursor += 4;
    

    Ok(Ipv4Header {
        version,
        len,
        ToS,
        total_len,
        id,
        flags,
        frag_offset,
        ttl,
        protocol,
        checksum,
        src: src,
        dst: dst ,
    })
}

fn deserialize_tcp_header(data: &[u8]) -> Result<TcpHeader, &'static str> {
    if data.len() < 20 {
        return Err("Byte stream too short for TcpHeader");
    }

    let mut cursor = 0;

    let src_port = u16::from_be_bytes(u8_to_array_2_with_padding(&data[cursor])) << 8 | data[cursor + 1]as u16;
    cursor += 2;

    let dst_port = u16::from_be_bytes(u8_to_array_2_with_padding(&data[cursor])) << 8 | data[cursor + 1] as u16;
    cursor += 2;

    let seq_arr = slice_to_array_4(&data[cursor..cursor + 4])?;
    let seq = u32::from_be_bytes(seq_arr);
    cursor += 4;

    let ack_arr = slice_to_array_4(&data[cursor..cursor + 4])?; 
    let ack = u32::from_be_bytes(ack_arr);
    cursor += 4;

    let offset = data[cursor]; // u8, no need for from_be_bytes()
    cursor += 1;

    let reserved = data[cursor]; // u8, no need for from_be_bytes()
    cursor += 1;

    let flags = data[cursor]; // u8, no need for from_be_bytes()
    cursor += 1;

    let window = u16::from_be_bytes(u8_to_array_2_with_padding(&data[cursor])) << 8 | data[cursor + 1] as u16;
    cursor += 2;

    let checksum = u16::from_be_bytes(u8_to_array_2_with_padding(&data[cursor])) << 8 | data[cursor + 1] as u16;
    cursor += 2;

    let urgent_ptr = u16::from_be_bytes(u8_to_array_2_with_padding(&data[cursor])) << 8 | data[cursor + 1] as u16;
    cursor += 2;

    Ok(TcpHeader {
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





fn slice_to_array_2(data: &[u8]) -> Result<[u8; 2], &'static str> {
    if data.len() < 2 {
        return Err("Slice must have at least 2 elements");
    }

    // Safe because the check above ensures enough elements
    let mut arr = [0; 2];
    arr.copy_from_slice(&data[..2]);
    Ok(arr)
}

fn slice_to_array_4(data: &[u8]) -> Result<[u8; 4], &'static str> {
    if data.len() < 2 {
        return Err("Slice must have at least 4 elements");
    }

    // Safe because the check above ensures enough elements
    let mut arr = [0; 4];
    arr.copy_from_slice(&data[..4]);
    Ok(arr)
}

fn u8_to_array_2_with_padding(value: &u8) -> [u8; 2] {
    let mut arr = [0; 2];
    arr[0] = *value; // Copy the byte value
    arr
  }
  fn u8_to_array_4_with_padding(value: &u8) -> [u8; 4] {
    let mut arr = [0; 4];
    arr[0] = *value; // Copy the byte value
    arr
  }
