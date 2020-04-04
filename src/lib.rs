extern crate winapi;
use std::io::Error;
use self::winapi::um::winnt::{HANDLE};
use self::winapi::shared::minwindef::BOOL;


pub fn assign_and_process_job(pids: Vec<u32>, net_ctl : u32, cpu_pct : f32, dbg : bool) -> Result<u32, &'static str> {
    println!("d00t!");
    Ok(1)
}

fn open_process() -> Result<HANDLE, Error>{
    Ok(std::ptr::null_mut())
}

fn is_proc_in_job(){}

fn create_job_object(){}

fn assign_proct_to_job_object(){}
