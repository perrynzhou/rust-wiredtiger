use crate::wiredtiger::api::*;
use std::ffi::CString;

pub struct Context {
   pub  home: CString,
   pub  connection: *mut __wt_connection,
}

impl Context {
    pub unsafe fn new(home : &str, open_config: &str) -> Self {
        let mut conn_ret: *mut WT_CONNECTION = std::ptr::null_mut();
        let home_c_str = CString::new(home).expect("CString::new failed");

        // let open_config =    CString::new("create,cache_size=4GB,session_max=50000,eviction=(threads_min=4,threads_max=8),log=(enabled=false),transaction_sync=(enabled=false),checkpoint_sync=true,checkpoint=(wait=10),statistics=(fast),statistics_log=(json,wait=1)").expect("CString::new failed");
        let open_config = CString::new(open_config).expect("CString::new failed");

        wiredtiger_open(
            home_c_str.as_ptr(),
            std::ptr::null_mut(),
            open_config.as_ptr(),
            &mut conn_ret,
        );
        let   conn = &mut *conn_ret;
        Self {
            home: home_c_str,
            connection: conn,
        }
    }
    pub unsafe fn close(&mut self) {
      let c1 = &mut *self.connection;
        c1.close(std::ptr::null_mut());
    }
}
