use std::env;

mod batwatcher;
mod bus;
mod nmwatcher;
mod pulsewatcher;
mod musicwatcher;
mod i3watcher;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let arg = &args[1];
        match &arg[..] {
            "nmwatcher" => nmwatcher::nmwatcher(),
            "batwatcher" => batwatcher::batwatcher(),
            "pulsewatcher" => pulsewatcher::pulsewatcher(),
            "musicwatcher" => musicwatcher::musicwatcher(),
            "i3watcher" => i3watcher::i3watcher(),
            _ => {}
        };
    }
}
