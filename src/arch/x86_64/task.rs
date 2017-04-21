use super::int;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Context {
    /// RFLAGS register
    pub rflags: usize,
    /// RBX register
    pub rbp: usize,
    /// R12 register
    pub r12: usize,
    /// R13 register
    pub r13: usize,
    /// R14 register
    pub r14: usize,
    /// R15 register
    pub r15: usize,
    /// Base pointer
    pub rbx: usize,
    /// Instruction pointer
    pub rip: usize
}

impl Context {
    const fn empty() -> Context {
        Context {
            rflags: 0,
            rbp: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rbx: 0,
            rip: 0
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Task {
    ctx: *mut Context,
    prio: u32,
    fromint: bool
}

impl Task {
    pub fn new(fun: fn ()) -> Task {
        unsafe {
            let sp = ::alloc::heap::allocate(4096*4, 4096)
                .offset(4096*4);
            *(sp.offset(-8) as *mut usize) = dead_task as usize;//task finished function
            *(sp.offset(-24) as *mut usize) = sp.offset(-8) as usize;                           //sp
            *(sp.offset(-32) as *mut usize) = 0x200;                                            //rflags enable interrupts
            *(sp.offset(-40) as *mut usize) = ::x86::shared::segmentation::cs().bits() as usize;//cs
            *(sp.offset(-48) as *mut usize) = fun as usize;                                     //rip
            let mut ctx = sp.offset(-(::core::mem::size_of::<Context>() as isize + 48 + 11*8)) as *mut Context;
            (*ctx).rip = isr_return as usize;
            println!("Set rip to 0x{:x}", isr_return as usize);
            Task {
                ctx: ctx,
                prio: 1,
                fromint: false
            }
        }
    }

    pub const fn empty() -> Task {
        Task {
            ctx: ::core::ptr::null_mut(),
            prio: 0,
            fromint: false
        }
    }
}

extern "C" {
    fn switch_to(old_ctx: *mut *mut Context, new_ctx: *const Context);
    fn isr_return();
}

macro_rules! switch {
    ($ctx1:ident, $ctx2:ident) => (
        switch_to((&mut $ctx1.ctx) as *mut *mut Context, $ctx2.ctx);
    )
}

static mut TASK1: Task = Task::empty();
static mut TASK2: Option<Task> = None;
static mut CTASK: *mut Task = ::core::ptr::null_mut();
static mut T1: bool = true;

pub fn sched(fromint: bool) {
    int::disable_interrupts();
    unsafe {
        if T1 {
            T1 = false;
            if let Some(ref mut t) = TASK2 {
                t.fromint = fromint;
                CTASK = t as *mut Task;
                switch!(TASK1, t);
            }
        } else {
            T1 = true;
            if let Some(ref mut t) = TASK2 {
                TASK1.fromint = fromint;
                CTASK = &mut TASK1 as *mut Task;
                switch!(t, TASK1);
            }
        }
    }
    int::enable_interrupts();
}

#[no_mangle]
pub extern "C" fn eoi() {
    int::end_of_interrupt();
}

#[no_mangle]
pub extern "C" fn dead_task() {
    println!("TASK 2 FINISHED");
    
    loop {
    }
}

fn resched() {
    sched(false);

    unsafe {
        if (*CTASK).fromint {
            eoi();
        }
    }
}

fn task_2() {
    let mut i = 0;
    loop {
        if i % 1000000 == 0 {
            println!("TASK 2 {}", i);
            resched();
        }
        i += 1;
    }
}

pub fn init() {
    unsafe {
        CTASK = &mut TASK1 as *mut Task;
        TASK2 = Some(Task::new(task_2));
    }

    int::enable_interrupts();
    int::fire_timer();

    let mut i = 0;
    loop {
        if i % 1000000 == 0 {
            println!("TASK 1 {}", i);
        }
        i += 1;
    }

}
