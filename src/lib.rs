extern crate winapi;
use self::winapi::shared::minwindef::{BOOL, DWORD, FALSE, TRUE, LPVOID};
use self::winapi::um::handleapi::CloseHandle;
use self::winapi::um::jobapi::IsProcessInJob;
use self::winapi::um::jobapi2::{AssignProcessToJobObject, SetInformationJobObject};
use self::winapi::um::processthreadsapi::OpenProcess;
use self::winapi::um::winbase::CreateJobObjectA;
use self::winapi::um::winnt::{
    JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_u, HANDLE, JOBOBJECT_CPU_RATE_CONTROL_INFORMATION,
    PJOBOBJECT_CPU_RATE_CONTROL_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SET_QUOTA,
    PROCESS_TERMINATE, JOB_OBJECT_CPU_RATE_CONTROL_ENABLE, JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP,
    JobObjectCpuRateControlInformation
};
use std::ffi::CString;
use std::io::Error;

/*
todo:

Consideration needs to be made to close handles to all processes before returning an error.
Change debug printing to only print when compiled in debug

*/

pub fn assign_and_process_job(
    pids: Vec<u32>,
    net_ctl: u32,
    cpu_pct: f32,
    dbg: bool,
) -> Result<u32, Error> {
    let mut v_proc_handles: Vec<HANDLE> = Vec::new();
    // open all of the processes and get their respective handles
    for pid in pids.into_iter() {
        let h_proc = match open_process(pid) {
            Ok(x) => x,
            // if we can't open a process given in the args, return an error after closing all the handles
            Err(x) => {
                close_handle_vec(v_proc_handles);
                return Err(x);
            }
        };
        println!("got handle for PID {}", pid);
        v_proc_handles.push(h_proc);
    }

    for h_proc in &v_proc_handles {
        if is_proc_in_job(h_proc) == FALSE {
            println!("Process already in job");
            close_handle_vec(v_proc_handles);
            return Err(Error::last_os_error());
        }
        println!("Process not in job...");
    }
    // create the job object
    println!("Creating job object");
    let h_job: HANDLE = match create_job_object() {
        Ok(x) => x,
        // close handles and return err
        Err(e) => {
            close_handle_vec(v_proc_handles);
            return Err(e);
        }
    };

    // assign all the processes to the job object
    for h_proc in &mut v_proc_handles {
        if assign_proc_to_job_object(&h_job, h_proc) == TRUE {
            println!("Added process to job");
        } else {
            // close the job handle
            close_handle(&h_job);
            // close the process handles
            close_handle_vec(v_proc_handles);
            return Err(Error::last_os_error());
        }
    }
    // now that all of the processes are assigned to the job
    // take the values from the args and take the appropriate actions

    let cpu_pct_mul: u32;
    // if we're rate limiting the CPU
    if cpu_pct != 0f32 {
        unsafe {
            // get the actual percent value needed for the API call
            // https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_cpu_rate_control_information
            cpu_pct_mul = (cpu_pct * 100f32) as u32;
            let mut jrci_u: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_u = std::mem::zeroed();

            // get the address of the CPU rate in the union
            let mut jrci_cpu_ptr: &DWORD = jrci_u.CpuRate_mut();

            // set it to the addr of the new cpu_pct_mul
            jrci_cpu_ptr = &cpu_pct_mul;
            let mut jcrci = JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
                ControlFlags: JOB_OBJECT_CPU_RATE_CONTROL_ENABLE | JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP,
                u: jrci_u,
            };
            let jcrci_ptr = &mut jcrci as *mut _ as LPVOID;

            // set the jobs information object 
            match SetInformationJobObject(h_job, JobObjectCpuRateControlInformation, jcrci_ptr, std::mem::size_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION>() as u32){
                TRUE => {println!("Set cpu percent control job information to {}%", cpu_pct)},
                _ => {
                    close_handle(&h_job);
                    close_handle_vec(v_proc_handles);
                    return Err(Error::last_os_error())
                }
            }
        }
    }
    Ok(1)
}

fn close_handle(h: &HANDLE) -> BOOL {
    let b_res: BOOL;
    unsafe {
        b_res = CloseHandle(*h);
    };
    b_res
}

fn close_handle_vec(v_handles: Vec<HANDLE>) {
    for h in &v_handles {
        close_handle(h);
    }
}

fn open_process(pid: u32) -> Result<HANDLE, Error> {
    let mut h_proc: HANDLE = std::ptr::null_mut();
    unsafe {
        h_proc = OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_TERMINATE | PROCESS_SET_QUOTA,
            FALSE,
            pid,
        );
    }
    if h_proc == std::ptr::null_mut() {
        return Err(Error::last_os_error());
    }
    Ok(h_proc)
}

fn is_proc_in_job(h_proc: &HANDLE) -> BOOL {
    let mut b_result: BOOL = FALSE;
    unsafe {
        IsProcessInJob(*h_proc, std::ptr::null_mut(), &mut b_result);
    }
    b_result
}

fn create_job_object() -> Result<HANDLE, Error> {
    let lp_job_name = CString::new("FUG").unwrap();
    let h_job: HANDLE;
    unsafe {
        h_job = CreateJobObjectA(std::ptr::null_mut(), lp_job_name.as_ptr());
    }
    if h_job == std::ptr::null_mut() {
        return Err(Error::last_os_error());
    }
    Ok(h_job)
}

fn assign_proc_to_job_object(h_job: &HANDLE, h_proc: &HANDLE) -> BOOL {
    let b_result: BOOL;
    unsafe {
        b_result = AssignProcessToJobObject(*h_job, *h_proc);
    };
    b_result
}
