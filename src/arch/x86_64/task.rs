use super::int;
use alloc::boxed::Box;
use alloc::allocator::Alloc;
use ::HEAP;

//TODO:
// - create proper scheduler with many processes
// - cleanup api

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

#[derive(Copy, Clone, Debug)]
struct ContextMutPtr(*mut Context);

unsafe impl Send for ContextMutPtr {}

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
    ctx: ContextMutPtr,

    //0 unused
    //1 running
    //2 runnable
    //3 to_reschedule
    //4 to_delete
    state: u32,
    locks: u32,

    //top of the stack, used to deallocate
    stack_top: usize,
}

impl Task {
    pub fn new(fun: fn ()) -> Task {
        unsafe {
            let sp = HEAP.alloc(::alloc::heap::Layout::from_size_align_unchecked(4096*4, 4096)).unwrap()
                .offset(4096*4);
            *(sp.offset(-8) as *mut usize) = dead_task as usize;//task finished function
            *(sp.offset(-24) as *mut usize) = sp.offset(-8) as usize;                           //sp
            *(sp.offset(-32) as *mut usize) = 0x200;                                            //rflags enable interrupts
            *(sp.offset(-40) as *mut usize) = ::x86::shared::segmentation::cs().bits() as usize;//cs
            *(sp.offset(-48) as *mut usize) = fun as usize;                                     //rip
            let mut ctx = sp.offset(-(::core::mem::size_of::<Context>() as isize + 48 + 11*8)) as *mut Context;
            (*ctx).rip = isr_return as usize;
            Task {
                ctx: ContextMutPtr(ctx),
                state: 2,
                locks: 0,
                stack_top: sp as usize - 4096*4,
            }
        }
    }

    pub fn new_sched(fun: fn ()) -> Task {
        unsafe {
            let sp = HEAP.alloc(::alloc::heap::Layout::from_size_align_unchecked(4096*4, 4096)).unwrap()
                .offset(4096*4);
            *(sp.offset(-8) as *mut usize) = dead_task as usize;//task finished function
            *(sp.offset(-24) as *mut usize) = sp.offset(-8) as usize;                           //sp
            *(sp.offset(-32) as *mut usize) = 0;                                            //rflags enable interrupts
            *(sp.offset(-40) as *mut usize) = ::x86::shared::segmentation::cs().bits() as usize;//cs
            *(sp.offset(-48) as *mut usize) = fun as usize;                                     //rip
            let mut ctx = sp.offset(-(::core::mem::size_of::<Context>() as isize + 48 + 11*8)) as *mut Context;
            (*ctx).rip = isr_return as usize;
            Task {
                ctx: ContextMutPtr(ctx),
                state: 2,
                locks: 0,
                stack_top: sp as usize - 4096*4,
            }
        }
    }


    pub fn new_user() -> Task {
        use arch::mm::virt;
        unsafe {
            virt::map_flags(0x600000, virt::entry::USER | virt::entry::WRITABLE);
            let sp = ((0x600000 + 4096) as *mut u8);
            //let cd = virt::map_flags(0x400000, virt::entry::USER);
            *(sp.offset(-16) as *mut usize) = 0x23;
            *(sp.offset(-24) as *mut usize) = sp.offset(-8) as usize;                           //sp
            *(sp.offset(-32) as *mut usize) = 0x200;                                            //rflags enable interrupts
            *(sp.offset(-40) as *mut usize) = 0x1b;//cs
            *(sp.offset(-48) as *mut usize) = 0x400000;                                    //rip
            let mut ctx = sp.offset(-(::core::mem::size_of::<Context>() as isize + 48 + 11*8)) as *mut Context;
            (*ctx).rip = isr_return as usize;
            Task {
                ctx: ContextMutPtr(ctx),
                state: 2,
                locks: 0,
                stack_top: sp as usize - 4096,
            }
        }
    }

    pub const fn empty() -> Task {
        Task {
            ctx: ContextMutPtr(::core::ptr::null_mut()),
            state: 0,
            locks: 0,
            stack_top: 0,
        }
    }

    pub fn deallocate(&mut self) {
        self.state = 0;
        self.ctx = ContextMutPtr(::core::ptr::null_mut());
        self.locks = 0;
        unsafe {
            HEAP.dealloc(self.stack_top as *mut u8, ::alloc::heap::Layout::from_size_align_unchecked(4096*4, 4096));
        }
        self.stack_top = 0;
    }
}

extern "C" {
    fn switch_to(old_ctx: *mut *mut Context, new_ctx: *const Context);
    fn isr_return();
}

macro_rules! switch {
    ($ctx1:expr, $ctx2:expr) => (
        switch_to((&mut $ctx1.ctx.0) as *mut *mut Context, $ctx2.ctx.0);
    )
}

struct Scheduler {
    sched_task: Task,
    tasks: [Task; 32],
    current: usize,
    init: bool
}

impl Scheduler {
    pub const fn empty() -> Scheduler {
        Scheduler {
            sched_task: Task::empty(),
            tasks: [Task::empty(); 32],
            current: 0,
            init: false
        }
    }

    pub fn init(&mut self) {
        self.sched_task = Task::new_sched(scheduler);
        self.tasks[0].state = 1;
        self.init = true;
    }

    pub fn add_task(&mut self, fun: fn()) {
        for i in 0..32 {
            if self.tasks[i].state == 0 {
                self.tasks[i] = Task::new(fun);
                return;
            }
        }
    }

    pub fn add_user_task(&mut self) {
        for i in 0..32 {
            if self.tasks[i].state == 0 {
                self.tasks[i] = Task::new_user();
                return;
            }
        }
    }

    pub fn resched(&mut self) {
        unsafe {
            switch!(self.tasks[self.current], self.sched_task);
        }
    }

    pub fn task_locked(&mut self) {
        if self.init {
            self.tasks[self.current].locks += 1;
        }
    }

    pub fn task_unlocked(&mut self) {
        if self.init {
            let ref mut t = self.tasks[self.current];
            t.locks -= 1;

            if t.state == 3 && t.locks == 0 {
                t.state = 1;
                resched();
            }
        }
    }

    pub fn task_finished(&mut self) {
        let ref mut t = self.tasks[self.current];
        t.state = 4;
        resched();
    }

    pub fn schedule_next(&mut self) {
        if self.tasks[self.current].state == 4 {
            self.tasks[self.current].deallocate();
            return;
        }
        else if self.tasks[self.current].locks > 0 {
            self.tasks[self.current].state = 3;
            unsafe {
                switch!(self.sched_task, self.tasks[self.current]);
            }
            return;
        }

        let mut to: Option<usize> = None;
        for i in (self.current+1)..32 {
            if self.tasks[i as usize].state == 2 {
                to = Some(i as usize);
                break;
            }
        }

        if to.is_none() {
            for i in 1..(self.current+1) {
                if self.tasks[i as usize].state == 2 {
                    to = Some(i as usize);
                    break;
                }
            }
        }

        if to.is_none() {
            to = Some(0 as usize);
        }

        if let Some(t) = to {
            if self.tasks[self.current as usize].state == 1 {
                self.tasks[self.current as usize].state = 2;
            }
            self.tasks[t].state = 1;
            self.current = t;

            unsafe {
                switch!(self.sched_task, self.tasks[t]);
            }
        } else {
            panic!("SCHED: to not found...");
        }
    }
}

static mut SCHEDULER : Scheduler = Scheduler::empty();

pub fn task_locked() {
    unsafe {
        SCHEDULER.task_locked();
    }
}

pub fn task_unlocked() {
    unsafe {
        SCHEDULER.task_unlocked();
    }
}

pub fn create_kern_task(fun: fn()) {
    unsafe {
        SCHEDULER.add_task(fun);
    }
}

pub fn create_user_task() {
    unsafe {
        SCHEDULER.add_user_task();
    }
}

pub fn scheduler() {
    loop {
        unsafe {
            SCHEDULER.schedule_next();
        }
    }
}

pub fn dead_task() {
    println!("TASK FINISHED");
    unsafe {
        SCHEDULER.task_finished();
    }
}

pub fn resched() {
    unsafe {
        SCHEDULER.resched();
    }
}

fn task_1() {
    let mut i: u32 = 0;
    loop {
        println!("KERNEL TASK 1 {}", i);
        i = i+1;
    }


}

fn task_2() {
    let mut i: u32 = 0;
    loop {
        println!("KERNEL TASK 2 {}", i);
        i = i+1;
    }


}

fn task_3() {
    let mut i: u32 = 0;
    for _ in 0..200 {
        let mut heap_test = Box::new(42);

        //println!("Allocated in task 3");
    }
}

fn task_4() {
    let mut i: u32 = 0;
    for _ in 0..200 {
        let mut heap_test = Box::new(42);

        //println!("Allocated in task 4");
    }
}

pub fn init() {
    unsafe {
        SCHEDULER.init();
    }
    create_kern_task(task_1);
    create_kern_task(task_2);
    create_user_task();

    int::fire_timer();
    int::enable_interrupts();
}
