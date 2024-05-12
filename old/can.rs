
use crate::bindings::* ;

pub struct CanFrame(*mut bindings::can_frame) ; 
pub struct CanfdFrame(*mut bindings::canfd_frame) ;

static DLCLEN: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48, 64];


pub fn can_dlc2len(can_dlc: u8) -> u8 {

    DLCLEN[(can_dlc & 0x0F) as usize]

}




///
/// can_rx_register - subscribe CAN frames from a specific interface
/// @net: the applicable net namespace
/// @dev: pointer to netdevice (NULL => subscribe from 'all' CAN devices list)
/// @can_id: CAN identifier (see description)
/// @mask: CAN mask (see description)
/// @func: callback function on filter match
/// @data: returned parameter for callback function
/// @ident: string for calling module identification
/// @sk: socket pointer (might be NULL)
///
/// Description:
///  Invokes the callback function with the received sk_buff and the given
/// parameter 'data' on a matching receive filter. A filter matches, when
///
/// The filter can be inverted (CAN_INV_FILTER bit set in can_id) or it can
/// filter for error message frames (CAN_ERR_FLAG bit set in mask).
///
/// The provided pointer to the sk_buff is guaranteed to be valid as long as
/// the callback function is running. The callback function must *not* free
/// the given sk_buff while processing it's task. When the given sk_buff is
/// needed after the end of the callback function it must be cloned inside
/// the callback function with skb_clone().
///
///Return:
/// 0 on success
/// -ENOMEM on missing cache mem to create subscription entry
/// -ENODEV unknown device

pub fn can_rx_register(
    net: *mut bindings::net,
    dev: *mut bindings::net_device,
    can_id: bindings::canid_t,
    mask: bindings::canid_t,
    func: ::core::option::Option<unsafe extern "C" fn(arg1: *mut bindings::sk_buff, arg2: *mut core::ffi::c_void)>,
    data: *mut core::ffi::c_void,
    ident: *mut core::ffi::c_char,
    sk: *mut bindings::sock,

) -> core::ffi::c_int {
    unsafe {
        bindings::can_rx_register(net, dev, can_id, mask, func, data, ident, sk)
    }

}

///
/// can_rx_unregister - unsubscribe CAN frames from a specific interface
/// @net: the applicable net namespace
/// @dev: pointer to netdevice (NULL => unsubscribe from 'all' CAN devices list)
/// @can_id: CAN identifier
/// @mask: CAN mask
/// @func: callback function on filter match
/// @data: returned parameter for callback function
///
/// Description:
/// Removes subscription entry depending on given (subscription) values.
///


pub fn can_rx_unregister(
    net: *mut net,
    dev: *mut net_device,
    can_id: canid_t,
    mask: canid_t,
    func: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut sk_buff, arg2: *mut core::ffi::c_void),
    >,
    data: *mut core::ffi::c_void,
){
    unsafe {
        bindings::can_rx_unregister(net,dev,can_id,mask,func,data)
    }
}

///
///can_send - transmit a CAN frame (optional with local loopback)
/// @skb: pointer to socket buffer with CAN frame in data section
/// @loop: loopback for listeners on local CAN sockets (recommended default!)
///
/// Due to the loopback this routine must not be called from hardirq context.
///
/// Return:
/// 0 on success
/// -ENETDOWN when the selected interface is down
/// -ENOBUFS on full driver queue (see net_xmit_errno())
/// -ENOMEM when local loopback failed at calling skb_clone()
/// -EPERM when trying to send on a non-CAN interface
/// -EMSGSIZE CAN frame size is bigger than CAN interface MTU
///  -EINVAL when the skb->data does not contain a valid CAN frame


pub fn can_send(skb: *mut sk_buff, loop_: core::ffi::c_int) -> core::ffi::c_int {
    unsafe {
        bindings::can_send(skb,loop_)
    }
}
