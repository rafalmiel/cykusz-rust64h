pub mod task;

use arch::task::switch;

#[macro_export]
macro_rules! switch {
    ($ctx1:expr, $ctx2:expr) => (
        switch(&mut $ctx1.arch_task, &$ctx2.arch_task);
    )
}

struct Scheduler {
    sched_task: task::Task,
    tasks: [task::Task; 32],
    current: usize,
    init: bool
}

pub fn create_kernel_task(fun: fn()) {
    unsafe {
        SCHEDULER.add_task(fun);
    }
}

pub fn create_user_task(fun: fn(), stack: usize, stack_size: usize) {
    unsafe {
        SCHEDULER.add_user_task(fun, stack, stack_size);
    }
}

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

fn scheduler() {
    loop {
        unsafe {
            SCHEDULER.schedule_next();
        }
    }
}

pub fn task_finished() {
    println!("[ TASK ] task finished");
    unsafe {
        SCHEDULER.task_finished();
    }
}

pub fn resched() {
    unsafe {
        SCHEDULER.resched();
    }
}

impl Scheduler {
    pub const fn empty() -> Scheduler {
        Scheduler {
            sched_task: task::Task::empty(),
            tasks: [task::Task::empty(); 32],
            current: 0,
            init: false
        }
    }

    pub fn init(&mut self) {
        self.sched_task = task::Task::new_sched(scheduler);
        self.tasks[0].state = task::TaskState::Running;
        self.init = true;
    }

    pub fn add_task(&mut self, fun: fn()) {
        for i in 0..32 {
            if self.tasks[i].state == task::TaskState::Unused {
                self.tasks[i] = task::Task::new_kern(fun);
                return;
            }
        }
    }

    pub fn add_user_task(&mut self, fun: fn(), stack: usize, stack_size: usize) {
        for i in 0..32 {
            if self.tasks[i].state == task::TaskState::Unused {
                self.tasks[i] = task::Task::new_user(fun, stack, stack_size);
                return;
            }
        }
    }

    pub fn resched(&mut self) {
        switch!(self.tasks[self.current], self.sched_task);
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

            if t.state == task::TaskState::ToResched && t.locks == 0 {
                t.state = task::TaskState::Running;
                resched();
            }
        }
    }

    pub fn task_finished(&mut self) {
        let ref mut t = self.tasks[self.current];
        t.state = task::TaskState::ToDelete;
        resched();
    }

    pub fn schedule_next(&mut self) {
        if self.tasks[self.current].state == task::TaskState::ToDelete {
            self.tasks[self.current].deallocate();
            return;
        }
        else if self.tasks[self.current].locks > 0 {
            self.tasks[self.current].state = task::TaskState::ToResched;
            switch!(self.sched_task, self.tasks[self.current]);
            return;
        }

        let mut to: Option<usize> = None;
        for i in (self.current+1)..32 {
            if self.tasks[i as usize].state == task::TaskState::Runnable {
                to = Some(i as usize);
                break;
            }
        }

        if to.is_none() {
            for i in 1..(self.current+1) {
                if self.tasks[i as usize].state == task::TaskState::Runnable {
                    to = Some(i as usize);
                    break;
                }
            }
        }

        if to.is_none() {
            to = Some(0 as usize);
        }

        if let Some(t) = to {
            if self.tasks[self.current as usize].state == task::TaskState::Running {
                self.tasks[self.current as usize].state = task::TaskState::Runnable;
            }
            self.tasks[t].state = task::TaskState::Running;
            self.current = t;

            switch!(self.sched_task, self.tasks[t]);
        } else {
            panic!("SCHED: to not found...");
        }
    }
}

static mut SCHEDULER : Scheduler = Scheduler::empty();

pub fn init() {
    unsafe {
        SCHEDULER.init();
    }
}