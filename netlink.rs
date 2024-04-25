use crate::bindings::*;



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




