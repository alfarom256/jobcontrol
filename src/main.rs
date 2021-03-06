extern crate getopts;

use getopts::Options;
use std::env;

mod lib;



/*
args: 
-p 1,2,3,4,5
    process IDs to interact with
-n <bytes_per_second>
    restrict network to N bytes per second
-c <100.0 - 0.01>
    Max CPU percenta
-q 
    quiet
-d 
    debug
*/


fn print_help(prog : &str, opts: Options){
    let fmt = format!("Usage: {} -p/-pids 1,2,3,4 -c 0.01 -n", prog);
    print!("{}", opts.usage(&fmt));
}

fn main(){
   
    //https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/arg/getopts.html
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    let ref prog = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "pids", "PID list to interact with", "4,1013,139213,400");
    opts.optopt("c", "cpu", "max CPU usage for all processes (must be between 100.0 and 0.01)", "0.01");
    opts.optopt("n", "netrate", "attempt to set the bytes per second of network traffic for all processes ( >= W10, Server 2016 ONLY )", "50");
    opts.optflag("s", "skip", "skip processes already in jobs instead of terminating");
    opts.optflag("h", "help", "display help and usage");
    
    let matches = match opts.parse(args){
        Ok(m) => {m},
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_help(prog, opts);
        return;
    }

    let do_skip = matches.opt_present("s");

    if do_skip {
        println!("Not terminating on failure to assign process to job...");
    }

    // get the list of pids
    let str_pids = match matches.opt_str("p"){
        Some(p_str) => p_str,
        None => {panic!("Must supply a list of PIDs");}
    };
    let s_pid_list : Vec<&str> = str_pids.split(",").collect::<Vec<&str>>();
    let mut pid_list : Vec<u32> = Vec::new();
    for s_pid in s_pid_list.into_iter() {
         match s_pid.parse::<u32>(){
            Ok(x) => pid_list.push(x),
            Err(_) => {println!("Error, invalid PID: {}", s_pid); return;}
        }
    }

    let cpu_pct : f32 = match matches.opt_str("c"){
        Some(x) => {
            match x.parse::<f32>(){
                Ok(x) => if x >= 0.01f32 {x} else {println!("Error, invalid cpu control rate: {}", x); return;},
                Err(x) => {println!("Error, invalid cpu control rate: {}", x); return;}
            }
        },
        None => 0f32
    };

    let net_ctl : u32 = match matches.opt_str("n"){
        Some(x) => {
            match x.parse::<u32>(){
                Ok(x) => if x >= 1 {x} else { println!("Error, invalid net control rate (must be >=1): {}", x); return;},
                Err(x) => { println!("Error, invalid net control rate (must be >=1): {}", x); return; }
            }
        },
        None => 0
    };

    if net_ctl > 0 {
        println!("setting net control rate to {}", net_ctl);
    }

    if cpu_pct > 0f32 {
        println!("setting CPU max rate percent to {}%", cpu_pct);
    }

   match lib::assign_and_process_job(pid_list, net_ctl, cpu_pct, do_skip){
       Ok(_) => {},
       Err(x) => {println!("Error: {:?}", x)}
   };  
}