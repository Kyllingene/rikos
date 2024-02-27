use super::{Task, TaskId};
use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::{
    fmt::Debug,
    task::{Context, Waker},
};
use crossbeam_queue::ArrayQueue;
use futures_util::Future;
use x86_64::instructions::interrupts;

pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

/// An asyncronous executor, utilizing cooperative async.
///
/// `Executor::run` hijacks the current thread. If you would
/// like to run a "sub-executor" to run "sub-tasks" (to allow
/// spawning new tasks at runtime, for instance), this also
/// implements [`Future`]; on every poll, it polls its tasks.
impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(128)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let id = task.id;
        if self.tasks.insert(id, task).is_some() {
            panic!("tried to spawn task with duplicate ID");
        }

        self.task_queue.push(id).expect("task pool is full");
    }

    /// Returns true if the task pool is full.
    pub fn full(&self) -> bool {
        self.task_queue.is_full()
    }

    /// Hijacks the current thread to run the executor forever.
    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();

            interrupts::disable();
            if self.task_queue.is_empty() {
                interrupts::enable_and_hlt();
            } else {
                interrupts::enable();
            }
        }
    }

    fn run_ready_tasks(&mut self) {
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(t) => t,
                None => continue,
            };

            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                core::task::Poll::Ready(()) => {
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                core::task::Poll::Pending => {}
            }
        }
    }
}

impl Future for Executor {
    type Output = ();

    fn poll(
        mut self: core::pin::Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        self.run_ready_tasks();
        core::task::Poll::Pending
    }
}

impl Debug for Executor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Executor")
            .field("tasks", &self.tasks.len())
            .field("task_queue", &self.task_queue)
            .field("waker_cache", &self.waker_cache)
            .finish()
    }
}

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(Self {
            task_id,
            task_queue,
        }))
    }

    fn wake_task(&self) {
        self.task_queue
            .push(self.task_id)
            .expect("task pool is full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task()
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task()
    }
}
