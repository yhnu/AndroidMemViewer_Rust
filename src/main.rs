use regex::{Match, Regex};
use std::io;
use std::sync::mpsc::channel;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::{process::Command, str};
enum Req {
    Start,
    Quit,
}

enum Res {
    Start,
}

struct Device {
    pub sn: String,
    pub status: String,
}

impl Device {
    pub fn new(sn: String, status: String) -> Device {
        Device { sn, status }
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
}

fn adb_current_app(adb: &str) -> String {
    let out = Command::new(adb)
        .arg("shell")
        .arg("dumpsys activity activities|grep mResumedActivity")
        .output()
        .unwrap();
    println!("{}", String::from_utf8(out.stderr).unwrap());

    // "mResumedActivity: ActivityRecord{a1ea80c u0 com.tencent.mm/.ui.LauncherUI t2683}"
    let p = String::from_utf8(out.stdout).unwrap();

    let re = Regex::new(r"com.[a-z0-9]*.[a-z0-9]*").unwrap();
    let cap = re.captures(p.as_str()).unwrap();
    cap[0].to_string()
}

fn adb_devices(adb: &str) -> Result<Vec<Device>, &'static str> {
    let mut devices: Vec<Device> = Vec::new();
    let out = Command::new(adb).arg("devices").output().unwrap().stdout;
    let device_re: Regex = Regex::new(r"^[0-9a-z]").unwrap();
    for line in std::str::from_utf8(&out).unwrap().split("\r\n") {
        let items: Vec<&str> = line.split("\t").collect();
        println!("{:?}", items);
        if device_re.is_match(items[0]) {
            devices.push(Device::new(items[0].to_string(), items[1].to_string()));
        }
        // slice
        // x str
        // &str
        // String
        // &String
    }
    Ok(devices)
}

fn main() {
    let adb = adb();
    let devices = adb_devices(&adb).unwrap();
    let package_name = adb_current_app(&adb);
    let (res_sender, res_receiver) = channel();
    let (cmd_sender, cmd_receiver) = channel();
    println!("{:#?}", package_name);

    // Spawn off an expensive computation
    let adb_thread = thread::spawn(move || {
        let mut sta = 0;
        loop {
            let cmd: Result<Req, TryRecvError> = cmd_receiver.try_recv();
            match cmd {
                Err(err) => {}
                Ok(req) => {
                    match req {
                        Req::Start => {
                            println!("xxxxxxxxxx");
                            sta = 1;
                            // println!("{:?}", output);
                            res_sender.send(Res::Start).unwrap();
                        }
                        Req::Quit => {
                            break;
                        }
                        _ => (),
                    }
                }
            }

            if sta == 1 {
                let output = Command::new("adb")
                    .arg("shell")
                    .arg(format!("dumpsys meminfo {}", package_name))
                    .output()
                    .unwrap();

                let o = String::from_utf8(output.stdout);
                println!("{}", o.unwrap().as_str());
            }

            // let o2 = output.stdout.as_slice();
            // println!("{}", o2);
            // let o = String::from(o2);
            // sender.send(o).unwrap();
            // sender.send(output.status).unwrap();
            // }
            // }
            // break;
        }
        // println!("AdbThread Exit")
    });

    // cmd_sender.send(Req::Start).unwrap();

    // loop {
    //     // let cmd = Req::Start;
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
    let mut cmd: String = "".to_string();
    loop {
        println!("Please Input Command:");
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
        println!("{:?}", cmd);
        match cmd.as_str() {
            "s\r\n" => {
                println!("Start Dump Mem Info");
                cmd_sender.send(Req::Start).unwrap_or_else(|err| {
                    println!("{:#?}", err);
                });
            }
            _ => {
                cmd_sender.send(Req::Quit).unwrap();
                break;
            }
        }
    }

    // assert_eq!(b"Hello world\n", output.stdout.as_slice());
    adb_thread.join().unwrap_or_else(|op| {
        panic!("Application Quit!");
    });
    println!("Application Quit!");
}
