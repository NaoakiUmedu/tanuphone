use pjsua::*;
use std::{ffi::CString, mem::MaybeUninit, os::raw::c_int, ptr};

const CSTRING_NEW_FAILED: &str = "CString::new failed!";

const SIP_DOMAIN: &str = "test.u.biztel.jp";
const SIP_USER: &str = "1001";
const SIP_PASSWD: &str = "p@ssw0rd";

const LOG_LEVEL_1: i32 = 1;
const LOG_LEVEL_2: i32 = 2;
const LOG_LEVEL_3: i32 = 3;
const LOG_LEVEL_4: i32 = 4;
const LOG_LEVEL_5: i32 = 5;

fn print_log(level: i32, msg: &str) {
    let cmsg = CString::new(msg).expect("CSTRING_NEW_FAILED");
    let dummfmt = CString::new("").expect("CSTRING_NEW_FAILED");
    let file = CString::new("APP").expect("CSTRING_NEW_FAILED");
    unsafe {
        match level {
            LOG_LEVEL_1 => pj_log_1(file.as_ptr(), cmsg.as_ptr(), dummfmt.as_ptr()),
            LOG_LEVEL_2 => pj_log_2(file.as_ptr(), cmsg.as_ptr(), dummfmt.as_ptr()),
            LOG_LEVEL_3 => pj_log_3(file.as_ptr(), cmsg.as_ptr(), dummfmt.as_ptr()),
            LOG_LEVEL_4 => pj_log_4(file.as_ptr(), cmsg.as_ptr(), dummfmt.as_ptr()),
            LOG_LEVEL_5 => pj_log_5(file.as_ptr(), cmsg.as_ptr(), dummfmt.as_ptr()),
            _ => (),
        };
    }
}

/* Callback called by the library upon receiving incoming call */
pub unsafe extern "C" fn on_incoming_call(
    _acc_id: pjsua_acc_id,
    call_id: pjsua_call_id,
    _rdata: *mut pjsip_rx_data,
) {
    print_log(LOG_LEVEL_1, "@@@@@@@@@ INCOMING! @@@@@@@@@@@@@@");
    let mut ci = MaybeUninit::<pjsua_call_info>::uninit();
    pjsua_call_get_info(call_id, ci.as_mut_ptr());

    /* Automatically answer incoming calls with 200/OK */
    pjsua_call_answer(call_id, 200, ptr::null(), ptr::null());
}

fn main() {
    unsafe {
        let mut _status = pjsua_create();

        let mut cfg = MaybeUninit::<pjsua_config>::uninit();
        pjsua_config_default(cfg.as_mut_ptr());
        let cfg_ptr = cfg.assume_init_mut();

        cfg_ptr.cb.on_incoming_call = Some(on_incoming_call);

        let mut log_cfg = MaybeUninit::<pjsua_logging_config>::uninit();
        pjsua_logging_config_default(log_cfg.as_mut_ptr());
        pj_log_set_level(5);
        let log_cfg = log_cfg.assume_init();

        _status = pjsua_init(cfg_ptr, &log_cfg, ptr::null());

        let mut t_cfg = MaybeUninit::<pjsua_transport_config>::uninit();
        pjsua_transport_config_default(t_cfg.as_mut_ptr());
        let t_cfg_ptr = t_cfg.assume_init_mut();

        t_cfg_ptr.port = 0;

        let mut transport_id = 0 as c_int;

        _status = pjsua_transport_create(
            pjsip_transport_type_e_PJSIP_TRANSPORT_UDP,
            t_cfg_ptr,
            &mut transport_id,
        );

        _status = pjsua_start();

        let mut acc_cfg = MaybeUninit::<pjsua_acc_config>::uninit();
        pjsua_acc_config_default(acc_cfg.as_mut_ptr());
        let acc_cfg_ptr = acc_cfg.assume_init_mut();

        let id =
            CString::new(&*format!("sip:{}@{}", SIP_USER, SIP_DOMAIN)).expect(CSTRING_NEW_FAILED);
        acc_cfg_ptr.id = pj_str(id.as_ptr() as *mut i8);

        let uri = CString::new(&*format!("sip:{}", SIP_DOMAIN)).expect(CSTRING_NEW_FAILED);
        acc_cfg_ptr.reg_uri = pj_str(uri.as_ptr() as *mut i8);

        acc_cfg_ptr.cred_count = 1;

        let sip_domain = CString::new("*").expect(CSTRING_NEW_FAILED);
        acc_cfg_ptr.cred_info[0].realm = pj_str(sip_domain.as_ptr() as *mut i8);

        let digest = CString::new("digest").expect(CSTRING_NEW_FAILED);
        acc_cfg_ptr.cred_info[0].scheme = pj_str(digest.as_ptr() as *mut i8);

        let username = CString::new(SIP_USER).expect(CSTRING_NEW_FAILED);
        acc_cfg_ptr.cred_info[0].username = pj_str(username.as_ptr() as *mut i8);

        acc_cfg_ptr.cred_info[0].data_type =
            pjsip_cred_data_type_PJSIP_CRED_DATA_PLAIN_PASSWD as i32;

        let password = CString::new(SIP_PASSWD).expect(CSTRING_NEW_FAILED);
        acc_cfg_ptr.cred_info[0].data = pj_str(password.as_ptr() as *mut i8);

        let mut acc_id = MaybeUninit::<pjsua_acc_id>::uninit();

        _status = pjsua_acc_add(
            acc_cfg_ptr,
            pj_constants__PJ_TRUE as i32,
            acc_id.as_mut_ptr(),
        );

        print_log(5, "@@@@@@@@@@@@ sleeping....");

        pj_thread_sleep(10000);

        print_log(5, "@@@@@@@@@@@@ wake....");

        /* Destroy pjsua */
        pjsua_destroy();
    }
}
