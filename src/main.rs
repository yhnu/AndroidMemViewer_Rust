use regex::Regex;
// use std::sync::mpsc::channel;
// use std::thread;
use std::{process::Command, str};

enum CmdId {
    Start,
}

struct Devices {
    pub sn: String,
    pub status: String,
}

impl Devices {
    pub fn new(sn: &str, status: &str) -> Devices {
        Devices{
            sn: String::from(sn),
            status: String::from(status)
        }
    }
}


fn adb<'a>() -> &'a str {
    let mut ret = "adb";
    let output = Command::new("adb").arg("--version").output();
    match output {
        Ok(_) => {}
        Err(_) => ret = "./adb",
    }
    ret
    // String::from(ret)
}

fn adb_devices(adb: &str) -> Result<Vec<Devices>, &'static str> {
    let mut devices: Vec<Devices> = Vec::new();

    let out = Command::new(adb).arg("devices").output().unwrap().stdout;
    let device_re: Regex = Regex::new(r"^[0-9a-z]").unwrap();
    for line in std::str::from_utf8(&out).unwrap().split("\r\n") {
        let items : Vec<&str> = line.split("\t").collect();                
        println!("{:?}", items);
        if device_re.is_match(items[0]) {       

            devices.push(Devices::new(items[0], items[1]));
            // Ok(Devices::new(items[0], items[1]))
        }
    }
    Ok(devices)

    // match stdout {
    //     Ok(out) => {
    //         let o = str::from_utf8(out.stdout)

    //         let x = o.split("\r\n");
    //         //.unwrap().as_str();
    //         // println!("{:?}", o);
    //         // println!("{:?}", out.stdout);
    //         for line in x {
    //             let items: Vec<&str> = line.split("\t").collect();
    //             if device_re.is_match(items[0]) {
    //                 devices.push(Devices{
    //                     sn: items[0],
    //                     status: items[1]
    //                 });
    //             }
    //         }
    //     }
    //     Err(_) => {
    //         panic!("[ERROR] adb_devices");
    //     }
    // }
    // Ok(devices)
}

fn main() {
    let adb = adb();
    adb_devices(&adb);
    // let (sender, receiver) = channel();
    // let (cmd_sender, cmd_receiver) = channel();

    // Spawn off an expensive computation
    // thread::spawn(move || {
    //     loop {
    //         // let cmd: CmdId = cmd_receiver.try_recv().unwrap();
    //         // match cmd {
    //         // CmdId::Start => {
    //         let output = Command::new("adb2")
    //             .arg("shell ls /sdcard")
    //             .output();

    //         match output {
    //             Err(err) =>{
    //                 if err.kind() == NotFound {

    //                 }
    //             }
    //         }

    //         println!("{:?}", output);
    //         let o = String::from_utf8(output.stdout);
    //         // let o2 = output.stdout.as_slice();
    //         // println!("{}", o2);
    //         // let o = String::from(o2);
    //         sender.send(o).unwrap();
    //         // sender.send(output.status).unwrap();
    //         // }
    //         // }
    //         break;
    //     }
    //     println!("AdbThread Exit")
    // });

    // cmd_sender.send(CmdId::Start).unwrap();

    // loop {
    //     // let cmd = CmdId::Start;
    //     let rec = receiver.recv();
    //     match rec {
    //         Ok(_) => {
    //             println!("{:?}", rec);
    //         }
    //         Err(e) => {
    //             println!("err: {}", e);
    //             break;
    //         }
    //     }
    // }

    // assert_eq!(b"Hello world\n", output.stdout.as_slice());
    println!("Hello, world!");
}
