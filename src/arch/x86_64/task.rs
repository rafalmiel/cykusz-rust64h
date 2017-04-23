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

static mut SCHED: Task = Task::empty();

static mut TASK0: Task = Task::empty();
static mut TASK1: Option<Task> = None;
static mut TASK2: Option<Task> = None;

static mut CTASK: u8 = 0;

pub fn scheduler() {
    loop {
        unsafe {
            let t1 = TASK1.unwrap();
            let t2 = TASK2.unwrap();

            CTASK = 1;
            switch!(SCHED, t1);

            CTASK = 2;
            switch!(SCHED, t2);
        }
    }
}

#[no_mangle]
pub extern "C" fn eoi() {
    int::end_of_interrupt();
}

#[no_mangle]
pub fn dead_task() {
    println!("TASK 2 FINISHED");
    
    loop {
    }
}

pub fn resched() {
    unsafe {      
        int::disable_interrupts();  
        match CTASK {
            0 => switch!(TASK0, SCHED),
            1 => if let Some(ref mut t) = TASK1 {
                switch!(t, SCHED);
            },
            2 => if let Some(ref mut t) = TASK2 {
                switch!(t, SCHED);
            },
            _ => {}
        }
        int::enable_interrupts();
    }
}

fn task_1() {
    let mut i = 0;
    loop {
        if i % 1000000 == 0 {
            println!("TASK 1 {}", i);
            //resched();
        }
        i += 1;
    }
}

fn task_2() {
    let mut i = 0;
    loop {
        if i % 1000000 == 0 {
            println!("TASK 2 {}", i);
        }
        i += 1;
    }
}

pub fn init() {
    unsafe {
        CTASK = 0;
        TASK1 = Some(Task::new(task_1));
        TASK2 = Some(Task::new(task_2));
        SCHED = Task::new(scheduler);
    }

    int::enable_interrupts();
    int::fire_timer();
}
