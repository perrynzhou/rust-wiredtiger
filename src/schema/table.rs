use crate::schema::context::Context;
use crate::wiredtiger::api::*;
use crate::wiredtiger::error::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::{mem, str};

pub struct Table {
    name: CString,
    uri: CString,
    table_format: CString,
    session: *mut __wt_session,
    cursor: *mut __wt_cursor,
}

impl Table {
    pub fn new(ctx: &mut Context, name: &str, table_format: &str) -> Result<Table, String> {
        unsafe {
            let mut session_ret: *mut WT_SESSION = std::ptr::null_mut();
            let mut cursor_ret: *mut WT_CURSOR = std::ptr::null_mut();

            let cname = CString::new(name).expect("CString::new failed");
            let uri = CString::new(format!("table:{}", name)).expect("CString::new failed");

            let conn = &mut *ctx.connection;

            let table_type = CString::new(table_format).expect("CString::new failed");
            let ret =
                conn.open_session(std::ptr::null_mut(), std::ptr::null_mut(), &mut session_ret);
            if ret != 0 {
                return Err(get_error(ret));
            }
            let session = &mut *session_ret;
            let mut ret = session.create(uri.as_ptr(), table_type.as_ptr());
            if ret != 0 {
                return Err(get_error(ret));
            }
            ret = session.open_cursor(
                uri.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut cursor_ret,
            );
            if ret != 0 {
                return Err(get_error(ret));
            }

            let cursor = &mut *cursor_ret;
            Ok(Table {
                name: cname,
                table_format: table_type,
                uri: uri,
                session: session,
                cursor: cursor,
            })
        }
    }

    pub fn set(&self, key: String, value: String) -> Result<(), String> {
        let ckey = CString::new(key).expect("CString::new failed");
        let cvalue = CString::new(value).expect("CString::new failed");
        unsafe {
            let cursor = &mut *self.cursor;
            cursor.set_key(ckey.as_ptr());

            cursor.set_value(cvalue.as_ptr());

            let ret = cursor.insert();
            if ret != 0 {
                return Err(get_error(ret));
            }
        }
        Ok(())
    }
    pub fn get(&self, key: String) ->Result<(),String> {
        let ckey = CString::new(key).expect("CString::new failed");
        unsafe {
            let cursor = &mut *self.cursor;
            let mut value: *mut c_char =mem::uninitialized();
            //MaybeUninit::uninit().as_mut_ptr();

            cursor.set_key(ckey.as_ptr());

            let ret = cursor.get_value(&mut value);
            if ret !=0 {
                return Err(get_error(ret));

            }
        }
        Ok(())
    }
    pub fn drop(&self) ->Result<(),String> {
        unsafe {
            let session = &mut *self.session;
            let flag = CString::new("force=true,remove_files=true").expect("CString::new failed");
            let  ret = session.drop(self.uri.as_ptr(), flag.as_ptr());
            if ret !=0 {
                return Err(get_error(ret));
            }
            if ret !=0 {
                return Err(get_error(ret));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
 //   use super::*;
    #[test]
    pub fn it_works() {
       // let mut i = 100;
        //let open_config =   "create,cache_size=4GB,session_max=50000,eviction=(threads_min=4,threads_max=8),log=(enabled=false),transaction_sync=(enabled=false),checkpoint_sync=true,checkpoint=(wait=10),statistics=(fast),statistics_log=(json,wait=1)";
       
        
    }
}
