use std::collections::HashMap;
use std::future::Future;
use std::mem::forget;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::{mem, thread};
use std::thread::JoinHandle;
use std::time::Duration;

fn block_on<F: Future>(mut future: F) -> F::Output {
    let parker = Arc::new(Parker::default()); // NB!
    let mywaker = Arc::new(MyWaker { parker: parker.clone() });
    let waker = waker_into_waker(Arc::into_raw(mywaker));

    let mut ctx = Context::from_waker(&waker);

    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    let val = loop {
        match Future::poll(future.as_mut(), &mut ctx) {
            Poll::Ready(val) => break val,
            Poll::Pending => parker.park(),
        };
    };

    val
}


#[derive(Clone)]
struct MyWaker {
    parker: Arc<Parker>,
}

fn mywaker_wake(s: &MyWaker) {
    let waker_ptr: *const MyWaker = s;
    let waker_arc = unsafe { Arc::from_raw(waker_ptr) };
    waker_arc.parker.unpark();
}

fn mywaker_clone(s: &MyWaker) -> RawWaker {
    let arc = unsafe { Arc::from_raw(s) };
    forget(arc.clone()); //增加计数
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |s| mywaker_clone(&*(s as *const MyWaker)),   // clone
        |s| mywaker_wake(&*(s as *const MyWaker)),    // wake
        |s| (*(s as *const MyWaker)).parker.unpark(), // wake by ref (不减少计数)
        |s| drop(Arc::from_raw(s as *const MyWaker)), // 减少引用计数
    )
};

fn waker_into_waker(s: *const MyWaker) -> Waker {
    let raw_waker = RawWaker::new(s as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}


#[derive(Clone)]
pub struct Task {
    id: usize,
    reactor: Arc<Mutex<Box<Reactor>>>,
    data: u64,
}


impl Task {
    fn new(reactor: Arc<Mutex<Box<Reactor>>>, data: u64, id: usize) -> Self {
        Task { id, reactor, data }
    }
}

impl Future for Task {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut r = self.reactor.lock().unwrap();
        if r.is_ready(self.id) {
            *r.tasks.get_mut(&self.id).unwrap() = TaskState::Finished;
            Poll::Ready(self.id)
        } else if r.tasks.contains_key(&self.id) {
            r.tasks.insert(self.id, TaskState::NotReady(cx.waker().clone()));
            Poll::Pending
        } else {
            r.register(self.data, cx.waker().clone(), self.id);
            Poll::Pending
        }
    }
}


enum TaskState {
    Ready,
    NotReady(Waker),
    Finished,
}

struct Reactor {
    dispatcher: Sender<Event>,
    handle: Option<JoinHandle<()>>,
    tasks: HashMap<usize, TaskState>,
}

#[derive(Debug)]
enum Event {
    Close,
    Timeout(u64, usize),
}

impl Reactor {
    fn new() -> Arc<Mutex<Box<Self>>> {
        let (tx, rx) = channel::<Event>();
        let reactor = Arc::new(Mutex::new(Box::new(Reactor {
            dispatcher: tx,
            handle: None,
            tasks: HashMap::new(),
        })));

        let reactor_clone = Arc::downgrade(&reactor);

        let handle = thread::spawn(move || {
            let mut handles = vec![];
            for event in rx {
                println!("REACTOR: {:?}", event);
                let reactor = reactor_clone.clone();
                match event {
                    Event::Close => break,
                    Event::Timeout(duration, id) => {
                        let event_handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(duration));
                            let reactor = reactor.upgrade().unwrap();
                            reactor.lock().map(|mut r| r.wake(id)).unwrap();
                        });
                        handles.push(event_handle);
                    }
                }
            }

            handles.into_iter().for_each(|handle| handle.join().unwrap());
        });

        reactor.lock().map(|mut r| r.handle = Some(handle)).unwrap();
        reactor
    }

    // wake 函数为特定 id 的任务调用其唤醒器的 wake。
    fn wake(&mut self, id: usize) {
        self.tasks.get_mut(&id).map(|state| {

            // 无论任务之前处于什么状态，我们都可以安全地将其设为就绪状态。这使得我们
            // 可以在替换之前的数据前获取其所有权。
            match mem::replace(state, TaskState::Ready) {
                TaskState::NotReady(waker) => waker.wake(),
                TaskState::Finished => panic!("Called 'wake' twice on task: {}", id),
                _ => unreachable!()
            }
        }).unwrap();
    }

    // 在反应器中注册新任务。在这个例子中如果有两个相同 id 的任务被注册会导致 panic。
    fn register(&mut self, duration: u64, waker: Waker, id: usize) {
        if self.tasks.insert(id, TaskState::NotReady(waker)).is_some() {
            panic!("Tried to insert a task with id: '{}', twice!", id);
        }
        self.dispatcher.send(Event::Timeout(duration, id)).unwrap();
    }

    // 检查指定 id 的任务是否处于 `TaskState::Ready` 状态k
    fn is_ready(&self, id: usize) -> bool {
        self.tasks.get(&id).map(|state| match state {
            TaskState::Ready => true,
            _ => false,
        }).unwrap_or(false)
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        // 向反应器发送关闭事件让其结束反应器线程。如果不这么做会使其阻塞等待新事件。
        self.dispatcher.send(Event::Close).unwrap();
        self.handle.take().map(|h| h.join().unwrap()).unwrap();
    }
}

#[derive(Default)]
struct Parker(Mutex<bool>, Condvar);

impl Parker {
    fn park(&self) {
        let mut resumable = self.0.lock().unwrap();
        while !*resumable {
            resumable = self.1.wait(resumable).unwrap();
        }
        *resumable = false;
    }

    fn unpark(&self) {
        *self.0.lock().unwrap() = true;
        self.1.notify_one();
    }
}


#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn it_works() {
        // This is just to make it easier for us to see when our Future was resolved
        let start = Instant::now();

        // Many runtimes create a global `reactor` we pass it as an argument
        let reactor = Reactor::new();

        // We create two tasks:
        // - first parameter is the `reactor`
        // - the second is a timeout in seconds
        // - the third is an `id` to identify the task
        let future1 = Task::new(reactor.clone(), 1, 1);
        let future2 = Task::new(reactor.clone(), 2, 2);

        // an `async` block works the same way as an `async fn` in that it compiles
        // our code into a state machine, `yielding` at every `await` point.
        let fut1 = async {
            let val = future1.await;
            println!("Got {} at time: {:.2}.", val, start.elapsed().as_secs_f32());
        };

        let fut2 = async {
            let val = future2.await;
            println!("Got {} at time: {:.2}.", val, start.elapsed().as_secs_f32());
        };

        // Our executor can only run one and one future, this is pretty normal
        // though. You have a set of operations containing many futures that
        // ends up as a single future that drives them all to completion.
        let mainfut = async {
            fut1.await;
            fut2.await;
        };

        // This executor will block the main thread until the futures are resolved
        block_on(mainfut);
    }
}