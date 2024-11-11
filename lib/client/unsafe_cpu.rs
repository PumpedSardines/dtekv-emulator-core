use crate::cpu::Cpu;
use std::{
    alloc::{alloc, Layout}, ptr, sync::atomic::{AtomicBool, Ordering}
};

#[derive(Debug)]
pub struct UnsafeCpu {
    ptr: *mut u8,
    mut_lock: AtomicBool,
}
unsafe impl Sync for UnsafeCpu {}
unsafe impl Send for UnsafeCpu {}

impl UnsafeCpu {
    pub fn new(cpu: Cpu) -> Self {
        let layout = Layout::new::<Cpu>();
        let cpu_ptr = unsafe {
            let ptr = alloc(layout);
            ptr::write(ptr as *mut Cpu, cpu);
            ptr
        };
        UnsafeCpu {
            ptr: cpu_ptr,
            mut_lock: AtomicBool::new(false),
        }
    }

    pub unsafe fn get(&self) -> &Cpu {
        &*(self.ptr as *const Cpu)
    }

    pub unsafe fn get_mut(&self) -> &mut Cpu {
        if self.mut_lock.load(Ordering::Relaxed) {
            panic!("CPU already borrowed mutably");
        }
        self.mut_lock.store(true, Ordering::Relaxed);

        &mut *(self.ptr as *mut Cpu)
    }
}

impl Drop for UnsafeCpu {
    fn drop(&mut self) {}
}
