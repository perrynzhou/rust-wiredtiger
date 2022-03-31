use crate::schema::context::Context;
use crate::wiredtiger::api::*;
use std::ffi::CString;

pub struct Table {
    name: CString,
    uri: CString,
    table_format: CString,
    session: *mut __wt_session,
    cursor: *mut __wt_cursor,
}

impl Table {
    pub fn new(ctx: &Context, name: &str, table_format: &str) -> Self {
        unsafe {
            let mut session_ret: *mut WT_SESSION = std::ptr::null_mut();
            let mut cursor_ret: *mut WT_CURSOR = std::ptr::null_mut();

            let cname = CString::new(name).expect("CString::new failed");
            let uri = CString::new(format!("table:{}", name)).expect("CString::new failed");

            let conn = &mut *ctx.connection;

            let table_type = CString::new(table_format).expect("CString::new failed");
            conn.open_session(std::ptr::null_mut(), std::ptr::null_mut(), &mut session_ret);
            let session = &mut *session_ret;
            session.create(uri.as_ptr(), table_type.as_ptr());
            session.open_cursor(
                uri.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut cursor_ret,
            );
            let cursor = &mut *cursor_ret;
            Self {
                name: cname,
                table_format: table_type,
                uri: uri,
                session: session,
                cursor: cursor,
            }
        }
    }
    pub fn drop(&self) {
        unsafe {
            let  session = &mut *self.session;
            let flag = CString::new("remove_files=true").expect("CString::new failed");
            session.drop(self.uri.as_ptr(), flag.as_ptr());
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn it_works() {
        let mut i = 100;
        let open_config =   "create,cache_size=4GB,session_max=50000,eviction=(threads_min=4,threads_max=8),log=(enabled=false),transaction_sync=(enabled=false),checkpoint_sync=true,checkpoint=(wait=10),statistics=(fast),statistics_log=(json,wait=1)";
        unsafe {
            let ctx = Context::new("./test", open_config);
            while i < 10 {
                let name = format!("Table-{}", i);
                let table = Table::new(&ctx, &name, "key_format=S,value_format=S");
                table.drop();
                i = i + 1;
            }
        }
    }
}
