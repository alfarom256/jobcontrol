extern crate winapi;
use std::io::Error;
use std::ffi::CString;
use self::winapi::um::winnt::{HANDLE, PROCESS_SET_QUOTA, PROCESS_TERMINATE, PROCESS_QUERY_LIMITED_INFORMATION};
use self::winapi::um::processthreadsapi::{OpenProcess};
use self::winapi::um::jobapi::{IsProcessInJob};
use self::winapi::um::jobapi2::{CreateJobObjectW};
use self::winapi::um::winbase::{CreateJobObjectA};
use self::winapi::shared::minwindef::{BOOL, TRUE, FALSE};




pub fn assign_and_process_job(pids: Vec<u32>, net_ctl : u32, cpu_pct : f32, dbg : bool) -> Result<u32, Error> {
    let mut v_proc_handles : Vec<HANDLE> = Vec::new();
    // open all of the processes and get their respective handles
    for pid in pids.into_iter(){
        let h_proc = match open_process(pid){
            Ok(x) => x,
            // if we can't open a process given in the args, return an error
            Err(x) => return Err(x)
        };
        println!("got handle for PID {}", pid);
        v_proc_handles.push(h_proc);
    }

    for h_proc in v_proc_handles.into_iter() {
        if is_proc_in_job(h_proc) == FALSE{
            println!("Process already in job");
            return Err(Error::last_os_error())
        } 
        println!("Process not in job...");
    }
    // create the job object
    println!("Creating job object");
    let h_job : HANDLE = match create_job_object(){
        Ok(x) => x,
        Err(e) => {return Err(e)}
    };

    Ok(1)
}

fn open_process(pid : u32) -> Result<HANDLE, Error>{
    let mut h_proc : HANDLE = std::ptr::null_mut();
    unsafe {
        h_proc = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_TERMINATE | PROCESS_SET_QUOTA, FALSE, pid);
    }
    if h_proc == std::ptr::null_mut() {
        return Err(Error::last_os_error())
    }
    Ok(h_proc)
}

fn is_proc_in_job(h_proc : HANDLE) -> BOOL{
    let mut b_result : BOOL = FALSE;
    unsafe {
        IsProcessInJob(h_proc, std::ptr::null_mut(), &mut b_result);
    }
    b_result
}

fn create_job_object() -> Result<HANDLE, Error>{
    let lp_job_name = CString::new("FUG").unwrap();
    let h_job : HANDLE;
    unsafe {
        h_job = CreateJobObjectA(std::ptr::null_mut(), lp_job_name.as_ptr());
    }
    if h_job == std::ptr::null_mut(){
        return Err(Error::last_os_error());
    }
    Ok(h_job)
}

fn assign_proc_to_job_object(h_job : HANDLE, h_proc : HANDLE) -> Result<BOOL, Error>{

    Ok(1)
}
