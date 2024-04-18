


use bindings::sk_buff;
use bindings::skb_put ; 

use core::ffi::c_void;
use core::ffi::CStr;
use core::mem;
use core::ptr::null;
use crate::str::CString ; 

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
}*/


pub fn nla_put_string(skb: &mut sk_buff, attrtype: i32, str: &CStr) {
    unsafe{
    let attrlen = str.to_bytes().len() + 1;

    nla_put(skb, attrtype, attrlen as i32, str.to_bytes().as_ptr() as *const c_void);
    }
}

pub unsafe fn skb_reserve(skb: *mut sk_buff, len: i32) {
    let len_usize = len as usize;
    unsafe {
    (*skb).data = ((*skb).data as usize + len_usize) as *mut u8;
    (*skb).tail = ((*skb).tail as usize + len_usize) as u32 ;
    }
}


// Genetlink APIs 
#[test]
fn test_nla_put_success() {
    // Allocate a sk_buff with sufficient tailroom.
    let mut skb = sk_buff::default();
    // ... (set tail and end pointers for sufficient tailroom)

    // Set attribute data
    let attrtype = 10;
    let attrlen = 16;
    let data = [1, 2, 3, 4]; // Sample data

    // Call nla_put
    let result = unsafe { nla_put(&skb as *const sk_buff, attrtype, attrlen, data.as_ptr() as *const c_void) };

    // Assertions
    assert_eq!(result, 0); // Verify successful return code

    // Check nlattr content (assuming you have a way to access skb data)
    // ... (verify nlattr type, length, and data match expectations)
}

#[test]
fn test_nla_put_insufficient_tailroom() {
    // Allocate a sk_buff with insufficient tailroom.
    let mut skb = sk_buff::default();
    // ... (set tail and end pointers for insufficient tailroom)

    // Set attribute data
    let attrtype = 10;
    let attrlen = 16;
    let data = [1, 2, 3, 4]; // Sample data

    // Call nla_put
    let result = unsafe { nla_put(&skb as *const sk_buff, attrtype, attrlen, data.as_ptr() as *const c_void) };

    // Assertions
    assert_eq!(result, -40); // Verify error return code for insufficient tailroom
}

#[test]
#[should_panic]
fn test_nla_put_null_skb() {
    // Set null skb pointer
    let skb: *const sk_buff = null();

    // Set attribute data
    let attrtype = 10;
    let attrlen = 16;
    let data = [1, 2, 3, 4]; // Sample data

    // Call nla_put (should panic)
    unsafe { nla_put(skb, attrtype, attrlen, data.as_ptr() as *const c_void) };
}

#[test]
fn test_skb_nonlinear_linear() {
    // Allocate a linear sk_buff
    let mut skb = sk_buff::default();
    // ... (set flags to indicate linear sk_buff)

    // Call skb_nonlinear
    let is_nonlinear = unsafe { skb_nonlinear(&skb as *const sk_buff) };

    // Assertion
    assert!(!is_nonlinear); // Verify it returns false for linear skb
}

#[test]
fn test_skb_nonlinear_nonlinear() {
    // Allocate a nonlinear sk_buff
    let mut skb = sk_buff::default();
    // ... (set flags to indicate nonlinear sk_buff)

    // Call skb_nonlinear
    let is_nonlinear = unsafe { skb_nonlinear(&skb as *const sk_buff) };

    // Assertion
    assert!(is_nonlinear); // Verify it returns true for nonlinear skb
}
#[test]
fn test_skb_tailroom_linear() {
    // Allocate a linear sk_buff with some data
    let mut skb = sk_buff::default();
    // ... (set data and tail pointers)

    let tailroom = unsafe { skb_tailroom(&skb as *const sk_buff) };

    // Assertion
    let expected_tailroom = (skb.end - skb.tail) as i32;
    assert_eq!(tailroom, expected_tailroom); // Verify tailroom matches end-tail difference
}

#[test]
fn test_skb_tailroom_nonlinear() {
    // Allocate a nonlinear sk_buff
    let mut skb = sk_buff::default();
    // ... (set flags to indicate nonlinear sk_buff)

    let tailroom = unsafe { skb_tailroom(&skb as *const sk_buff) };

    // Assertion
// Assertion
assert_eq!(tailroom, 0); // Verify tailroom is 0 for nonlinear skb
}

#[test]
fn test_nla_total_size() {
    let payload = 10;
    let total_size = unsafe { nla_total_size(payload) };

    let expected_size = NLA_HDRLEN as i32 + nla_align(payload as usize) as i32;
    assert_eq!(total_size, expected_size); // Verify total size includes header and aligned payload
}

#[test]
fn test_nla_align() {
    let len = 8;
    let aligned_len = nla_align(len);

    assert!(aligned_len >= len as i32); // Verify aligned length is greater than or equal to original length
    assert!((aligned_len % NLA_ALIGNTO as i32) == 0); // Verify aligned length is a multiple of NLA_ALIGNTO
}
// This test case requires creating an nlattr and accessing the skb data (implementation specific)
/*#[test]
fn test_nla_data() {
    // ... (create an nlattr within a sk_buff)

    let data_ptr = unsafe { nla_data(nla_ptr) }; // Replace nla_ptr with your nlattr pointer

    // Assertion
    // ... (verify data_ptr points to the data section after nlattr header)
}*/

/*#[test]
fn test_nla_put_string() {
    let mut skb = sk_buff::default();
    // ... (set tailroom for skb)

    let test_str = "Hello, world!";

    unsafe { nla_put_string(&mut skb, 1, test_str) };

    // Assertions
    // ... (verify nla_put is called with expected parameters based on string length)
    // ... (or verify nlattr content if you have access to skb data)
}
*//*
#[test]
fn test_skb_reserve_positive() {
    let mut skb = sk_buff::default();
    // ... (set data and tail pointers with some data)

    let reserve_len = 5;
    unsafe { skb_reserve(&mut skb, reserve_len) };

    // Assertions
    assert_eq!(skb.data as usize + reserve_len as usize, skb.tail as usize); // Verify data and tail pointers are moved by reserve_len
}*/

#[test]
fn test_skb_reserve_zero() {
    let mut skb = sk_buff::default();
    // ... (set data and tail pointers with some data)

    let reserve_len = 0;
    unsafe { skb_reserve(&mut skb, reserve_len) };

    // Assertions
    assert_eq!(skb.data, skb.tail as *mut u8); // Verify data and tail pointers remain unchanged
}
