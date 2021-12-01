use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX:u16 = 65535;

struct Arguments{
    flag : String,
    ip_address: IpAddr,
    threads: u16,
}

impl Arguments{
    fn create_argument(args:&[String]) -> Result<Arguments,  &'static str >{
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        else if args.len() > 4 {
            return Err("Too Many Arguments");
        }

        let first_argument = args[1].clone();
        if let Ok(ip_address)  = IpAddr::from_str(&first_argument){
            return Ok(Arguments{
                flag : String::from(""),
                ip_address,
                threads : 4
            })
        }else{
            let flag =  args[1].clone();

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2{
                println!("Usage: This is how to use the Port Sniffing CLI Tool \n
                
                -j : Meant to select how many threads you want \n

                -h or -help : Meant to ask for help or get guidance on how to use this tool ");
                return Err("help");
            }
            else if  flag.contains("-h") || flag.contains("-help") {
                return Err("Too Many Arguments Passed");
            } 
            else{
                if flag.contains("-j"){
                    let ip_address =  match IpAddr::from_str(&args[3]){
                        Ok(s)=>s,
                        Err(_)=> return Err("not a valid IpAddress ; must be an IpV4  or IPV6")
                    };

                    let threads =  match args[2].parse::<u16>(){
                        Ok(s)=>s,
                        Err(_)=> return Err("failed to parse thread number")
                    };
                    return Ok(Arguments{threads, flag, ip_address});
                }else{
                    return Err("invalid Syntax");
                }
            }
        
        }
    }
}

fn scan(tx:Sender<u16>, start_port:u16, address : IpAddr, threads:u16){

    let mut port:u16 = start_port + 1;

    loop{
        match TcpStream::connect((address, port)){ 
            Ok(_)=>{
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap() 
            }
            Err(_)=>{}
        }

        if (MAX - port) <= threads{
            break;
        }
        
        port += threads
    
    }
    

}

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let program =  args[0].clone();

    let argument = Arguments::create_argument(&args).unwrap_or_else(
        |err|{
            if err.contains("help"){
                process::exit(0);
            }else{
                eprintln!("{}, causing the error : {}", program, err);
                process::exit(0);
            }
        }
    );
   
    let flag_used=  argument.flag;

    println!("{}", flag_used);
    
    let num_threads = argument.threads;

    let (tx,rx) = channel();

    for i in 0..num_threads{
        let tx = tx.clone();

        let address = argument.ip_address;

        thread::spawn(move||{
            scan(tx, i, address, num_threads);
        });
    }

    let mut out = vec![];
        drop(tx);

        for p in rx{
            out.push(p);
        }

        println!("");

        out.sort();

        for v in out{
            println!("{} is open ", v);
        }
        
}
