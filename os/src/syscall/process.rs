//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::PAGE_SIZE,
    mm::{translated_byte_buffer, PageTable, VirtAddr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_cnt, mmap,
        munmap, suspend_current_and_run_next,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let ts_in_kernel =
        translated_byte_buffer(current_user_token(), ts as *const u8, size_of::<TimeVal>());
    let ts_ptr = ts_in_kernel[0].as_ptr() as *mut TimeVal;
    let us = get_time_us();
    unsafe {
        (*ts_ptr).sec = us / 1_000_000;
        (*ts_ptr).usec = us % 1_000_000;
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    if !(id >> 39 == 0x1ff_ffff || id >> 39 == 0x0) {
        return -1;
    }
    match trace_request {
        0 => {
            let app_vaddr = id as *const u8;
            let kernel_vaddr =
                translated_byte_buffer(current_user_token(), app_vaddr, size_of::<u8>());
            if kernel_vaddr.is_empty() {
                return -1;
            }

            let pt = PageTable::from_token(current_user_token());
            let vpn = VirtAddr::from(id).floor();
            let pte = pt.translate(vpn).unwrap();
            if !pte.readable() {
                return -1;
            }

            let val = unsafe { *(kernel_vaddr[0].as_ptr()) };
            val as isize
        }
        1 => {
            let app_vaddr = id as *const u8;
            let kernel_vaddr =
                translated_byte_buffer(current_user_token(), app_vaddr, size_of::<u8>());
            if kernel_vaddr.is_empty() {
                return -1;
            }

            let pt = PageTable::from_token(current_user_token());
            let vpn = VirtAddr::from(id).floor();
            let pte = pt.translate(vpn).unwrap();
            if !pte.writable() {
                return -1;
            }

            unsafe { *(kernel_vaddr[0].as_ptr() as *mut u8) = data as u8 };
            0
        }
        2 => get_syscall_cnt(id),
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if start % PAGE_SIZE != 0 || port & !0x7 != 0 || port & 0x7 == 0 {
        return -1;
    }

    // align to page size
    if start & 0xfff != 0 {
        return -1;
    }

    mmap(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");

    // align to page size
    if start & 0xfff != 0 {
        return -1;
    }

    munmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
