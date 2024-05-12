use crate::bindings::*;
use crate::netlink::SkBuff ; 


/// NetDevice wraps C struct net_device 
pub struct NetDevice(*mut bindings::net_device);


impl NetDevice {

    ///    as_ptr : a function to extract 
    ///    a raw pointer from the net_device stuct
    pub fn as_ptr(&self) -> *mut bindings::net_device 
    {
        self.0
    }

    
    ///    register_netdev
    ///    - register a network device
    ///    @dev: device to register
    ///
    ///    Take a completed network device structure and add it to the kernel
    ///    interfaces. A %NETDEV_REGISTER message is sent to the netdev notifier
    ///    chain. 0 is returned on success. A negative errno code is returned
    ///    on a failure to set up the device, or if the name is a duplicate.
    

    pub fn register_netdev(&self) -> core::ffi::c_int 
    {
        unsafe { bindings::register_netdev(self.0) }
    }

    
    ///    unregister_netdev - remove device from the kernel
    ///    @dev: device
    ///
    ///    This function shuts down a device interface and removes it
    ///    from the kernel tables.
    ///
    ///    This is just a wrapper for unregister_netdevice that takes
    ///    the rtnl semaphore.  In general you want to use this and not
    ///    unregister_netdevice.
    
    pub fn unregister_netdev(&self) 
    {
        unsafe { bindings::unregister_netdev(self.0) }
    }
    
    
    ///    netif_start_queue - allow transmit
    ///    @dev: network device
    ///
    ///    Allow upper layers to call the device hard_start_xmit routine.
     

    pub fn netif_start_queue(&self) 
    {
        unsafe { bindings::netif_start_queue(self.0) }
    }


    
    ///    netif_stop_queue - stop transmitted packets
    ///    @dev: network device
    ///
    ///    Stop upper layers calling the device hard_start_xmit routine.
    ///    Used for flow control when transmit resources are unavailable.
    


    pub fn netif_stop_queue(&self)
    {
        unsafe { bindings::netif_stop_queue(self.0) }
    }


    
    ///
    ///    netif_device_present - is device available or removed
    ///    @dev: network device
    ///
    ///    Check if device has not been removed from system.
    

    pub fn netif_device_present(&self) -> bool
    {
        unsafe { bindings::netif_device_present(self.0) as bool} 
    }

 
    
    
    ///    free_netdev - free network device
    ///    @dev: device
    ///
    ///    This function does the last stage of destroying an allocated device
    ///    interface. The reference to the device object is released. If this
    ///    is the last reference then it will be freed.Must be called in process
    ///    context.
    
    pub fn free_netdev(&self)
    {
        unsafe { bindings::free_netdev(self.0) }
    }

    
    ///    netdev_priv - access network device private data
    ///    @dev: network device
    ///
    ///    Get network device private data
    

    pub fn netdev_priv(&self){
        unsafe { bindings::netdev_priv(self.0) }
    }
    
    ///	netdev_alloc_skb - allocate an skbuff for rx on a specific device
    ///	@dev: network device to receive on
    ///	@length: length to allocate
    ///
    ///	Allocate a new &sk_buff and assign it a usage count of one. The
    ///	buffer has unspecified headroom built in. Users should allocate
    ///	the headroom they think they need without accounting for the
    ///	built in space. The built in space is used for optimisations.
    ///
    ///	%NULL is returned if there is no free memory. Although this function
    /// allocates memory it can be called from an interrupt.
    
    pub fn netdev_alloc_skb(&self , length : core::ffi::c_uint)->*mut bindings::sk_buff {
        unsafe {
             bindings::netdev_alloc_skb(self.as_ptr(),length) 
        }
    }

    ///
    ///	dev_put - release reference to device
    ///	@dev: network device
    ///
    /// Release reference to device to allow it to be freed.
    /// Try using netdev_put() instead.
    ///
    pub fn dev_put(&self) {
        unsafe {
            bindings::dev_put(self.0) 
        }
    }

}

///    KmemCache wrapping kmem_cache of C #include <linux/slab.h> 
pub struct KmemCache(*mut bindings::kmem_cache) ; 
impl  KmemCache {

    ///    as_ptr : a function to extract 
    ///    a raw pointer from the kmem_cache stuct
    
    pub fn as_ptr(&self) -> *mut bindings::kmem_cache 
    {
        self.0
    }

    ///    kmem_cache_alloc
    ///    is used to allocate memory from a kernel cache.
    ///    It is a part of the kernel's memory management system and
    ///    is used to efficiently allocate memory 
    ///    for kernel objects that have a uniform size.

    pub fn kmem_cache_alloc(&self, flags: bindings::gfp_t) -> *mut core::ffi::c_void 
    {
        unsafe { bindings::kmem_cache_alloc(self.0, flags) }
    }

    ///    kmem_cache_free
    ///    is used to free memory
    ///    that was previously allocated by kmem_cache_alloc()
    ///    in the Linux kernel memory management system.
    ///    It releases the memory back to the slab cache,
    ///    making it available for reuse. 

    pub fn kmem_cache_free(&self, objp: *mut core::ffi::c_void) 
    {
        unsafe { bindings::kmem_cache_free(self.0, unsafe {objp}) }
    }

    ///    kmem_cache_destroy
    ///    is used to destroy a slab cache in the Linux kernel memory management system.
    ///    It releases all the memory associated with the cache and frees any resources used by the cache.

    pub fn kmem_cache_destroy(&self) 
    {
        unsafe { bindings::kmem_cache_destroy(self.0) }
    }
    
}

///    HlistHead This struct wraps hlist_head's struct in C 
pub struct HlistHead(*mut bindings::hlist_head) ;

///   HlistNode This struct wraps hlist_node's struct in C 
pub struct HlistNode(*mut bindings::hlist_node) ; 

impl HlistNode {

    ///    as_ptr : a function to extract 
    ///    a raw pointer from the hlist_node struct
    pub fn as_ptr(&self) -> *mut bindings::hlist_node
    {
        self.0
    }

    ///    hlist_del_rcu 
    ///    deletes entry from hash list without re-initialization
    ///    @n: the element to delete from the hash list.
    ///
    ///    Note: list_unhashed() on entry does not return true after this,
    ///    the entry is in an undefined state. It is useful for RCU based
    ///    lockfree traversal.
       
    pub fn hlist_del_rcu(&self){
        unsafe { bindings::hlist_del_rcu(self.0) }
    }
}


///
///    hlist_add_tail_rcu
///    @n: the element to add to the hash list.
///    @h: the list to add to.
///
///    Description:
///    Adds the specified element to the specified hlist,
///    while permitting racing traversals.

pub fn hlist_add_head_rcu(n:&HlistNode,h:&HlistHead)
    {   
    unsafe { bindings::hlist_add_head_rcu(n.0,h.0)
     }
    }

///
///hlist_add_head - add a new entry at the beginning of the hlist
///@n: new entry to be added
///@h: hlist head to add it after
///
///Insert a new entry after the specified head.
///This is good for implementing stacks.
///



/// Allocates a new `net_device` structure with a single transmit and receive queue.
///
/// This function is unsafe because it directly manipulates memory and assumes that
/// the provided arguments are valid.
///
/// # Safety
///
/// - `sizeof_priv` must be the correct size of the private data associated with the
///   `net_device` structure.
/// - `name` must be a valid null-terminated string.
/// - `name_assign_type` must be a valid value for the `IFF_NAMETYPE` flag.
/// - `setup` must be a valid function pointer that can be called with a mutable
///   `net_device` pointer as its argument.
///
/// # Returns
///
/// A pointer to the allocated `net_device` structure, or `null_mut()` if the
/// allocation fails.

pub unsafe fn alloc_netdev (sizeof_priv:core::ffi::c_int,
    name: *const core::ffi::c_char,
    name_assign_type: core::ffi::c_uchar,
    setup: ::core::option::Option<unsafe extern "C" fn(arg1: *mut net_device)>,
    _txqs: core::ffi::c_uint,
    _rxqs: core::ffi::c_uint,)->  *mut bindings::net_device {
unsafe {bindings::alloc_netdev_mqs(sizeof_priv,name,name_assign_type,setup,1,1) } 
}


///    Defining macro HLIST_HEAD in c : 
///    macro hlist_head! 
///    hlist_head
///    Opting for using Option is also an option here 

#[warn(unused_macros)]
macro_rules! hlist_head {
    ($name:ident) => {
    struct $name {
    first: Option<Box>,
    }
    impl $name {
    fn new() -> Self {
    $name { first: None }
    }
    }
    };
    }


///    Defining macro hlist_entery from c 
///    using container_of! macro in rust  

#[warn(unused_macros)]
#[macro_export]
macro_rules! hlist_entry {
    ($ptr:expr, $type:ty, $($f:ident).*) => {{
        //    Use the container_of macro to calculate the address of the struct
        container_of!($ptr, $type, $($f).*)
    }};
}


///    Taking care of Null using Options in rust for the following macro 
///    hlist_entry_safe_macro 
///    hlist_entry_safe 
///    Usage : 
///    let ptr: Option<*const Field> = ...; // Pointer to the field inside the struct
///    let entry_ptr: Option<*const Struct> = hlist_entry_safe!(ptr, Struct, member);

#[warn(unused_macros)]

#[macro_export]
macro_rules! hlist_entry_safe {
    ($ptr:expr, $type:ty, $($f:ident).*) => {{
        let ____ptr = $ptr;
        if let Some(____ptr) = ____ptr {
            Some(hlist_entry!(____ptr, $type, $($f).*))
        } else {
            None
        }
    }};
}


///    hlist_for_eachçentry_safe
///    is a looping construct in the Linux kernel that is used 
///    to iterate over each element in a doubly linked list (hlist) in a safe manner.
///    It allows you to iterate over the list and free or modify the elements
///    within the list without worrying about invalidating the list itself.

#[warn(unused_macros)]

#[macro_export]
macro_rules! hlist_for_each_entry_safe {
    ($pos:ident, $n:ident, $head:expr, $type:ty, $member:ident) => {
        let mut $pos = hlist_entry_safe!($head.first, $type, $member);
        while let Some(mut ____pos) = $pos {
            let $n = unsafe { (*____pos).$member.next };
            $pos = hlist_entry_safe!($n, $type, $member) ; c
        }
    };
}
