use pjsua::*;
use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    os::raw::c_int,
    process::exit,
    ptr::null,
};

pub enum LogLevel {
    LogLevel1 = 0,
    LogLevel2 = 1,
    LogLevel3 = 2,
    LogLevel4 = 3,
    LogLevel5 = 4,
}

pub fn init() {
    unsafe {
        let mut _status = pjsua_create();

        let mut cfg = MaybeUninit::<pjsua_config>::uninit();
        pjsua_config_default(cfg.as_mut_ptr());
        let cfg_ptr = cfg.assume_init_mut();

        cfg_ptr.cb.on_incoming_call = Some(on_incoming_call);
        cfg_ptr.cb.on_call_state = Some(on_call_state);
        cfg_ptr.cb.on_call_media_state = Some(on_call_media_state);

        let mut log_cfg = MaybeUninit::<pjsua_logging_config>::uninit();
        pjsua_logging_config_default(log_cfg.as_mut_ptr());
        let filename = CString::new("log.log").expect("CSTRING NEW ERROR");
        let mut lgcfg = *log_cfg.as_mut_ptr();
        lgcfg.log_filename = pj_str(filename.as_ptr() as *mut i8);
        pj_log_set_level(5);

        //let log_cfg = log_cfg.assume_init();

        _status = pjsua_init(cfg_ptr, &lgcfg, null());

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
    }
}

pub fn error_exit(msg: &str, status: pj_status_t) {
    unsafe {
        let file = CString::new("APP").expect("CSTRING_NEW_FAILED");
        let cmsg = CString::new(msg).expect("CSTRING_NEW_FAILED");
        pjsua_perror(file.as_ptr(), cmsg.as_ptr(), status);
        pjsua_destroy();
    }
    exit(1);
}

/**
 * print_log
 */
pub fn print_log(level: LogLevel, msg: &str) {
    let cmsg = CString::new(msg).expect("CSTRING_NEW_FAILED");
    let dummfmt = CString::new("").expect("CSTRING_NEW_FAILED");
    let file = CString::new("APP").expect("CSTRING_NEW_FAILED");
    unsafe {
        match level {
            LogLevel::LogLevel1 => pj_log_1(file.as_ptr(), cmsg.as_ptr(), dummfmt.as_ptr()),
            LogLevel::LogLevel2 => pj_log_2(file.as_ptr(), cmsg.as_ptr(), dummfmt.as_ptr()),
            LogLevel::LogLevel3 => pj_log_3(file.as_ptr(), cmsg.as_ptr(), dummfmt.as_ptr()),
            LogLevel::LogLevel4 => pj_log_4(file.as_ptr(), cmsg.as_ptr(), dummfmt.as_ptr()),
            LogLevel::LogLevel5 => pj_log_5(file.as_ptr(), cmsg.as_ptr(), dummfmt.as_ptr()),
        };
    }
}

/**
 * Callback called by the library upon receiving incoming call
 */
pub extern "C" fn on_incoming_call(
    _acc_id: pjsua_acc_id,
    call_id: pjsua_call_id,
    _rdata: *mut pjsip_rx_data,
) {
    print_log(LogLevel::LogLevel1, "@@@@@@@@@ INCOMING! @@@@@@@@@@@@@@");
    let mut ci = MaybeUninit::<pjsua_call_info>::uninit();
    unsafe {
        pjsua_call_get_info(call_id, ci.as_mut_ptr());

        /* Automatically answer incoming calls with 200/OK */
        pjsua_call_answer(call_id, 200, null(), null());
    }
}

/**
 * on_call_state callback
 */
pub extern "C" fn on_call_state(call_id: pjsua_call_id, e: *mut pjsip_event) {
    let mut ci = MaybeUninit::<pjsua_call_info>::uninit();
    unsafe {
        pjsua_call_get_info(call_id, ci.as_mut_ptr());
        let state_text = CStr::from_ptr((*ci.as_mut_ptr()).state_text.ptr)
            .to_str()
            .expect("CSTRING ERROR!");
        print_log(
            LogLevel::LogLevel1,
            &format!("@@@@@ Call {} sate={}", call_id, state_text),
        );
    }
}

pub extern "C" fn on_call_media_state(call_id: pjsua_call_id) {
    let mut ci = MaybeUninit::<pjsua_call_info>::uninit();
    unsafe {
        pjsua_call_get_info(call_id, ci.as_mut_ptr());
        let ci_hontai = *ci.as_ptr();
        if ci_hontai.media_status == pjsua_call_media_status_PJSUA_CALL_MEDIA_ACTIVE {
            pjsua_conf_connect(ci_hontai.conf_slot, 0);
            pjsua_conf_connect(0, ci_hontai.conf_slot);
        }
    }
}

pub fn make_call(acc_id: pjsua_acc_id, uri: pj_str_t) {
    unsafe {
        let v = std::ptr::null_mut();
        let mut dummy: i32 = 0;
        {
            let rdummy = &mut dummy;
            let status = pjsua_call_make_call(acc_id, &uri, null(), v, null(), rdummy);
            if status as u32 != pj_constants__PJ_SUCCESS {
                error_exit("Error making call", status);
            }
        }
    }
}

pub fn account_add(user: &str, pass: &str, domein: &str) -> i32 {
    let mut acc_id_ret: pjsua_acc_id = 0;
    unsafe {
        let mut acc_cfg = MaybeUninit::<pjsua_acc_config>::uninit();
        pjsua_acc_config_default(acc_cfg.as_mut_ptr());
        let acc_cfg_ptr = acc_cfg.assume_init_mut();

        let id = CString::new(&*format!("sip:{}@{}", user, domein)).expect("CSTRING_NEW_FAILED");
        acc_cfg_ptr.id = pj_str(id.as_ptr() as *mut i8);

        let uri = CString::new(&*format!("sip:{}", domein)).expect("CSTRING_NEW_FAILED");
        acc_cfg_ptr.reg_uri = pj_str(uri.as_ptr() as *mut i8);

        acc_cfg_ptr.cred_count = 1;

        let sip_domain = CString::new("*").expect("CSTRING_NEW_FAILED");
        acc_cfg_ptr.cred_info[0].realm = pj_str(sip_domain.as_ptr() as *mut i8);

        let digest = CString::new("digest").expect("CSTRING_NEW_FAILED");
        acc_cfg_ptr.cred_info[0].scheme = pj_str(digest.as_ptr() as *mut i8);

        let username = CString::new(user).expect("CSTRING_NEW_FAILED");
        acc_cfg_ptr.cred_info[0].username = pj_str(username.as_ptr() as *mut i8);

        acc_cfg_ptr.cred_info[0].data_type =
            pjsip_cred_data_type_PJSIP_CRED_DATA_PLAIN_PASSWD as i32;

        let password = CString::new(pass).expect("CSTRING_NEW_FAILED");
        acc_cfg_ptr.cred_info[0].data = pj_str(password.as_ptr() as *mut i8);

        let mut acc_id = MaybeUninit::<pjsua_acc_id>::uninit();

        let _status = pjsua_acc_add(
            acc_cfg_ptr,
            pj_constants__PJ_TRUE as i32,
            acc_id.as_mut_ptr(),
        );

        acc_id_ret = *acc_id.as_mut_ptr();
        print_log(LogLevel::LogLevel1, &format!("@@@@ acc_id={}", acc_id_ret));
    }
    return acc_id_ret;
}

pub fn callto(caller: i32, uri: &str) {
    unsafe {
        let dsturi = CString::new(uri).expect("CSTRING FAILED");
        let uri_str = pj_str(dsturi.as_ptr() as *mut i8);
        make_call(caller, uri_str);

        pj_thread_sleep(10000);

        print_log(LogLevel::LogLevel1, "@@@@@@@@@@ END!");
    }
}

pub fn destroy() {
    unsafe {
        /* Destroy pjsua */
        pjsua_destroy();
    }
}
