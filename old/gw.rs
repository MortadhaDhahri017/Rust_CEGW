/// Different Importations : Begin 

use crate::netlink::* ; 
use crate::genetlink::*; 
use core::ffi::* ; 
use core::option::Option::*;
use core::mem::*;
use core::ptr ; 
use alloc::boxed::Box; 
use crate::str::CStr ;
use crate::bindings::*; 
use crate::{
    pr_info , pr_err , pr_warn
} ; 


//pub const CE_GW_GE_FAMILY_NAME: &CStr = "Mortadha" ; 
pub const CE_GW_GE_FAMILY_VERSION: c_uint = 2;
pub const CE_GW_USER_HDR_SIZE: c_size_t = 0; // <user header size>
pub const CE_GW_NO_FLAG: c_uint = 0;


pub struct SkBuff(*mut bindings::sk_buff) ; 

impl SkBuff{
    
    pub unsafe fn as_ptr(&self) -> *mut bindings::sk_buff {
        self.0
    }
}


/// Defining the different constants we need : 
pub const NLA_UNSPEC: core::ffi::c_uint = 0;
pub const NLA_U8: core::ffi::c_uint = 1;
pub const NLA_U16: core::ffi::c_uint = 2;
pub const NLA_U32: core::ffi::c_uint = 3;
pub const NLA_U64: core::ffi::c_uint = 4;
pub const NLA_STRING: core::ffi::c_uint = 5;
pub const NLA_FLAG: core::ffi::c_uint = 6;
pub const NLA_MSECS: core::ffi::c_uint = 7;
pub const NLA_NESTED: core::ffi::c_uint = 8;
pub const NLA_NESTED_ARRAY: core::ffi::c_uint = 9;
pub const NLA_NUL_STRING: core::ffi::c_uint = 10;
pub const NLA_BINARY: core::ffi::c_uint = 11;
pub const NLA_S8: core::ffi::c_uint = 12;
pub const NLA_S16: core::ffi::c_uint = 13;
pub const NLA_S32: core::ffi::c_uint = 14;
pub const NLA_S64: core::ffi::c_uint = 15;
pub const NLA_BITFIELD32: core::ffi::c_uint = 16;
pub const NLA_REJECT: core::ffi::c_uint = 17;
pub const __NLA_type__MAX: core::ffi::c_uint = 18;



/// @enum
/// @brief Data which can be send and received.
/// @details Set int values as Identifiers for userspace application.
/// @ingroup net


use bindings::nla_policy;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CeGwAttr {
    CeGwAUnspec = 0, //< Dummy to skip index 0 
    CeGwAData = 1,   //< NLA_STRING 
    CeGwASrc = 2,    //< NLA_STRING 
    CeGwADst = 3,    //< NLA_STRING 
    CeGwAId = 4,     //< NLA_U32 
    CeGwAFlags = 5,  //< NLA_U32 
    CeGwAtype_ = 6,   //< NLA_U8 
    CeGwAHndl = 7,   //< NLA_U32 Handled Frames 
    CeGwADrop = 8,   //< NLA_U32 Dropped Frames 
    __CeGwAMax = 9,  //< Maximum Number of Attribute + 1 
}

impl CeGwAttr {
    pub fn as_nla_type_(&self) -> u32 {
        match self {
            CeGwAttr::CeGwAData => NLA_STRING as u32,
            CeGwAttr::CeGwASrc => NLA_STRING as u32,
            CeGwAttr::CeGwADst => NLA_STRING as u32,
            CeGwAttr::CeGwAId => NLA_U32 as u32,
            CeGwAttr::CeGwAFlags => NLA_U32 as u32,
            CeGwAttr::CeGwAtype_ => NLA_U8 as u32,
            CeGwAttr::CeGwAHndl => NLA_U32 as u32,
            CeGwAttr::CeGwADrop => NLA_U32 as u32,
            _ => 0,
        }
    }

    pub fn as_nla_policy(&self) -> nla_policy {
        nla_policy {
            type_: self.as_nla_type_() as u8,
            ..nla_policy::default()
        }
    }
}


pub const CE_GW_A_MAX: u32 = CeGwAttr::__CeGwAMax as u32 - 1; 


pub fn ce_gw_genl_family() -> bindings::genl_family {

    bindings::genl_family {
        id:123,
        hdrsize: CE_GW_USER_HDR_SIZE as u32,
        //name: CE_GW_GE_FAMILY_NAME.as_ptr(),
        version: CE_GW_GE_FAMILY_VERSION,
        maxattr: CE_GW_A_MAX,
        //netnsok: false,
        pre_doit: None,
        post_doit: None , 
        ..Default::default()
    }

}

/**
 * @enum
 * @brief Generic Netlink Commands
 * @details Set int values as Identifiers for userspace application.
 */
#[repr(i32)]
pub enum CeGwCommand {
    CeGwCUnspec ,
    CeGwCEcho ,   
    CeGwCAdd ,    
    CeGwCDel ,   
    CeGwCList ,   
    CeGwCMax(i32),    
}
//pub const __CeGwCMax :isize= CeGwCommand::CeGwCMax - 1 ; 



///
/// @fn int ce_gw_netlink_echo(struct sk_buff *skb_info, struct genl_info *info)
/// @brief Generic Netlink Command - Sends a massage back
/// @param skb_info Netlink Socket Buffer with Message
/// @param info Additional Netlink Information
/// @retval 0 if parsing is finished
/// @ingroup net
///

pub fn is_null_ref<T>(ptr: Option<&T>) -> bool {
    ptr.is_none()
}

pub fn is_null(ptr:*const bindings::sk_buff)->bool {
    ptr.is_null()
}
pub fn is_null_info(ptr:*const bindings::genl_info)->bool {
    ptr.is_null()
}



fn ce_gw_netlink_echo(skb_info: SkBuff, info: Option<&GenlInfo>) -> i32 {
    let mut skb:SkBuff ; 
    let mut err: i32 ;
    let mut user_hdr: *mut core::ffi::c_void;

      if is_null_ref(info) {
        // Print an error message and return -1
        pr_err!("ce_gw: info attribute is missing. No Message received.\n");
        return -1;
    }

    // Access the CE_GW_A_DATA attribute
    let nla_a_msg = Nlattr::default()  ; //unsafe { (*info).unwrap().attrs[CeGwAttr::__CeGwAMax as usize] };
    let nla_a_msg_pay = unsafe { nla_data(nla_a_msg.as_ptr()) as *const u8 };

    if nla_a_msg_pay.is_null() {
        // Print a warning about missing string message
        pr_warn!("ce_gw: String Message is missing.\n");
    } else {
        // Print the received message
        pr_info!("ce_gw: Message received: {:?}\n", unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(nla_a_msg_pay, nla_len(nla_a_msg.as_ptr()) as usize ))
        });
    }

    // Allocate a new sk_buff for sending a message back
    skb = unsafe {SkBuff(genlmsg_new(100, GFP_KERNEL)) }; // NLMSG_GOODSIZE HAS BEEN SET TO 100
    unsafe {if is_null(skb.as_ptr()) {
        err = -40 as i32 ;
        pr_err!("ce_gw: Socket allocation failed.\n");
        // Handle the error (e.g., goto ce_gw_add_error)
        // ...
    }
}
    // Prepare the response message header
    let user_hdr = unsafe {
        genlmsg_put(
            skb.as_ptr() as *mut _, // Convert SkBuff to raw pointer (unsafe)
            (*info.unwrap().as_ptr()).snd_portid,  // Guaranteed to be Some due to previous check
            (*info.unwrap().as_ptr()).snd_seq,
            &ce_gw_genl_family as *const _ as *mut _, // Convert struct reference to raw pointer (unsafe)
            CE_GW_NO_FLAG as i32,
            1 as u8 , // contains CE_GW_C_Echo
        )
    };

    // Check for errors during header creation
    if user_hdr.is_null() {
        err = 40;
        pr_err!("ce_gw: Error during putting header\n");
        unsafe { kfree_skb(skb.as_ptr()) }; // Free the allocated skb on error
        return err;
    }

    // Add "hello world from kernel space" attribute
    let err = unsafe { nla_put_string(skb.as_ptr() as *mut _, 4 , "hello world from kernel space\n".as_ptr() as *const i8) };
    if err != 0 {
        pr_err!("ce_gw: Putting Netlink Attribute Failed.\n");
        unsafe { kfree_skb(skb.as_ptr()) }; // Free the allocated skb on error
        return err;
    }

    // Finalize the message header
    unsafe { genlmsg_end(skb.as_ptr() as *mut _, user_hdr) };

    // Send the message (consider error handling for specific kernel versions)
    let err = unsafe {genlmsg_unicast(genl_info_net(info.unwrap().as_ptr()) , skb.as_ptr() as *mut _, (*info.unwrap().as_ptr()).snd_portid)} ; 
    /* *    
     {
        Ok(_) => 0,
        Err(val) => {
            pr_err!("ce_gw: Message sending failed.\n");
            unsafe { kfree_skb(skb) }; // Free the allocated skb on error
            val
        }
    };

    // Cleanup and return
    unsafe { kfree_skb(skb) }; // Free the allocated skb after sending
    */
    err 
}

