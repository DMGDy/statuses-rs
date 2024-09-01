use std::error::Error;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;
use std::process;
use std::fmt;

/* TODO: function to print local IPv4 address
 *      - could be its own window or as tooltip (easier)
*/

struct MemInfo {
    total: f64,
    used: f64
}

impl MemInfo {
    pub fn new() -> Result<MemInfo, Box<dyn Error>> {

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
    pub fn get_total(&self) -> f64{
        return self.total
    }
    
    pub fn set_total(&mut self,num: f64) {
        self.total = num
    }

}

impl fmt::Display for MemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:.2}/{:.2}  ",self.used,self.total)
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args[1].len() < 1 {
        println!("Usage: status-rs [options]");
        process::exit(1)
    }
    let status = args[1].as_str();

    match status {
        "--mem" => {
            
            if args.len() > 2 {
                if args[2].as_str() == "-i" {
                    print!("");
                    process::exit(0)
                }
            }
           
            let mut mem = MemInfo::new().unwrap();
            println!("{}Gb",MemInfo::new().unwrap());
            mem.set_total(10.0);
            println!(" Total {}",mem.get_total());

        }
        _ => {
            println!("Usage: status-rs [options]");
            process::exit(1)
        }
    }
    Ok(())
}
