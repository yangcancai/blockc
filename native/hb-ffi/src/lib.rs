#![allow(clippy::missing_safety_doc, clippy::not_unsafe_ptr_arg_deref)]

use allo_isolate::Isolate;
use atomic::Atomic;
use ffi_helpers::{null_pointer_check, NullPointer};
// use hb::ws::Ws;
use lazy_static::lazy_static;
// use std::error::Error;
use std::sync::atomic::Ordering;
use std::{ffi::CStr, io, os::raw};
use tokio::runtime::{Builder, Runtime};
static mut PORT_COBJECT: Atomic<Option<i64>> = Atomic::new(None);

lazy_static! {
    static ref RUNTIME: io::Result<Runtime> = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .core_threads(4)
        .thread_name("blockc")
        .build();
}

macro_rules! error {
    ($result:expr) => {
        error!($result, 0);
    };
    ($result:expr, $error:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                ffi_helpers::update_last_error(e);
                return $error;
            }
        }
    };
}

macro_rules! cstr {
    ($ptr:expr) => {
        cstr!($ptr, 0);
    };
    ($ptr:expr, $error:expr) => {{
        null_pointer_check!($ptr);
        error!(unsafe { CStr::from_ptr($ptr).to_str() }, $error)
    }};
}

macro_rules! runtime {
    () => {
        match RUNTIME.as_ref() {
            Ok(rt) => rt,
            Err(_) => {
                return 0;
            }
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn last_error_length() -> i32 {
    ffi_helpers::error_handling::last_error_length()
}

#[no_mangle]
pub unsafe extern "C" fn error_message_utf8(buf: *mut raw::c_char, length: i32) -> i32 {
    ffi_helpers::error_handling::error_message_utf8(buf, length)
}

#[no_mangle]
pub extern "C" fn load_page(port: i64, url: *const raw::c_char) -> i32 {
    let rt = runtime!();
    let url = cstr!(url);
    let t = Isolate::new(port).task(hb::load_page(url));
    rt.spawn(t);
    1
}
pub async fn do_task() -> i32 {
    let mut i = 0;
    while i < 100 {
        unsafe {
            if let Some(port) = PORT_COBJECT.load(Ordering::Relaxed) {
                let rt = runtime!();
                let t = Isolate::new(port).task(async move { i + 1 });
                rt.spawn(t);
                i = i + 1;
                std::thread::sleep(std::time::Duration::from_millis(1000))
            } else {
                break;
            }
        }
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn start_timer(port: i64) -> i32 {
    PORT_COBJECT.store(Some(port), Ordering::Relaxed);
    let rt = runtime!();
    rt.spawn(do_task());
    1
}
// #[no_mangle]
// pub unsafe extern "C" fn start_ws(url: *const raw::c_char) -> *mut Ws {
//     let url = CStr::from_ptr(url).to_str().unwrap();
//     let ws = Ws::new_ssl(url);
//     if let Ok(rs) = ws {
//         let b = Box::new(rs);
//         Box::into_raw(b)
//     } else {
//         std::ptr::null_mut()
//     }
// }
// #[no_mangle]
// pub unsafe extern "C" fn is_alive(ws: *mut Ws) -> bool {
//     let mut b = Box::from_raw(ws);
//     b.is_alive()
// }
// #[no_mangle]
// pub unsafe extern "C" fn stop_ws(ws: *mut Ws) {
//     let mut b = Box::from_raw(ws);
//     b.close()
// }
