

impl EthFrame {
  
    
    pub fn deserialize_eth_frame(buffer:&[u8]) -> Option<EthFrame> {
        let (dst, rest) = buffer.split_at(6);
        let (src, rest) = rest.split_at(6);
        let (ethertype, rest) = rest.split_at(2);
        let (iphdr_bytes, rest) = rest.split_at(21);
        let (tcphdr_bytes, rest) = rest.split_at(21);
        let (data_eth, rest) = rest.split_at(6);
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
        let data_eth_v1=vec_to_array6(&data_eth_vec) ; 
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
