extern crate libpulse_binding as pulse;

use pulse::callbacks::ListResult;
use pulse::context::subscribe::{Facility, InterestMaskSet, Operation};
use pulse::context::{Context, FlagSet as ContextFlagSet};
use pulse::mainloop::standard::IterateResult;
use pulse::mainloop::standard::Mainloop;
use pulse::proplist::Proplist;
use pulse::sample::{Format, Spec};
use serde::Serialize;
use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Serialize, Debug)]
struct SinkState<'a> {
    volume_percent: u64,
    muted: bool,
    device_desc: Option<Cow<'a, str>>,
}

fn out_info(info: ListResult<&pulse::context::introspect::SinkInfo<'_>>) {
    if let ListResult::Item(i) = info {
        let volume_level: f64 = i.volume.get()[0].0.into();
        let volume_base: f64 = i.base_volume.0.into();
        let volume: u64 = ((volume_level / volume_base) * 100.0) as u64;

        let muted = i.mute;

        let port_desc = i
            .active_port
            .as_ref()
            .map(|port| port.description.to_owned().unwrap());

        let state = SinkState {
            device_desc: port_desc,
            muted,
            volume_percent: volume,
        };

        let mut stdout = std::io::stdout().lock();

        match serde_json::to_string(&state) {
            Ok(out) => {
                let _ = stdout.write_all(&[out.as_bytes(), b"\n"].concat());
                let _ = stdout.flush();
            }
            Err(e) => {
                eprintln!("Failed to serialize output: {}", e);
            }
        };
    }
}

pub fn pulsewatcher() {
    let spec = Spec {
        format: Format::S16NE,
        channels: 2,
        rate: 44100,
    };
    assert!(spec.is_valid());

    let mut proplist = Proplist::new().unwrap();
    proplist
        .set_str(
            pulse::proplist::properties::APPLICATION_NAME,
            "BartenderPulse",
        )
        .unwrap();

    let mainloop: Rc<RefCell<Mainloop>> = Rc::new(RefCell::new(
        Mainloop::new().expect("Failed to create mainloop"),
    ));

    let context: Rc<RefCell<Context>> = Rc::new(RefCell::new(
        Context::new_with_proplist(
            mainloop.borrow().deref(),
            "BartenderPulseContext",
            &proplist,
        )
        .expect("Failed to create new context"),
    ));

    context
        .borrow_mut()
        .connect(None, ContextFlagSet::NOFLAGS, None)
        .expect("Failed to connect context");

    // Wait for context to be ready
    loop {
        match mainloop.borrow_mut().iterate(false) {
            IterateResult::Quit(_) | IterateResult::Err(_) => {
                eprintln!("Iterate state was not success, quitting...");
                return;
            }
            IterateResult::Success(_) => {}
        }
        match context.borrow().get_state() {
            pulse::context::State::Ready => {
                break;
            }
            pulse::context::State::Failed | pulse::context::State::Terminated => {
                eprintln!("Context state failed/terminated, quitting...");
                return;
            }
            _ => {}
        }
    }

    context.borrow_mut().subscribe(InterestMaskSet::SINK, |s| {
        if !s {
            panic!("could not subscribe and hit that like button");
        }
    });

    // it works on my PC - Patrick
    context
        .borrow()
        .introspect()
        .get_sink_info_by_index(0, out_info);

    // Actual event detection
    // I have no idea if it can detect specific events, so this just fires every time the sink updates.
    {
        let cont = context.clone();
        context.borrow_mut().set_subscribe_callback(Some(Box::new(
            move |_: Option<Facility>, op: Option<Operation>, n: u32| {
                if op.unwrap() == Operation::Changed {
                    cont.borrow()
                        .introspect()
                        .get_sink_info_by_index(n, out_info);
                }
            },
        )));
    }

    let _ = mainloop.borrow_mut().run();

    // I sure hope this doesn't cause memory leaks or some shit

    // Clean shutdown
    // mainloop.borrow_mut().quit(Retval(0)); // uncertain whether this is necessary
}
