use rust_wiredtiger::schema::context::*;
use rust_wiredtiger::schema::table::*;
use rust_wiredtiger::wiredtiger::api::*;
pub fn main() {
    const WT_HOME: &str = "./WT_TEST";
    setup_home(WT_HOME);
    unsafe {
        let open_config =   "create,cache_size=4GB,session_max=50000,eviction=(threads_min=4,threads_max=8),log=(enabled=false),transaction_sync=(enabled=false),checkpoint_sync=true,checkpoint=(wait=10),statistics=(fast),statistics_log=(json,wait=1)";

        let ctx_result: Result<Context, String> = Context::new(WT_HOME, open_config);
        if ctx_result.is_ok() {
            let mut ctx: Context = ctx_result.unwrap();
            let mut tables:Vec<Table> = Vec::new();
            for i in 0..100 {
                let name = format!("schema-{}", i);
                let table_result = Table::new(&mut ctx, &name, "key_format=S,value_format=S");
                if table_result.is_ok() {
                    let  table = table_result.ok().unwrap();
                  
                    println!("create schmea = {} succ", name);
                   
                    tables.push(table) ;
                }
            }
           
            while let Some(table) = tables.pop() {
                // Prints 3, 2, 1
                let drop_result =  table.drop();
                    if drop_result.is_err() {
                        println!("drop table err:{:?}",drop_result.err());
                    }else {
                        println!("drop table succ");
                    }
            }

            ctx.close();

          
        }
    }
}
