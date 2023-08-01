use std::env;

mod nmwatcher;
mod bus;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        2 => {
            let arg = &args[1];
            let _ = match &arg[..] {
                "nmwatcher" => nmwatcher::nmwatcher(),
                _ => Ok(())
            };
        },
        _ => {}
    }

}