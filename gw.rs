static CE_GW_GENL_POLICY: [NlaPolicy; CE_GW_A_MAX + 1] = [
    NlaPolicy { type_: NLA_NUL_STRING },
    NlaPolicy { type_: NLA_NUL_STRING },
    NlaPolicy { type_: NLA_NUL_STRING },
    NlaPolicy { type_: NLA_U32 },
    NlaPolicy { type_: NLA_U32 },
    NlaPolicy { type_: NLA_U8 },
    NlaPolicy { type_: NLA_U32 },
    NlaPolicy { type_: NLA_U32 },
];





fn ce_gw_netlink_echo(skb_info: Skbuff, info: GenlInfo) -> i32 {
    let mut skb: Skbuff;
    let mut err: i32;
    let mut user_hdr: *mut core::ffi::c_void;

      if info.is_null() {
        // Print an error message and return -1
        pr_err!("ce_gw: info attribute is missing. No Message received.\n");
        return -1;
    }

    // Access the CE_GW_A_DATA attribute
    let nla_a_msg = unsafe { (*info).attrs[CE_GW_A_DATA as usize] };
    let nla_a_msg_pay = unsafe { nla_data(nla_a_msg) as *const u8 };

    if nla_a_msg_pay.is_null() {
        // Print a warning about missing string message
        pr_warning!("ce_gw: String Message is missing.\n");
    } else {
        // Print the received message
        pr_info!("ce_gw: Message received: {:?}\n", unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(nla_a_msg_pay, nla_len(nla_a_msg)))
        });
    }

    // Allocate a new sk_buff for sending a message back
    skb = unsafe { genlmsg_new(NLMSG_GOODSIZE, GFP_KERNEL) };
    if skb.is_null() {
        err = -ENOMEM;
        pr_err!("ce_gw: Socket allocation failed.\n");
        // Handle the error (e.g., goto ce_gw_add_error)
        // ...
    }

    // Return a value (e.g., 0 for success)
    0
}
