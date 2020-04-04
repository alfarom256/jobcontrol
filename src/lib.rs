extern crate winapi;
use std::io::Error;
use self::winapi::um::winnt::{HANDLE, PROCESS_SET_QUOTA, PROCESS_TERMINATE, PROCESS_QUERY_LIMITED_INFORMATION};
use self::winapi::um::processthreadsapi::{OpenProcess, };
use self::winapi::shared::minwindef::{BOOL, TRUE, FALSE};


pub fn assign_and_process_job(pids: Vec<u32>, net_ctl : u32, cpu_pct : f32, dbg : bool) -> Result<u32, Error> {
    let mut v_proc_handle : Vec<HANDLE> = Vec::new();
    for pid in pids.into_iter(){
        let h_proc = match open_process(pid){
            Ok(x) => x,
            Err(x) => return Err(x)
        };
        v_proc_handle.push(h_proc);
    }
    Ok(1)
}

fn open_process(pid : u32) -> Result<HANDLE, Error>{
    let mut hProc : HANDLE = std::ptr::null_mut();
    unsafe {
        hProc = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_TERMINATE | PROCESS_SET_QUOTA, FALSE, pid);
    }
    if hProc == std::ptr::null_mut() {
        return Err(Error::last_os_error())
    }
    Ok(hProc)
}

fn is_proc_in_job(hJob : HANDLE, hProcess : HANDLE) -> Result<BOOL, Error>{
    Ok(TRUE)
}

fn create_job_object() -> Result<HANDLE, Error>{
    Ok(std::ptr::null_mut())
}

fn assign_proct_to_job_object(){}
