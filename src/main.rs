extern crate getopts;
use getopts::Options;

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

use std::env;

fn print_help(prog : &str, opts: Options){
    let fmt = format!("Usage: {} -p/-pids 1,2,3,4 -c 0.01 -n", prog);
    print!("{}", opts.usage(&fmt));
}

fn main(){
    let mut quiet : bool = false;
    let mut dbg : bool = false;

    //https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/arg/getopts.html
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    let ref prog = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "pids", "PID list to interact with", "4,1013,139213,400");
    opts.optopt("c", "cpu", "max CPU usage for all processes (must be between 100.0 and 0.01)", "0.01");
    opts.optopt("n", "netrate", "attempt to set the bytes per second of network traffic for all processes ( >= W10, Server 2016 ONLY )", "50");

    opts.optflag("q", "quiet", "do not display output");
    opts.optflag("d", "debug", "display debug output");
    opts.optflag("h", "help", "display help and usage");
    
    let matches = match opts.parse(args){
        Ok(m) => {m},
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_help(prog, opts);
        return;
    }

    quiet = matches.opt_present("q");    
    dbg = matches.opt_present("d");

    if quiet && dbg {
        panic!("Cannot have debug *and* quiet output...");
    }
    
    println!("woooo!");
}