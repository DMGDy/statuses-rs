use std::error::Error as StdError;
use std::io::{Write,BufReader,BufRead};
use std::os::unix::net::UnixStream;
use std::fs::File;
use std::env;
use std::process;
use std::fmt;
use nix::sys::socket::SockaddrLike;
use nix::sys::socket::AddressFamily;
use networkmanager::devices::{Device,Wireless};
use networkmanager::{NetworkManager,Error};
use dbus::blocking::Connection;

/* TODO: rewrite C modules in Rust
 *  * Battery
 *  * time/date
 *  
*/



fn print_ipv4() {
    let addrs = nix::ifaddrs::getifaddrs().unwrap();
    for addr in addrs {
        if addr.interface_name == WIFI_DEV {
            match addr.address {
                Some(address) =>  {
                    match address.family() {
                        Some(family) => {
                            if family == AddressFamily::Inet {
                                println!("{}",address.to_string().strip_suffix(":0").unwrap_or("N/A"));
                            }
                        },
                        None => {
                            println!("N/A");
                        }
                    }
                }
                None => println!("N/A"),
            }
        }
    }
}

const WIFI_DEV: &str = "wlp170s0";

fn print_wifi_strength() {
    let dbus_connection = Connection::new_system().unwrap();
    let nm = NetworkManager::new(&dbus_connection);
    let wifidev = nm.get_device_by_ip_iface(WIFI_DEV).unwrap();

    match wifidev {
        Device::WiFi(x) => {
            match Some(x.active_access_point().unwrap()) {
                Some(ap) => {
                    let ascii_strength =  match Some(ap.strength()) {
                        Some(Ok(strength)) => match strength {
                            0..=12 => "󰤯",
                            13..=25=>"󰤟",
                            26..=50 => "󰤢",
                            51..=75 => "󰤥",
                            76..=100 => "󰤨",
                            _ => "󰤫",
                        }
                        Some(Err(_)) | None =>"󱛏",
                    };
                    println!("{}",ascii_strength);
                }

                None => {
                    print!("󰤫");

                }
            }
        }
        _ => { 
                println!("󰤫");
        }
    }
}

fn print_ssid() -> Result<(), networkmanager::Error > {
    let dbus_connection = Connection::new_system().unwrap();
    let nm = NetworkManager::new(&dbus_connection);
    let wifidev = nm.get_device_by_ip_iface(WIFI_DEV).unwrap();

    match wifidev {
        Device::WiFi(x) => {
            match Some(x.active_access_point().unwrap()) {
                Some(ap) => {
                    let ssid = match Some(ap.ssid()) {
                        Some(Ok(ssid)) => ssid,
                        Some(Err(_)) | None => "Not Connected".to_owned(),

                    };
                    print!("{} ",ssid);
                },

                None => {
                    print!("Not Connected");

                }
            }
        }
        _ => { 
                println!("N/A");
        }
    }

    Ok(())
}

struct MemInfo {
    total: f64,
    used: f64
}

impl MemInfo {
    pub fn new() -> Result<MemInfo, Box<dyn StdError>> {

        let file = File::open("/proc/meminfo")?;
        let reader = BufReader::new(file);
        
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        let memtotal_kb: f64 = lines[0]
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()?;

        let memused_kb: f64 = lines[2]
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()?;

        let total = memtotal_kb / 1_048_576.0;
        let used = (memtotal_kb - memused_kb) / 1_048_576.0;
        
        Ok(MemInfo {
            total,
            used,
        })
    }

}

impl fmt::Display for MemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:.2}/{:.2}  ",self.used,self.total)
    }

}

fn main() -> Result<(), Box<dyn StdError>> {
    let args: Vec<String> = env::args().collect();

    if args[1].len() < 1 {
        println!("Usage: status-rs [options]");
        process::exit(1)
    }
    let status = args[1].as_str();

    match status {
        "--mem" => {
            println!("{}",MemInfo::new().unwrap());
        }
        "--wifi-info" => {
            print_ssid();
        }
        "--wifi-strength" => {
            print_wifi_strength();
        }
        "--ip" => {
            print_ipv4();
        }
        _ => {
            println!("Usage: {} [options]",args[0].as_str());
            process::exit(1)
        }
    }
    Ok(())
}
