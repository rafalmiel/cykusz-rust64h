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

extern {
    fn switch_to(old_ctx: *mut *mut Context, new_ctx: *const Context);
    fn read_rsp() -> u64;
}

macro_rules! switch {
    ($ctx1:ident, $ctx2:ident) => (
        switch_to((&mut $ctx1) as *mut *mut Context, $ctx2);
    )
}

static mut CTX1: *mut Context = 0 as *mut Context;
static mut CTX2: *mut Context = 0 as *mut Context;

fn task_2() {
    loop {
        println!("TASK 2");

        unsafe {
            switch!(CTX2, CTX1);
        }
    }
}

pub fn init() {
    unsafe {
        let sp = ::alloc::heap::allocate(4096*4, 4096)
            .offset(4096*4)
            .offset(-(::core::mem::size_of::<Context>() as isize));

        CTX2 = sp as *mut Context;

        (*CTX2).rip = task_2 as usize;

        println!("Set rip to 0x{:x}", (*CTX2).rip);

        println!("CTX1 addr 0x{:x}", (&mut CTX1) as *mut *mut Context as usize);
        println!("CTX2 addr 0x{:x}", CTX2 as usize);
    }

    loop {
        println!("TASK 1");

        unsafe {

            switch!(CTX1, CTX2);
        }
    }

}