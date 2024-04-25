use bindings::*;
use crate::netlink::SkBuff;

/// 1.The Netlink subsystem which serves as the underlying transport layer for all of the Generic Netlink communications.
/// 2.The Generic Netlink bus which is implemented inside the kernel, but which is available to userspace through the socket API and inside the kernel via the normal Netlink and Generic Netlink APIs.
/// 3.The Generic Netlink users who communicate with each other over the Generic Netlink bus; users can exist both in kernel and user space.
/// 4.The Generic Netlink controller which is part of the kernel and is responsible for dynamically allocating Generic Netlink communication channels and other management tasks. The Generic Netlink controller is implemented as a standard Generic Netlink user, however, it listens on a special, pre-allocated Generic Netlink channel.
/// 5.The kernel socket API. Generic Netlink sockets are created with the PF_NETLINK domain and the NETLINK_GENERIC protocol values.

/* 
 +---------------------+      +---------------------+
 | (3) application "A" |      | (3) application "B" |
 +------+--------------+      +--------------+------+
        |                                    |
        \                                    /
         \                                  /
          |                                |
  +-------+--------------------------------+-------+
  |        :                               :       |   user-space
=====+        :   (5)  kernel socket API      :       +================
  |        :                               :       |   kernel-space
  +--------+-------------------------------+-------+
           |                               |
     +-----+-------------------------------+----+
     |        (1)  Netlink subsystem            |
     +---------------------+--------------------+
                           |
     +---------------------+--------------------+
     |       (2) Generic Netlink bus            |
     +--+--------------------------+-------+----+
        |                          |       |
+-------+---------+                |       |
|  (4) controller |               /         \
+-----------------+              /           \
                                 |           |
              +------------------+--+     +--+------------------+
              | (3) kernel user "X" |     | (3) kernel user "Y" |
              +---------------------+     +---------------------+
 

*/


/* 
Message Format

Generic Netlink uses the standard Netlink subsystem as a transport layer which means that the foundation of the Generic Netlink message is the standard Netlink message format - the only difference is the inclusion of a Generic Netlink message header. The format of the message is defined as shown below:

  0                   1                   2                   3
  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |                Netlink message header (nlmsghdr)              |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |           Generic Netlink message header (genlmsghdr)         |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |             Optional user specific message header             |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |           Optional Generic Netlink message payload            |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

*/
 

pub struct GenlFamily (*mut bindings::genl_family); 
impl GenlFamily {


    /// as_ptr 
    /// Returns a raw pointer to a `genl_family` struct in C.
    ///
    /// # Safety
    ///
    /// This function returns a raw pointer to a `genl_family` struct in C.
    /// The caller is responsible for ensuring that the pointer is used safely.
    
    pub unsafe fn as_ptr(&self) -> *mut bindings::genl_family {
        self.0  
    }


    ///  genl_register_family - register a generic netlink family
    /// @family: generic netlink family
    ///
    /// Registers the specified family after validating it first. Only one
    /// family may be registered with the same family name or identifier.
    ///
    /// The family's ops, multicast groups and module pointer must already
    /// be assigned.
    ///
    /// Return 0 on success or a negative error code.
    /// 
    pub fn genl_register_family(&self)->i32{
        unsafe { bindings::genl_register_family(self.0) }
    } 

    
    /// genl_unregister_family - unregister generic netlink family
    /// @family: generic netlink family
    /// Unregisters the specified family.
    ///
    /// Returns 0 on success or a negative error code.
    /// 
    pub fn genl_unregister_family(&self)->i32{
        unsafe { bindings::genl_unregister_family(self.0) }
    }
    
}

/// GenlInfo struct wraps C's struct genl_info
/// u32 snd_seq
/// This is the Netlink sequence number of the request.
/// u32 snd_pid
/// This is the Netlink PID of the client which issued the request; it is important to note that the Netlink PID is not the same as the standard kernel PID.
/// struct nlmsghdr *nlhdr
/// This is set to point to the Netlink message header of the request.
/// struct genlmsghdr *genlhdr
/// This is set to point to the Generic Netlink message header of the request.
/// void *userhdr
/// If the Generic Netlink family makes use of a family specific header, this pointer will be set to point to the start of the family specific header.
/// struct nlattr **attrs
/// The parsed Netlink attributes from the request; if the Generic Netlink family definition specified a Netlink attribute policy then the attributes would have already been validated.
/// The doit() handler should do whatever processing is necessary and return zero on success or a negative value on failure. Negative return values will cause an NLMSG_ERROR message to be sent while a zero return value will only cause the NLMSG_ERROR message to be sent if the request is received with the NLM_F_ACK flag set.
 
pub struct GenlInfo (*mut bindings::genl_info);
impl GenlInfo {

    /// returns a raw pointer to a genl_info struct in C
    /// # Safety 
    /// well let's hope it works first 
    pub unsafe fn as_ptr(&self) -> *mut bindings::genl_info {
        self.0

    }

    ///  he genl_info_net function extracts the _net field from a genl_info struct
    ///  and returns a pointer to the net struct that it points to.
    ///  This function is likely used in the Linux kernel to extract
    ///  network namespace information from GENL messages.
    pub fn genl_info_net(&self) -> Net {
        unsafe { Net(bindings::genl_info_net(self.0)) }
    }


}

/// GenlOps wraps genl_ops C function in rust 
pub struct GenlOps (*mut bindings::genl_ops);
impl GenlOps {

    /// Returns a raw pointer to a `genl_ops` struct in C.
    ///
    /// # Safety
    ///
    /// This function returns a raw pointer to a `genl_ops` struct in C.
    /// The caller is responsible for ensuring that the pointer is used safely.
    pub unsafe fn as_ptr(&self) -> *mut bindings::genl_ops {
        self.0
    }

}

/// Net wraps net struct in rust 
pub struct Net (*mut bindings::net);
impl Net {
    
    /// returns a raw pointer to a net struct in C 
    /// # Safety 
    /// well let's hope it works first 

    pub fn as_ptr(&self) -> *mut bindings::net {
        self.0
    }

    /// genlmsg_unicast - unicast a netlink message
    /// @net: network namespace to look up @portid in
    /// @skb: netlink message as socket buffer
    /// @portid: netlink portid of the destination socket

    pub fn genlmsg_unicast(&self, skb: &SkBuff,portid: u32_,) -> core::ffi::c_int {
        unsafe { bindings::genlmsg_unicast(self.0, skb.as_ptr(), portid) }
    }

}

impl SkBuff {

    /// The genlmsg_put() function creates the required Netlink and Generic Netlink message headers, 
    /// populating them with the given values; see the Generic Netlink header file for a description of the parameters.
    /// The nla_put_string() function is a standard Netlink attribute function which adds a string attribute to the end
    /// of the Netlink message; see the Netlink header file for a description of the parameters.
    /// The genlmsg_end() function updates the Netlink message header once the message payload has been finalized.
    /// This function should be called before sending the message.
    ///
    /// genlmsg_put - Add generic netlink header to netlink message
    /// @skb: socket buffer holding the message
    /// @portid: netlink portid the message is addressed to
    /// @seq: sequence number (usually the one of the sender)
    /// @family: generic netlink family
    /// @flags: netlink message flags
    /// @cmd: generic netlink command
    ///
    /// Returns pointer to user specific header
    /// 
    pub fn genlmsg_put(&self, portid: u32_, seq: u32_, family: &GenlFamily, flags: core::ffi::c_int, cmd: u8_,
    ) -> *mut core::ffi::c_void {
        unsafe { bindings::genlmsg_put(self.as_ptr(), portid, seq, (*family).0, flags, cmd) }
    }

}   
    /// genl_register_ops
    /// NOTE: This function doesn't exist past linux 3.12. Up to linux 4.10,
    /// use genl_register_family_with_ops(). On 4.10 and later,
    /// include a reference to your genl_ops struct as an element in the genl_family struct (element .ops),
    /// as well as the number of commands (element .n_ops).

    pub fn genl_register_ops(family: GenlFamily, ops: GenlOps , n_ops : u8) -> Result<(), i32> {
        unsafe {
            (*family.0).ops = ops.0;
            (*family.0).n_ops = n_ops; // Assuming one echo command
        
            let ret = family.genl_register_family() ;
            if ret < 0 {
            Err(ret)
            } else {
            Ok(())
            }
        }
    }
    
    
    /// genl_unregister_ops - unregister generic netlink operations
    /// @family: generic netlink family
    /// @ops: operations to be unregistered
    ///
    /// Unregisters the specified operations and unassigns them from the
    /// specified family. The operation blocks until the current message
    /// processing has finished and doesn't start again until the
    /// unregister process has finished.
    ///
    /// Note: It is not necessary to unregister all operations before
    /// unregistering the family, unregistering the family will cause
    /// all assigned operations to be unregistered automatically.
    ///
    /// Returns 0 on success or a negative error code.
    /// 
    pub fn unregister_netlink_ops(family: GenlFamily) -> Result<(), i32> {
    
    let ret = family.genl_unregister_family() ;
    if ret < 0 {
      Err(ret)
    } else {
      Ok(())
    }
    }
