use std::env;

mod batwatcher;
mod bus;
mod nmwatcher;
mod pulsewatcher;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let arg = &args[1];
        match &arg[..] {
            "nmwatcher" => nmwatcher::nmwatcher(),
            "batwatcher" => batwatcher::batwatcher(),
            "pulsewatcher" => pulsewatcher::pulsewatcher(),
            _ => {}
        };
    }
}
