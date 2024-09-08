pub mod condition;
pub mod query;
pub mod libs;

use libs::error::print_err;
use libs::exec::exec_query;
use libs::parsing::build_query;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Uso: cargo run -- ruta/a/tablas \"query\"");
    } else {
        let path: &String = &args[1]; 
        let text_query: &String = &args[2];   
        match build_query(text_query, path) {                             
            Ok(x) => exec_query(x),                          
            Err(x) => print_err(x)                  
        }
    }
}
