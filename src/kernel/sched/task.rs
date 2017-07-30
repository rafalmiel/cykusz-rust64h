use arch::task::Task as ArchTask;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TaskState {
    Unused = 0,
    Running = 1,
    Runnable = 2,
    ToResched = 3,
    ToDelete = 4,
}

#[derive(Copy, Clone, Debug)]
pub struct Task {
    pub arch_task: ::arch::task::Task,
    pub state: TaskState,
    pub locks: u32,
}

impl Task {
    pub fn new_sched(fun: fn ()) -> Task {
        Task {
            arch_task: ArchTask::new_sched(fun),
            state: TaskState::Runnable,
            locks: 0
        }
    }

    pub fn new_kern(fun: fn ()) -> Task {
        Task {
            arch_task: ArchTask::new_kern(fun),
            state: TaskState::Runnable,
            locks: 0
        }
    }

    pub fn new_user(fun: fn (), stack: usize, stack_size: usize) -> Task {
        Task {
            arch_task: ArchTask::new_user(fun, stack, stack_size),
            state: TaskState::Runnable,
            locks: 0
        }
    }

    pub const fn empty() -> Task {
        Task {
            arch_task: ArchTask::empty(),
            state: TaskState::Unused,
            locks: 0
        }
    }

    pub fn deallocate(&mut self) {
        self.arch_task.deallocate();
        self.state = TaskState::Unused;
        self.locks = 0;
    }
}
