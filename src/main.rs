
use std::ffi::CString;
use rust_wiredtiger::wiredtiger::api::*;

pub fn main() {

    const WT_HOME: &str = "./WT_TEST";
    setup_home(WT_HOME);

    unsafe {
        let mut conn_ret : *mut WT_CONNECTION = std::ptr::null_mut();
        let mut session_ret : *mut WT_SESSION = std::ptr::null_mut();
        let mut cursor_ret : *mut WT_CURSOR = std::ptr::null_mut();

        let home_c_str =CString::new(WT_HOME).expect("CString::new failed");
        let config =  CString::new("create").expect("CString::new failed");
        let uri =  CString::new("table:store").expect("CString::new failed");
        let uri2 =CString::new("table:store2").expect("CString::new failed");

        println!("Opening wiredtiger connection..");
        wiredtiger_open(home_c_str.as_ptr(), std::ptr::null_mut(), config.as_ptr(), &mut conn_ret);
        let conn =  &mut *conn_ret;
        

        println!("Opening a session and creating tables..");
        conn.open_session(std::ptr::null_mut(), std::ptr::null_mut(), &mut session_ret);
        let session =  &mut *session_ret;
        let create_config = CString::new("key_format=S,value_format=S").expect("CString::new failed");
        session.create(uri.as_ptr(), create_config.as_ptr());
        session.create(uri2.as_ptr(), create_config.as_ptr());

        session.open_cursor(uri.as_ptr(), std::ptr::null_mut(),std::ptr::null_mut(), &mut cursor_ret);
        let cursor =  &mut *cursor_ret;

        println!("Inserting K/V pair..");
        for i in 0..1000000 {
            let key = CString::new(format!("KEY{}", i)).expect("CString::new failed");
            let value = CString::new(format!("VALUE{}", i)).expect("CString::new failed");
        
            cursor.set_key(key.as_ptr());
            cursor.set_value(value.as_ptr());
            cursor.insert();
        }

        println!("Closing cursor, session and connection..");
        cursor.close();
        session.close(std::ptr::null_mut());
        conn.close(std::ptr::null_mut());
    }

   
}
