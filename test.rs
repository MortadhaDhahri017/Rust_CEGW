use linux_version_code::LINUX_VERSION_CODE;
use linux_version_code::KERNEL_VERSION;
use std::ffi::CString;

fn ce_gw_netlink_list(skb_info: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut skb: *mut sk_buff;
    let mut err = 0;
    let mut user_hdr: *mut c_void;

    pr_debug("ce_gw_netlink: ce_gw_netlink_list is called.\n");

    let nla_id = (*info).attrs[CE_GW_A_ID as usize];
    let nla_id_data = nla_data(nla_id) as *mut u32;

    // TODO: Perhaps pare arguments like ID to only transmit a special GW
    let mut cgj: *mut ce_gw_job;
    let mut node: *mut hlist_node;

    if LINUX_VERSION_CODE >= KERNEL_VERSION(3, 9, 0) {
        hlist_for_each_entry_safe!(cgj, node, ce_gw_get_job_list(), list);
    } else {
        let mut pos: *mut hlist_node;
        hlist_for_each_entry_safe!(cgj, pos, node, ce_gw_get_job_list(), list);
    }

    if *nla_id_data != 0 && (*cgj).id != *nla_id_data {
        continue;
    }

    pr_debug("ce_gw_netlink: Job List entry is send.\n");

    skb = genlmsg_new(NLMSG_GOODSIZE, GFP_KERNEL);
    if skb.is_null() {
        pr_err("ce_gw: Socket allocation failed.\n");
        goto ce_gw_list_error;
    }

    if LINUX_VERSION_CODE < KERNEL_VERSION(3, 7, 0) {
        user_hdr = genlmsg_put(
            skb,
            (*info).snd_pid,
            (*info).snd_seq,
            &ce_gw_genl_family,
            NLM_F_MULTI,
            CE_GW_C_ECHO,
        );
    } else {
        user_hdr = genlmsg_put(
            skb,
            (*info).snd_portid,
            (*info).snd_seq,
            &ce_gw_genl_family,
            NLM_F_MULTI,
            CE_GW_C_ECHO,
        );
    }

    if user_hdr.is_null() {
        err = -ENOMEM;
        pr_err("ce_gw: Error during putting header\n");
        goto ce_gw_list_error;
    }

    err += nla_put_string(skb, CE_GW_A_SRC, CString::new((*cgj).src.dev.name).unwrap().as_ptr());
    err += nla_put_string(skb, CE_GW_A_DST, CString::new((*cgj).dst.dev.name).unwrap().as_ptr());
    err += nla_put_u32(skb, CE_GW_A_ID, (*cgj).id);
    err += nla_put_u32(skb, CE_GW_A_FLAGS, (*cgj).flags);
    err += nla_put_u8(skb, CE_GW_A_TYPE, (*cgj).type);
    err += nla_put_u32(skb, CE_GW_A_HNDL, (*cgj).handled_frames);
    err += nla_put_u32(skb, CE_GW_A_DROP, (*cgj).dropped_frames);

    if err != 0 {
        pr_err("ce_gw: Putting Netlink Attribute Failed.\n");
        goto ce_gw_list_error;
    }

    // Rest of the function implementation...

    return 0;

ce_gw_list_error:
    // Handle error case...
    return err;
}
