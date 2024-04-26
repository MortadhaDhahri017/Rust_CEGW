

use crate::bindings::*;
use crate::vdev::NetDevice ; 



///Overrides rust's ethhdr struct in C

pub struct Ethhdr(*mut bindings::ethhdr) ; 

/// Overrides rust's SkBuff struct for our own module
pub struct SkBuff(*mut bindings::sk_buff);
impl SkBuff {

    /// as_ptr returns a raw pointer to sk_buff 
    /// # Safety
    ///
    /// The caller must ensure that `self.0` is a valid and properly-aligned raw
    /// pointer to a `sk_buff` struct in C.
    ///
    /// The caller is responsible for ensuring that the pointer is used safely.
    /// Dereferencing an invalid pointer can cause undefined behavior.

    pub unsafe fn as_ptr(&self) -> *mut bindings::sk_buff {
        self.0
    }
    
    /// nla_put_u32 - Add a u32 netlink attribute to a socket buffer
    /// @skb: socket buffer to add attribute to
    /// @attrtype: attribute type
    /// @value: numeric value

    pub fn nla_put_u32(&self, attrtype: core::ffi::c_int, value: u32_,) -> core::ffi::c_int {
        unsafe { bindings::nla_put_u32(self.0, attrtype, value) }
    }

    /// nla_put_u8 - Add a u8 netlink attribute to a socket buffer
    /// @skb: socket buffer to add attribute to
    /// @attrtype: attribute type
    /// @value: numeric value

    pub fn nla_put_u8(&self, attrtype: core::ffi::c_int, value: u8_,) -> core::ffi::c_int {
            
        unsafe { bindings::nla_put_u8(self.0, attrtype, value) }
    }
        
    /// nla_put_string - Add a string netlink attribute to a socket buffer
    /// @skb: socket buffer to add attribute to
    /// @attrtype: attribute type
    /// @str_: numeric value


    pub fn nla_put_string(&self, attrtype: core::ffi::c_int, str_: *const core::ffi::c_char,) -> core::ffi::c_int {
        unsafe { bindings::nla_put_string(self.0, attrtype, unsafe{str_}) }
    }



    /// nlmsg_new - Allocate a new netlink message
    /// @payload: size of the message payload
    /// @flags: the type of memory to allocate.
    /// Use NLMSG_DEFAULT_SIZE if the size of the payload isn't known
    /// and a good default is needed.
 
    pub fn nlmsg_new(payload: usize, flags: gfp_t) -> Self {
        unsafe { Self(bindings::nlmsg_new(payload, flags)) }
    }



    /// nlmsg_put - Add a new netlink message to an skb
    /// @skb: socket buffer to store message in
    /// @portid: netlink PORTID of requesting application
    /// @seq: sequence number of message
    /// @type: message type
    /// @payload: length of message payload
    /// @flags: message flags
    ///
    ///Returns NULL if the tailroom of the skb is insufficient to store
    ///the message header and payload.
 

    pub fn nlmsg_put(&self,portid: u32_,seq: u32_,type_: core::ffi::c_int, payload: core::ffi::c_int, flags: core::ffi::c_int,) -> Nlmsghdr {
        unsafe { Nlmsghdr(bindings::nlmsg_put(self.0, portid, seq, type_, payload, flags)) }
    }
    
    
    /// nlmsg_end - Finalize a netlink message
    /// @skb: socket buffer the message is stored in
    /// @nlh: netlink message header
    ///
    /// Corrects the netlink message header to include the appeneded
    /// attributes. Only necessary if attributes have been added to
    /// the message.
  
    pub fn nlmsg_end(&self, nlh: Nlmsghdr) {
        unsafe { bindings::nlmsg_end(self.0, nlh.0) }
    }

    /// genlmsg_new - Allocate a new generic netlink message
    /// @payload: size of the message payload
    /// @flags: the type of memory to allocate. 

    pub fn genlmsg_new(payload: usize, flags: gfp_t) -> SkBuff{
        unsafe { SkBuff(bindings::genlmsg_new(payload, flags)) }
    }   

    /// genlmsg_end - Finalize a generic netlink message
    /// @skb: socket buffer the message is stored in
    /// @hdr: user specific header
    ///

    pub fn genlmsg_end(&self,hdr : *mut core::ffi::c_void ) {
        unsafe { bindings::genlmsg_end(self.0, unsafe{hdr}) }  
    }

    ///   The function skb_set_transport_header is a part of
    ///   the Linux kernel's socket buffer (skb) API.
    ///   It is used to set the transport header of a socket buffer.

    pub fn skb_set_transport_header(&self , _offset :  core::ffi::c_int) {
        unsafe {
            //self.skb_set_transport_header(_offset, _)
            bindings::skb_set_transport_header(self.0 ,_offset)
        }
    } 

    /// skb_set_network_header
    /// Sets the network header of a `sk_buff` struct.
    ///
    /// This function is a helper function for `rust_helper_skb_set_network_header` that contains the actual implementation.
    /// It sets the `network_header` field of the `sk_buff` struct to the given value, and then copies the first 14 bytes of the struct to the new location of the network header.
    ///
    /// Note that this function assumes that the `sk_buff` struct has a particular layout and that the network header is located at a fixed _offset.
    /// In practice, you should use the `sk_buff` API provided by the Linux kernel to safely manipulate `sk_buff` structs.
    ///
    /// # Safety
    ///
    /// This function takes a raw pointer to a `sk_buff` struct, which can be dangerous if the pointer is invalid or dangling.
    /// You should ensure that the pointer is valid and points to a properly initialized `sk_buff` struct before calling this function.
    ///
    /// # Arguments
    ///
    /// * `skb` - A raw pointer to a `sk_buff` struct.
    /// * `_offset` - The value to set the network header to.

    pub fn skb_set_network_header(&self , _offset :  core::ffi::c_int) {
        unsafe {
            //self.skb_set_network_header(_offset, _)
            bindings::skb_set_network_header(self.0 ,_offset)
        }
    } 

    /// skb_set_mac_header
    /// Sets the MAC header of a `sk_buff` struct.
    ///
    /// This function is a wrapper around the `skb_set_mac_header` function from the Linux kernel's networking stack.
    /// It sets the `mac_header` field of the `sk_buff` struct to the given value, and then updates the length of the packet.
    ///
    /// Note that this function assumes that the `sk_buff` struct has a particular layout and that the MAC header is located at a fixed _offset.
    /// In practice, you should use the `sk_buff` API provided by the Linux kernel to safely manipulate `sk_buff` structs.
    ///
    /// # Safety
    ///
    /// This function takes a raw pointer to a `sk_buff` struct, which can be dangerous if the pointer is invalid or dangling.
    /// You should ensure that the pointer is valid and points to a properly initialized `sk_buff` struct before calling this function.
    ///
    /// # Arguments
    ///
    /// * `skb` - A raw pointer to a `sk_buff` struct.
    /// * `value` - The value to set the MAC header to.
    
    pub fn skb_set_mac_header(&self , _offset :  core::ffi::c_int) {
        unsafe {
            //self.skb_set_mac_header(_offset, _)
            bindings::skb_set_mac_header(self.0 ,_offset)
        }
    } 

    ///    skb_reserve - adjust headroom
    ///    @skb: buffer to alter
    ///    @len: bytes to move
    ///
    ///    Increase the headroom of an empty &sk_buff by reducing the tail
    ///    room. This is only allowed for an empty buffer.
    
    pub fn skb_reserve(&self , len: i32) {
        unsafe {
            bindings::skb_reserve(self.0,len)
        }
    }


    /// skb_network_header
    /// The function calculates the address by adding the network_header field's value to the head field's address.
    /// The network_header field contains the _offset of the network header from the start of the socket buffer, 
    /// and the head field points to the start of the socket buffer's data area.
    /// This function is useful when you need to directly access or manipulate 
    /// the network header of a packet stored in a socket buffer.


    pub fn skb_network_header(&self) ->*mut core::ffi::c_uchar{
        unsafe {
            bindings::skb_network_header(self.0)
        }
    }

    /// skb_tail_pointer 
    /// Returns a pointer to the tail of the socket buffer (sk_buff)
    /// The tail pointer marks the end of the valid data in the socket buffer

    pub fn skb_tail_pointer(&self)->*mut core::ffi::c_uchar{
        unsafe {
            bindings::skb_tail_pointer(self.0)
        }
    }
    
    ///	skb_push - add data to the start of a buffer
    ///	@skb: buffer to use
    ///	@len: amount of data to add
    ///
    ///	This function extends the used data area of the buffer at the buffer
    ///	start. If this would exceed the total buffer headroom the kernel will
    ///	panic. A pointer to the first byte of the extra data is returned.
    
    pub fn skb_push(&self, len: core::ffi::c_uint) -> *mut core::ffi::c_void{
        unsafe {
            bindings::skb_push(self.0 ,len)
        }
    }

    ///
    ///	skb_copy_expand	-	copy and expand sk_buff
    ///	@skb: buffer to copy
    ///	@newheadroom: new free bytes at head
    ///	@newtailroom: new free bytes at tail
    ///	@gfp_mask: allocation priority
    ///
    ///	Make a copy of both an &sk_buff and its data and while doing so
    ///	allocate additional space.
    ///
    ///	This is used when the caller wishes to modify the data and needs a
    ///	private copy of the data to alter as well as more space for new fields.
    ///	Returns %NULL on failure or the pointer to the buffer
    ///	on success. The returned buffer has a reference count of 1.
    ///
    ///	You must pass %GFP_ATOMIC as the allocation priority if this function
    ///	is called from an interrupt.

    pub fn skb_copy_expand(
        skb: *const sk_buff,
        newheadroom: core::ffi::c_int,
        newtailroom: core::ffi::c_int,
        priority: gfp_t,
    ) -> SkBuff {
        unsafe {
            SkBuff(bindings::skb_copy_expand(skb,newheadroom,newtailroom,priority)) 
        }
    }

    /// No docs as for now 

    pub fn ip_hdrlen(&self)->u32 {
        unsafe {
            bindings::ip_hdrlen(self.0) 
        }
    }

    //// No docs as for now 
    pub fn eth_hdr(&self)->Ethhdr {
        unsafe{
            Ethhdr(bindings::eth_hdr(self.0))
        }
    }


}

/// Sock is a struct that wraps C's sruct sock using
/// bindings::sock

pub struct Sock(*mut bindings::sock); 
impl Sock {


    /// nlmsg_unicast - unicast a netlink message
    /// @sk: netlink socket to spread message to
    /// @skb: netlink message as socket buffer
    /// @portid: netlink portid of the destination socket

    pub fn nlmsg_unicast(&self, skb: SkBuff, portid: u32_) -> core::ffi::c_int {
        unsafe { bindings::nlmsg_unicast(self.0, skb.0, portid) }
    }

}

/// Nlmsghdr is a struct that wraps C's nlmsghdr

pub struct Nlmsghdr(*mut bindings::nlmsghdr);
impl Nlmsghdr {

    /// new is a function to create a new Nlmsghdr
    pub fn new(nlh: *mut bindings::nlmsghdr) -> Self {
        Self(nlh)
    }
}


/// Nlattr wraps C's nlattr struct  
/// 
pub struct Nlattr(*mut bindings::nlattr);

impl Nlattr {

    ///  nla_data - head of payload
    /// @nla: netlink attribute
    /// 
    pub fn nla_data(&self)-> *mut core::ffi::c_void{
        unsafe { bindings::nla_data(self.0) }
}
}
