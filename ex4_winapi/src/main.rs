// 詳解Rustプログラミング by Tim McNamara
#[cfg(windows)]
use kernel32;
#[cfg(windows)]
use winapi;

#[cfg(windows)]
use winapi::um::winnt::MEMORY_BASIC_INFORMATION as MEMINFO;

#[cfg(windows)]
fn run() {
    let this_pid;
    let this_proc;
    let min_addr;
    let max_addr;
    let mut base_addr;
    let proc_info;
    let mut mem_info;
    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMINFO>();
    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        kernel32::GetSystemInfo(proc_info);
        min_addr = (*proc_info).lpMinimumApplicationAddress;
        max_addr = (*proc_info).lpMaximumApplicationAddress;
    }
    println!("{:?}@{:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min:{:p},max:{:p}", min_addr, max_addr);
    loop {
        let rc = unsafe {
            kernel32::VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEMINFO_SIZE as u64)
        };
        if rc == 0 {
            break;
        }
        // println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as *const std::ffi::c_void;
    }
}

fn main() {
    run();
}
