use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;

enum CmdId {
    Start,
}

fn main() {
    let (sender, receiver) = channel();
    let (cmd_sender, cmd_receiver) = channel();

    // Spawn off an expensive computation
    thread::spawn(move || {
        loop {
            // let cmd: CmdId = cmd_receiver.try_recv().unwrap();
            // match cmd {
            // CmdId::Start => {
            let output = Command::new("echo")
                .arg("Hello world")
                .output()
                .expect("Failed to execute command");

            let o = String::from_utf8(output.stdout);
            // let o2 = output.stdout.as_slice();
            // println!("{}", o2);
            // let o = String::from(o2);
            sender.send(o).unwrap();
            // sender.send(output.status).unwrap();
            // }
            // }
        }
    });

    cmd_sender.send(CmdId::Start).unwrap();

    loop {
        // let cmd = CmdId::Start;
        let rec = receiver.recv();

        match rec {
            Ok(_) => {
                println!("{:?}", rec);
            }
            Err(e) => {
                println!("err: {}", e);
            }
        }
    }

    // assert_eq!(b"Hello world\n", output.stdout.as_slice());
    // println!("Hello, world!");
}
