use std::env;

mod batwatcher;
mod bus;
mod nmwatcher;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let arg = &args[1];
        match &arg[..] {
            "nmwatcher" => nmwatcher::nmwatcher(),
            "batwatcher" => batwatcher::batwatcher(),
            _ => {}
        };
    }
}
