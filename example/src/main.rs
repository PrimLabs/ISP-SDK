mod test_isp;
mod test_isp_certified_log;
use std::env;
use std::process;

struct Config {
    pub test_module: String,
    pub test_function: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            test_module: args[1].clone(),
            test_function: args[2].clone(),
        })
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("config arguments error {:?}", err);
        process::exit(1);
    });
    run_test(config).await;
}

async fn run_test(args: Config) {
    if args.test_module == "isp".to_string() {
        println!("test_module : {:?}", args.test_module);
        println!("test_function : {:?}\n", args.test_function);
        if args.test_function == "get_user_icsps".to_string() {
            test_isp::get_user_icsps().await;
        } else if args.test_function == "get_sub_account".to_string() {
            test_isp::get_sub_account().await;
        } else if args.test_function == "get_isp_admins".to_string() {
            test_isp::get_isp_admins().await;
        } else if args.test_function == "create_icsp".to_string() {
            test_isp::create_icsp().await;
        } else if args.test_function == "top_up_icsp".to_string() {
            test_isp::top_up_icsp().await;
        } else if args.test_function == "get_bucket_of_file".to_string() {
            test_isp::get_bucket_of_file().await;
        } else if args.test_function == "get_icsp_buckets".to_string() {
            test_isp::get_icsp_buckets().await;
        } else if args.test_function == "get_icsp_admins".to_string() {
            test_isp::get_icsp_admins().await;
        } else if args.test_function == "store_files".to_string() {
            test_isp::store_files().await;
        } else if args.test_function == "store_file".to_string() {
            test_isp::store_file().await;
        } else if args.test_function == "store_str".to_string() {
            test_isp::store_str().await;
        } else if args.test_function == "get_file".to_string() {
            test_isp::get_file().await;
        } else if args.test_function == "add_icsp_admin".to_string() {
            test_isp::add_icsp_admin().await;
        } else if args.test_function == "delete_icsp_admin".to_string() {
            test_isp::delete_icsp_admin().await;
        } else if args.test_function == "top_up_icsp_with_xtc".to_string() {
            test_isp::top_up_icsp_with_xtc().await;
        } else if args.test_function == "get_user_sub_account_icp_balance".to_string() {
            test_isp::get_user_sub_account_icp_balance().await;
        } else if args.test_function == "transfer_out_user_sub_account_icp".to_string() {
            test_isp::transfer_out_user_sub_account_icp().await;
        } else if args.test_function == "get_cycle_balance".to_string() {
            test_isp::get_cycle_balance().await;
        } else if args.test_function == "get_all_ic_file_key".to_string() {
            test_isp::get_all_ic_file_key().await;
        } else if args.test_function == "get_file_info".to_string() {
            test_isp::get_file_info().await;
        } else if args.test_function == "delete_file".to_string() {
            test_isp::delete_file().await;
        } else if args.test_function == "top_up_bucket".to_string() {
            test_isp::top_up_bucket().await;
        } else if args.test_function == "get_icsp_version".to_string() {
            test_isp::get_icsp_version().await;
        } else if args.test_function == "get_ic_file_numbers".to_string() {
            test_isp::get_ic_file_numbers().await;
        } else if args.test_function == "get_field_file_infos".to_string() {
            test_isp::get_field_file_infos().await;
        } else if args.test_function == "replace_str".to_string() {
            test_isp::replace_str().await;
        } else if args.test_function == "get_isp_version".to_string() {
            test_isp::get_isp_version().await;
        } else if args.test_function == "store_file_by_key".to_string() {
            test_isp::store_file_by_key().await;
        }
    } else if args.test_module == "isp_certified_log" {
        println!("test_module : {:?}", args.test_module);
        println!("test_function : {:?}\n", args.test_function);
        if args.test_function == "get_buckets".to_string() {
            test_isp_certified_log::get_buckets().await;
        } else if args.test_function == "get_log_num".to_string() {
            test_isp_certified_log::get_log_num().await;
        } else if args.test_function == "get_logs".to_string() {
            test_isp_certified_log::get_logs().await;
        } else if args.test_function == "get_admins".to_string() {
            test_isp_certified_log::get_admins().await;
        } else if args.test_function == "store".to_string() {
            test_isp_certified_log::store().await;
        } else if args.test_function == "add_admin".to_string() {
            test_isp_certified_log::add_admin().await;
        } else if args.test_function == "delete_admin".to_string() {
            test_isp_certified_log::delete_admin().await;
        }
    }
}
