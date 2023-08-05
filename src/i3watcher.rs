use std::{io::Write, collections::HashMap};

use i3ipc::{I3Connection, I3EventListener, Subscription, event::Event};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Out {
    mode: String,
    workspaces: HashMap<String, WorkspaceOutput>
}

// Implementing my own struct because for some reason I can't
// implement serialize for external structs
#[derive(Debug, Serialize)]
struct WorkspaceOutput {
    num: i32,
    name: String,
    visible: bool,
    focused: bool,
    urgent: bool
}

fn get_workspaces(connection: &mut I3Connection) -> Option<Vec<WorkspaceOutput>>{
    // ig we're just gonna ignore possible errors for now
    if let Ok(w) = connection.get_workspaces() {
            let workspaces_out: Vec<WorkspaceOutput> = w.workspaces.iter().map(|w| {
                WorkspaceOutput{
                    num: w.num,
                    name: w.name.clone(),
                    visible: w.visible,
                    focused: w.focused,
                    urgent: w.urgent
                }
            }).collect();
        
            return Some(workspaces_out);
    }
    None
}

fn print_output(output: &mut Out, connection: &mut I3Connection) {
    let workspaces = get_workspaces(connection);

    output.workspaces.clear();

    for workspace in workspaces.unwrap() {
        output.workspaces.insert(workspace.num.to_string(), workspace);
    }

    let mut stdout = std::io::stdout().lock();

    match serde_json::to_string(&output) {
        Ok(out) => {
            let _ = stdout.write_all(&[out.as_bytes(), b"\n"].concat());
            let _ = stdout.flush();
        }
        Err(e) => {
            eprintln!("Failed to serialize output: {}", e);
        }
    };
}

pub fn i3watcher() {
    // let listener = I3EventListener::connect();

    let mut output = Out{
        mode: String::from("default"),
        workspaces: HashMap::new()
    };

    let mut connection = match I3Connection::connect() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return;
        }
    };

    let mut listener = match I3EventListener::connect() {
        Ok(mut listener) => {
            listener.subscribe(&[Subscription::Mode, Subscription::Workspace]).unwrap();
            listener
        },
        Err(e) => {
            println!("Failed to connect listener: {}", e);
            return;
        },
    };

    print_output(&mut output, &mut connection);

    for event in listener.listen() {
        match event {
            Ok(Event::ModeEvent(mode)) => {
                output.mode = mode.change;
                print_output(&mut output, &mut connection); 
            },
            Ok(Event::WorkspaceEvent(_)) => {
                print_output(&mut output, &mut connection)
            },
            _ => {}
        }
    }

}