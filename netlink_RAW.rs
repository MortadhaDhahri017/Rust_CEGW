


use crate::bindings::sk_buff;

use core::ffi::c_void;
use core::ffi::CStr;
use core::mem;
 
use core::ptr;

pub const NLA_ALIGNTO: usize = 4; // Changed type to usize
pub const NLA_HDRLEN: usize = {

    let hdrlen = mem::size_of::<nlattr>();

    let aligned_hdrlen = (hdrlen + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1);

    aligned_hdrlen

};

#[repr(C)]
pub struct nlattr {
    pub nla_len: u16, // Changed to u16
    pub nla_type: u16, // Changed to u16
}

/*extern "C" {
    fn skb_nonlinear(skb: *const sk_buff) -> bool; // Changed return type to bool
    fn skb_tailroom(skb: *const sk_buff) -> i32; // Changed return type to i32
    fn nla_total_size(skb: *const sk_buff) -> i32; // Changed return type to i32
    fn nla_put(skb: *const sk_buff, attrtype: i32, attrlen: i32, data: *const c_void) -> i32; // Changed return type to i32
    fn __nla_put(skb: *mut sk_buff, attrtype: i32, attrlen: i32, data: *const c_void) -> Result<(), Option<NoneError>>; // Changed return type to Result
    fn __nla_reserve(skb: *mut sk_buff, attrtype: i32, attrlen: i32) -> *mut nlattr; // Changed return type to *mut nlattr
}*/

pub unsafe fn nla_put(skb: *const sk_buff, attrtype: i32, attrlen: i32, data: *const c_void) -> i32 {
    unsafe{if skb_tailroom(skb) < nla_total_size(attrlen) {
        return -40; // Changed to return -40 directly
    }
    __nla_put(skb as *mut sk_buff, attrtype, attrlen, data); // Casted 'skb' to *mut sk_buff
    0 // Changed to return 0 directly }
}
}
pub unsafe fn __nla_put(skb: *mut sk_buff, attrtype: i32, attrlen: i32, data: *const c_void)  {
    unsafe {let nla = __nla_reserve(skb as *mut sk_buff, attrtype, attrlen); // Casted 'skb' to *mut sk_buff    
    ptr::copy_nonoverlapping(data as *const u8, (nla as *mut u8).add(mem::size_of::<nlattr>()), attrlen as usize);
    (*nla).nla_type = attrtype as u16; // Changed type to u16
    (*nla).nla_len = nla_attr_size(attrlen) as u16 ; }// Removed casting to c_int
}

pub unsafe fn skb_nonlinear(skb: *const sk_buff) -> bool {
    if skb.is_null() {
        panic!("skb is null");
    }
    true // Changed to return true
}

pub unsafe fn skb_tailroom(skb: *const sk_buff) -> i32 {
    unsafe{if skb_nonlinear(skb) {
        0
    } else {
        ((*skb).end - (*skb).tail) as i32 // Removed 'as c_int'
    }
}
}

pub unsafe fn nla_total_size(payload: i32) -> i32 {
    nla_align(nla_attr_size(payload)as usize)
}

pub fn nla_align(len: usize) -> i32 {
    ((len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)) as i32 // Added casting to i32
}

pub fn nla_attr_size(payload: i32) -> i32 {
    NLA_HDRLEN as i32 + payload
}

pub unsafe fn __nla_reserve(skb: *mut sk_buff, attrtype: i32, attrlen: i32) -> *mut nlattr {
    unsafe {
        let nla: *mut nlattr;
        nla = bindings::skb_put(skb, nla_total_size(attrlen)as u32) as *mut nlattr; // Changed to 'bindings::skb_put'
        (*nla).nla_type = attrtype as u16; // Changed type to u16
        (*nla).nla_len = nla_attr_size(attrlen) as u16; // Removed casting to c_int
        let padlen = nla_padlen(attrlen);
        if padlen > 0 {
            ptr::write_bytes((nla as *mut u8).add(core::mem::size_of::<nlattr>() + (*nla).nla_len as usize), 0, padlen as usize); // Added casting to usize
                }
        nla }
}

pub fn nla_padlen(payload: i32) -> i32 {
   unsafe {nla_total_size(payload) - nla_attr_size(payload) }
}

pub fn nla_data(nla: *const nlattr) -> *const c_void {
    unsafe {(nla as *const _ as *const c_void).add(NLA_HDRLEN as usize) }
}


pub fn nla_put_string(skb: &mut sk_buff, attrtype: i32, str: &CStr) {
    unsafe{
    let attrlen = str.to_bytes().len() + 1;

    nla_put(skb, attrtype, attrlen as i32, str.to_bytes().as_ptr() as *const c_void);
    }
}
