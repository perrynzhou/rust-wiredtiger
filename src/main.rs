use rust_wiredtiger::schema::table::*;
use rust_wiredtiger::schema::context::*;
use rust_wiredtiger::wiredtiger::api::*;
pub fn main() {
    const WT_HOME: &str = "./WT_TEST";
    setup_home(WT_HOME);
    unsafe {
    let open_config =   "create,cache_size=4GB,session_max=50000,eviction=(threads_min=4,threads_max=8),log=(enabled=false),transaction_sync=(enabled=false),checkpoint_sync=true,checkpoint=(wait=10),statistics=(fast),statistics_log=(json,wait=1)";

    let mut ctx =  Context::new(WT_HOME,open_config);  

    for i in 0..100 {
        let name = format!("schema-{}", i);
        println!("create schmea = {}", name);
        let table =Table::new(&ctx, &name, "key_format=S,value_format=S");
        table.drop();
    }
    
    ctx.close();
}
}
