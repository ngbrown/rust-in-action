use winapi::{
    DUPLICATE_SAME_ACCESS, DWORD, FALSE, HANDLE, LPSYSTEM_INFO, LPVOID,
    MEMORY_BASIC_INFORMATION as MEM_INFO, PVOID, SIZE_T, SYSTEM_INFO,
};

fn main() {
    let this_pid: DWORD;
    let this_proc_psudo: HANDLE;
    let mut this_proc: HANDLE;
    let min_addr: LPVOID;
    let max_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEM_INFO;

    const MEM_INFO_SIZE: usize = std::mem::size_of::<MEM_INFO>();

    unsafe {
        this_proc = std::mem::zeroed();
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    unsafe {
        this_pid = kernel32::GetCurrentProcessId();

        // returns a psudo handle of -1
        this_proc_psudo = kernel32::GetCurrentProcess();

        // duplicate to get the real handle number
        kernel32::DuplicateHandle(
            this_proc_psudo,
            this_proc_psudo,
            this_proc_psudo,
            &mut this_proc,
            0,
            FALSE,
            DUPLICATE_SAME_ACCESS,
        );

        kernel32::GetSystemInfo(&mut proc_info as LPSYSTEM_INFO);
    }

    min_addr = proc_info.lpMinimumApplicationAddress;
    max_addr = proc_info.lpMaximumApplicationAddress;

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_addr, max_addr);

    loop {
        let rc: SIZE_T = unsafe {
            kernel32::VirtualQueryEx(
                this_proc_psudo,
                base_addr,
                &mut mem_info,
                MEM_INFO_SIZE as SIZE_T,
            )
        };

        if rc == 0 {
            break;
        }

        println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
    }
}
