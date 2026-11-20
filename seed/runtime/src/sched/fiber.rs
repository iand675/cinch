//! The fiber backend: tasks as stackful fibers over a few worker threads.
//! The model, the state machine and the rules this file follows are the
//! module docs of [`crate::sched`].

use std::cell::{Cell, UnsafeCell};
use std::collections::{BTreeMap, VecDeque};
use std::ptr::null;
use std::sync::atomic::Ordering::{Relaxed, SeqCst};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, AtomicUsize};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use super::Stats;
use crate::stack::{self, Zone};
use crate::task::TaskState;

#[path = "switch.rs"]
mod switch;

pub(super) const FIBERS: bool = true;

// The states of a fiber (module docs of `crate::sched`).
const RUNNABLE: u8 = 0;
const RUNNING: u8 = 1;
const PARKING: u8 = 2;
const PARKED: u8 = 3;
const DONE: u8 = 4;

/// The stack reserved for each fiber.
const STACK_BYTES: usize = crate::task::TASK_STACK_BYTES;

/// The stack of a worker thread. It runs the scheduler loop and nothing
/// else: every task frame is on a fiber's stack.
const WORKER_STACK_BYTES: usize = 2 * stack::MIN_STACK_BYTES;

/// Finished fibers' stacks kept mapped for the next fibers, beyond the
/// one each scheduler thread keeps for itself. A stack past the cap is
/// unmapped, so a burst of suspended fibers does not pin its high-water
/// mark of stacks for the rest of the run.
const STACK_CACHE: usize = 64;

/// The state that the runtime keeps per OS thread but that belongs to
/// whatever context the thread is running (the thread-local audit in the
/// module docs of `crate::sched`). A fiber's copy is installed when the
/// fiber is switched in and taken back when it is switched out.
struct Locals {
    task: Option<Arc<TaskState>>,
    arena: *mut crate::arena::Arena,
    eval: crate::eval::State,
}

impl Locals {
    /// The state of a context that runs `task`'s body and has entered
    /// nothing yet.
    fn of(task: Option<Arc<TaskState>>) -> Locals {
        Locals { task, arena: std::ptr::null_mut(), eval: crate::eval::State::IDLE }
    }

    /// Make this the calling thread's state and answer the state it
    /// replaces.
    fn install(self) -> Locals {
        Locals {
            task: crate::task::swap_current(self.task),
            arena: crate::arena::replace_current(self.arena),
            eval: crate::eval::swap_state(self.eval),
        }
    }
}

/// One fiber: a task body, the stack it runs on and its place in the
/// state machine.
///
/// `state` and `token` are shared by every thread that parks, unparks or
/// schedules the fiber. The other fields have one owner at a time and
/// need no synchronization of their own: whoever made the fiber until it
/// is queued, then the scheduler thread that took it from the queue
/// (while the state is `RUNNABLE` in its hands, or `RUNNING`, `PARKING`
/// or `DONE` after the switch back), and the fiber's own code while it
/// runs. Storing `PARKED` gives the ownership up: any thread may queue
/// the fiber from then on.
pub(crate) struct Fiber {
    state: AtomicU8,
    token: AtomicBool,
    /// How many blocking regions the fiber's code is inside.
    blocking: AtomicU32,
    /// The saved context while the fiber is switched out.
    sp: UnsafeCell<usize>,
    /// The lowest address of the fiber's stack, 0 before its first run
    /// and after its last.
    stack: UnsafeCell<usize>,
    /// The stack's guard band, which the fault handler reads through the
    /// thread's key while the fiber runs.
    zone: UnsafeCell<Zone>,
    body: UnsafeCell<Option<Box<dyn FnOnce() + Send>>>,
    locals: UnsafeCell<Locals>,
}

// SAFETY: the non-atomic fields have one owner at a time (see the type's
// docs), and the raw arena pointer in `locals` is only ever dereferenced
// by the context that entered the arena.
unsafe impl Send for Fiber {}
unsafe impl Sync for Fiber {}

/// A thread that runs fibers while it waits (the program's thread): its
/// wake token, the condition variable it sleeps on when nothing is
/// runnable, and how many blocking regions its own code is inside.
pub(crate) struct Helper {
    token: AtomicBool,
    wake: Condvar,
    blocking: AtomicU32,
}

/// What this OS thread is doing for the scheduler.
struct Run {
    /// The thread's own context while it runs a fiber: where a fiber
    /// that suspends switches to.
    sched_sp: Cell<usize>,
    /// The fiber this thread is running, or null.
    fiber: Cell<*const Fiber>,
    /// This thread's helper record when it is a helping thread, or null.
    helper: Cell<*const Helper>,
    /// While a helping thread runs a fiber: when its own timed wait ends.
    deadline: Cell<Option<Instant>>,
    /// A finished fiber's stack this thread keeps for the next fiber it
    /// starts, or 0.
    spare: Cell<usize>,
}

thread_local! {
    static RUN: Run = const {
        Run {
            sched_sp: Cell::new(0),
            fiber: Cell::new(null()),
            helper: Cell::new(null()),
            deadline: Cell::new(None),
            spare: Cell::new(0),
        }
    };
}

/// One execution context that can be resumed (the three kinds of
/// [`crate::park`]).
#[derive(Clone)]
pub(crate) enum Context {
    /// A thread the scheduler does not own: it parks as a thread.
    Thread(std::thread::Thread),
    Helper(Arc<Helper>),
    Fiber(Arc<Fiber>),
}

impl Context {
    /// Resume the context, or leave it one token.
    pub(crate) fn unpark(&self) {
        match self {
            Context::Thread(thread) => thread.unpark(),
            Context::Helper(helper) => {
                // The sleeper checks the token under the lock before it
                // waits, so a notification under the lock cannot fall
                // between its check and its wait.
                if !helper.token.swap(true, SeqCst) {
                    let _held = lock();
                    helper.wake.notify_one();
                }
            }
            Context::Fiber(fiber) => {
                fiber.token.store(true, SeqCst);
                if fiber.state.compare_exchange(PARKED, RUNNABLE, SeqCst, SeqCst).is_ok() {
                    lock().push(Arc::clone(fiber));
                }
            }
        }
    }

    /// Whether both name the same context.
    pub(crate) fn is(&self, other: &Context) -> bool {
        match (self, other) {
            (Context::Thread(a), Context::Thread(b)) => a.id() == b.id(),
            (Context::Helper(a), Context::Helper(b)) => Arc::ptr_eq(a, b),
            (Context::Fiber(a), Context::Fiber(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

/// Everything the scheduler threads share, behind one lock.
struct Shared {
    /// The run queue: every fiber in the `RUNNABLE` state, oldest first.
    run: VecDeque<Arc<Fiber>>,
    /// The timer queue: fibers in a timed park, by deadline. The second
    /// key component makes equal deadlines distinct.
    timers: BTreeMap<(Instant, u64), Arc<Fiber>>,
    timer_seq: u64,
    /// Worker threads alive.
    workers: usize,
    /// Worker threads not running a fiber: waiting on [`IDLE`], or
    /// started and not yet at their first look at the queue.
    idle: usize,
    /// How many threads are already on their way to the queue for a
    /// fiber that was queued: idle workers that were notified, workers
    /// that were started, helping threads that were woken. One queued
    /// fiber claims one of them, so a second fiber does not count on the
    /// same thread.
    roused: usize,
    /// Helping threads registered.
    helpers: usize,
    /// Helping threads asleep in a park with nothing to run, unclaimed.
    asleep: Vec<Arc<Helper>>,
    /// Scheduler contexts inside a blocking region.
    blocked: usize,
    /// Whether a thread the scheduler does not own has spawned a fiber.
    /// Such a thread cannot run fibers itself, so from then on one worker
    /// is allowed whatever the budget says.
    foreign: bool,
    /// Stacks of finished fibers, kept mapped (at most [`STACK_CACHE`]).
    stacks: Vec<usize>,
}

static SHARED: Mutex<Shared> = Mutex::new(Shared {
    run: VecDeque::new(),
    timers: BTreeMap::new(),
    timer_seq: 0,
    workers: 0,
    idle: 0,
    roused: 0,
    helpers: 0,
    asleep: Vec::new(),
    blocked: 0,
    foreign: false,
    stacks: Vec::new(),
});

/// Where idle workers wait for the run queue to fill.
static IDLE: Condvar = Condvar::new();

/// The run queue's length, readable without the lock: a running fiber
/// asks it whether yielding would let anyone else run.
static QUEUED: AtomicUsize = AtomicUsize::new(0);

static SPAWNED: AtomicU64 = AtomicU64::new(0);
static SWITCHES: AtomicU64 = AtomicU64::new(0);
static THREADS: AtomicU64 = AtomicU64::new(0);
static MAPPED: AtomicUsize = AtomicUsize::new(0);

fn lock() -> MutexGuard<'static, Shared> {
    SHARED.lock().unwrap_or_else(|e| e.into_inner())
}

impl Shared {
    /// How many worker threads may exist: the budget less the helping
    /// threads, which are workers themselves, plus one for every context
    /// in a blocking region.
    fn allowed(&self) -> usize {
        let own = crate::par::budget().saturating_sub(self.helpers);
        own.max(usize::from(self.foreign)) + self.blocked
    }

    /// Queue a fiber whose state the caller just made `RUNNABLE`.
    fn push(&mut self, fiber: Arc<Fiber>) {
        self.run.push_back(fiber);
        QUEUED.store(self.run.len(), Relaxed);
        self.rouse();
    }

    fn pop(&mut self) -> Option<Arc<Fiber>> {
        let fiber = self.run.pop_front()?;
        QUEUED.store(self.run.len(), Relaxed);
        Some(fiber)
    }

    /// Get one more thread to look at the run queue: a sleeper if there
    /// is one, else a new worker if the budget allows. With neither, the
    /// threads already running fibers reach the queue when their fiber
    /// suspends or ends.
    fn rouse(&mut self) {
        if self.wake_sleeper() {
            return;
        }
        if self.workers < self.allowed() {
            // A worker that is starting is an idle one already claimed.
            self.workers += 1;
            self.idle += 1;
            self.roused += 1;
            start_worker();
        }
    }

    /// Wake one sleeping scheduler thread that nothing has claimed yet,
    /// if there is one.
    fn wake_sleeper(&mut self) -> bool {
        if self.idle > self.roused {
            self.roused += 1;
            IDLE.notify_one();
            true
        } else if let Some(helper) = self.asleep.pop() {
            self.roused += 1;
            helper.wake.notify_one();
            true
        } else {
            false
        }
    }

    /// A worker has reached the queue, from its start or from a wait:
    /// it is no longer idle, and the claim that brought it is spent. A
    /// worker that woke for a timer spends a claim meant for another,
    /// which that one then finds already spent: the count never stays
    /// above the threads still on their way.
    fn arrive(&mut self) {
        self.idle -= 1;
        self.roused = self.roused.saturating_sub(1);
    }

    /// Whether a queued fiber has no thread on its way to it.
    fn unclaimed(&self) -> bool {
        self.run.len() > self.roused
    }

    /// Resume every fiber whose timed park ended at or before `now`.
    fn fire(&mut self, now: Instant) {
        while self.timers.first_key_value().is_some_and(|(key, _)| key.0 <= now) {
            let Some((_, fiber)) = self.timers.pop_first() else {
                return;
            };
            fiber.token.store(true, SeqCst);
            if fiber.state.compare_exchange(PARKED, RUNNABLE, SeqCst, SeqCst).is_ok() {
                self.push(fiber);
            }
        }
    }

    /// Fire the due timers and answer the next fiber to run, or how long
    /// a thread with nothing to run may sleep before a timer needs it.
    fn next(&mut self) -> Result<Arc<Fiber>, Option<Duration>> {
        let mut sleep = None;
        if !self.timers.is_empty() {
            let now = Instant::now();
            self.fire(now);
            sleep = self.timers.first_key_value().map(|(key, _)| key.0.saturating_duration_since(now));
        }
        self.pop().ok_or(sleep)
    }
}

/// Start one worker thread. The caller counted it already, as a worker
/// and as an idle one that has been roused.
#[cold]
#[inline(never)]
fn start_worker() {
    THREADS.fetch_add(1, Relaxed);
    std::thread::Builder::new()
        .name("cinch-sched".into())
        .stack_size(WORKER_STACK_BYTES)
        .spawn(|| stack::guarded(worker))
        .unwrap_or_else(|e| stack::spawn_refused(&e));
}

/// A worker thread's life: run what is queued, sleep when nothing is,
/// leave when it is idle and the budget no longer has room for it.
fn worker() {
    let mut shared = lock();
    shared.arrive();
    loop {
        match shared.next() {
            Ok(fiber) => {
                drop(shared);
                run(fiber);
                shared = lock();
            }
            Err(sleep) => {
                if shared.workers > shared.allowed() {
                    shared.workers -= 1;
                    drop(shared);
                    release_spare();
                    return;
                }
                shared.idle += 1;
                shared = match sleep {
                    Some(timeout) => IDLE.wait_timeout(shared, timeout).unwrap_or_else(|e| e.into_inner()).0,
                    None => IDLE.wait(shared).unwrap_or_else(|e| e.into_inner()),
                };
                shared.arrive();
            }
        }
    }
}

/// Switch into `fiber` on the calling thread and, once it switches back,
/// finish the transition it asked for. The caller took `fiber` from the
/// run queue, so it is the only thread that holds it.
fn run(fiber: Arc<Fiber>) {
    unsafe {
        if *fiber.stack.get() == 0 {
            let base = obtain_stack();
            *fiber.stack.get() = base;
            *fiber.zone.get() = Zone { lo: base, hi: base + stack::GUARD_BYTES };
            *fiber.sp.get() = switch::prepare(base + STACK_BYTES, fiber_main, Arc::as_ptr(&fiber) as usize);
        }
    }
    fiber.state.store(RUNNING, SeqCst);
    // The thread's own state waits in this frame while the fiber's is
    // installed. This frame never moves to another thread (a fiber
    // switches back to the context of the thread it is running on), so
    // the thread-local reads on both sides of the switch are the same
    // thread's.
    let own = unsafe { std::ptr::replace(fiber.locals.get(), Locals::of(None)) }.install();
    let own_zone = unsafe { stack::swap_zone(fiber.zone.get()) };
    RUN.with(|run| {
        run.fiber.set(Arc::as_ptr(&fiber));
        unsafe { switch::switch(run.sched_sp.as_ptr(), *fiber.sp.get()) };
        run.fiber.set(null());
    });
    unsafe {
        stack::swap_zone(own_zone);
        *fiber.locals.get() = own.install();
    }
    SWITCHES.fetch_add(1, Relaxed);
    match fiber.state.load(SeqCst) {
        PARKING => {
            // The fiber's registers are saved and this thread is off its
            // stack: from this store on, any thread may resume it.
            fiber.state.store(PARKED, SeqCst);
            // An unpark that found the fiber still `PARKING` left its
            // token and nothing else. Either that unpark's exchange or
            // this one queues the fiber, never both.
            if fiber.token.load(SeqCst)
                && fiber.state.compare_exchange(PARKED, RUNNABLE, SeqCst, SeqCst).is_ok()
            {
                lock().push(fiber);
            }
        }
        DONE => {
            let base = unsafe {
                *fiber.locals.get() = Locals::of(None);
                *fiber.zone.get() = NO_ZONE;
                std::mem::replace(&mut *fiber.stack.get(), 0)
            };
            recycle_stack(base);
        }
        _ => crate::abi::panic_exit(b"compiler bug: a fiber switched out in a state it cannot leave from"),
    }
}

/// The first frame of every fiber.
extern "C" fn fiber_main(fiber: usize) -> ! {
    let fiber = unsafe { &*(fiber as *const Fiber) };
    if let Some(body) = unsafe { (*fiber.body.get()).take() } {
        body();
    }
    fiber.state.store(DONE, SeqCst);
    suspend(fiber);
    crate::abi::panic_exit(b"compiler bug: a finished fiber was resumed")
}

/// Leave `fiber`, the fiber the calling thread is running, for the
/// thread's scheduler context. Returns when a scheduler thread resumes
/// the fiber, which may be a different thread: the caller reads nothing
/// thread-local it found before the call.
#[inline(never)]
fn suspend(fiber: &Fiber) {
    let scheduler = RUN.with(|run| run.sched_sp.get());
    unsafe { switch::switch(fiber.sp.get(), scheduler) };
}

/// Park `fiber`, the running fiber, with its state already `RUNNING`:
/// record the intent, leave, and consume the token on the way back in.
fn park_fiber(fiber: &Fiber) {
    fiber.state.store(PARKING, SeqCst);
    suspend(fiber);
    fiber.token.store(false, SeqCst);
}

/// The zone of a fiber that has no stack: no address is in it.
const NO_ZONE: Zone = Zone { lo: 0, hi: 0 };

/// Reserve a fiber's stack: address space only, committed as it is
/// touched, whose low [`stack::GUARD_BYTES`] stay inaccessible (the
/// fiber's fault zone). Answers the mapping's lowest address, or `None`
/// when the system has no address space or mapping left for it.
fn map_stack() -> Option<usize> {
    let (base, _) = crate::sys::mmap_reserve(STACK_BYTES);
    if base < 0 {
        return None;
    }
    let usable = base + stack::GUARD_BYTES as i64;
    let access = stack::PROT_READ | stack::PROT_WRITE;
    if crate::sys::mprotect(usable, STACK_BYTES - stack::GUARD_BYTES, access).0 != 0 {
        crate::sys::munmap(base, STACK_BYTES);
        return None;
    }
    MAPPED.fetch_add(1, Relaxed);
    Some(base as usize)
}

fn unmap_stack(base: usize) {
    crate::sys::munmap(base as i64, STACK_BYTES);
    MAPPED.fetch_sub(1, Relaxed);
}

/// A stack for a fiber's first run: this thread's spare one, a cached
/// one, or a new mapping. A refused mapping is the stack `Exhausted`
/// exit.
fn obtain_stack() -> usize {
    let spare = RUN.with(|run| run.spare.replace(0));
    if spare != 0 {
        return spare;
    }
    if let Some(base) = lock().stacks.pop() {
        return base;
    }
    map_stack().unwrap_or_else(|| crate::abi::exhausted(crate::abi::Resource::Stack))
}

/// Keep a finished fiber's stack for the next one: on this thread if it
/// has none, else in the shared cache, else not at all.
fn recycle_stack(base: usize) {
    let held = RUN.with(|run| {
        if run.spare.get() == 0 {
            run.spare.set(base);
            true
        } else {
            false
        }
    });
    if !held {
        cache_stack(base);
    }
}

fn cache_stack(base: usize) {
    let mut shared = lock();
    if shared.stacks.len() < STACK_CACHE {
        shared.stacks.push(base);
        return;
    }
    drop(shared);
    unmap_stack(base);
}

/// Hand this thread's spare stack on before the thread stops running
/// fibers.
fn release_spare() {
    let spare = RUN.with(|run| run.spare.replace(0));
    if spare != 0 {
        cache_stack(spare);
    }
}

/// The calling context.
#[inline(never)]
pub(crate) fn current() -> Context {
    let (fiber, helper) = RUN.with(|run| (run.fiber.get(), run.helper.get()));
    if !fiber.is_null() {
        Context::Fiber(unsafe { share(fiber) })
    } else if !helper.is_null() {
        Context::Helper(unsafe { share(helper) })
    } else {
        Context::Thread(std::thread::current())
    }
}

/// Another reference to the record `this` points at.
///
/// # Safety
///
/// `this` came from `Arc::as_ptr` of a reference that is still held.
unsafe fn share<T>(this: *const T) -> Arc<T> {
    unsafe {
        Arc::increment_strong_count(this);
        Arc::from_raw(this)
    }
}

/// Suspend the calling context until its token arrives.
#[inline(never)]
pub(crate) fn park() {
    let (fiber, helper) = RUN.with(|run| (run.fiber.get(), run.helper.get()));
    if !fiber.is_null() {
        let fiber = unsafe { &*fiber };
        if !fiber.token.swap(false, SeqCst) {
            park_fiber(fiber);
        }
    } else if !helper.is_null() {
        help(helper, None);
    } else {
        std::thread::park();
    }
}

/// Suspend the calling context until its token arrives or `timeout` has
/// passed.
#[inline(never)]
pub(crate) fn park_timeout(timeout: Duration) {
    let (fiber, helper) = RUN.with(|run| (run.fiber.get(), run.helper.get()));
    // A deadline past the clock's range never arrives.
    let deadline = Instant::now().checked_add(timeout);
    if !fiber.is_null() {
        let this = unsafe { &*fiber };
        if this.token.swap(false, SeqCst) {
            return;
        }
        let timer = deadline.map(|at| add_timer(at, unsafe { share(fiber) }));
        park_fiber(this);
        if let Some(key) = timer {
            lock().timers.remove(&key);
        }
    } else if !helper.is_null() {
        help(helper, deadline);
    } else {
        std::thread::park_timeout(timeout);
    }
}

/// Queue `fiber` to be resumed at `at`, and answer the timer's key.
fn add_timer(at: Instant, fiber: Arc<Fiber>) -> (Instant, u64) {
    let mut shared = lock();
    shared.timer_seq += 1;
    let key = (at, shared.timer_seq);
    shared.timers.insert(key, fiber);
    // A sleeper waits no longer than the earliest timer it saw. This one
    // is earlier: one sleeper reads the queue again.
    if shared.timers.first_key_value().is_some_and(|(first, _)| *first == key) {
        shared.wake_sleeper();
    }
    key
}

/// A helping thread's park: run fibers until the thread's own token
/// arrives or `deadline` passes, and sleep while there is none to run.
fn help(helper: *const Helper, deadline: Option<Instant>) {
    let this = unsafe { &*helper };
    loop {
        if this.token.swap(false, SeqCst) {
            return;
        }
        let mut shared = lock();
        let fiber = loop {
            if this.token.swap(false, SeqCst) || deadline.is_some_and(|at| Instant::now() >= at) {
                // A queued fiber may have woken this thread for itself:
                // the thread is leaving, so another one is asked to look.
                if shared.unclaimed() {
                    shared.rouse();
                }
                return;
            }
            match shared.next() {
                Ok(fiber) => break fiber,
                Err(sleep) => {
                    let own = deadline.map(|at| at.saturating_duration_since(Instant::now()));
                    let timeout = match (sleep, own) {
                        (Some(a), Some(b)) => Some(a.min(b)),
                        (a, b) => a.or(b),
                    };
                    let me = unsafe { share(helper) };
                    shared.asleep.push(Arc::clone(&me));
                    shared = match timeout {
                        Some(t) => this.wake.wait_timeout(shared, t).unwrap_or_else(|e| e.into_inner()).0,
                        None => this.wake.wait(shared).unwrap_or_else(|e| e.into_inner()),
                    };
                    // Still listed: the thread woke for its own token or
                    // a deadline. Taken off the list: a queued fiber
                    // claimed it, and the claim is spent now.
                    let listed = shared.asleep.len();
                    shared.asleep.retain(|other| !Arc::ptr_eq(other, &me));
                    if shared.asleep.len() == listed {
                        shared.roused = shared.roused.saturating_sub(1);
                    }
                }
            }
        };
        drop(shared);
        RUN.with(|run| run.deadline.set(deadline));
        run(fiber);
        RUN.with(|run| run.deadline.set(None));
    }
}

/// Give the processor to another runnable fiber, if one waits or the
/// thread underneath wants its own context back. A helping thread that
/// is not in a fiber runs one queued fiber instead.
#[inline(never)]
pub(crate) fn yield_now() {
    let (fiber, helper, deadline) = RUN.with(|run| (run.fiber.get(), run.helper.get(), run.deadline.get()));
    if fiber.is_null() {
        if !helper.is_null() && QUEUED.load(Relaxed) != 0 {
            let next = lock().next();
            if let Ok(next) = next {
                // The fiber gives the thread back at its next yield.
                RUN.with(|run| run.deadline.set(Some(Instant::now())));
                run(next);
                RUN.with(|run| run.deadline.set(None));
            }
        }
        return;
    }
    let host_waits = !helper.is_null()
        && (unsafe { (*helper).token.load(SeqCst) } || deadline.is_some_and(|at| Instant::now() >= at));
    if QUEUED.load(Relaxed) == 0 && !host_waits {
        return;
    }
    // A yield is a park whose wake has already happened: the scheduler
    // finds the token and queues the fiber behind the ones waiting.
    let fiber = unsafe { &*fiber };
    fiber.token.store(true, SeqCst);
    park_fiber(fiber);
}

/// Run `body` as a fiber with `task` as its running task.
pub(crate) fn spawn(task: Arc<TaskState>, body: Box<dyn FnOnce() + Send>) {
    let fiber = Arc::new(Fiber {
        state: AtomicU8::new(RUNNABLE),
        token: AtomicBool::new(false),
        blocking: AtomicU32::new(0),
        sp: UnsafeCell::new(0),
        stack: UnsafeCell::new(0),
        zone: UnsafeCell::new(NO_ZONE),
        body: UnsafeCell::new(Some(body)),
        locals: UnsafeCell::new(Locals::of(Some(Arc::clone(&task)))),
    });
    // The task can be cancelled as soon as it can run: its context is
    // recorded before the fiber is queued.
    task.set_parker(crate::park::Parker::of(Context::Fiber(Arc::clone(&fiber))));
    SPAWNED.fetch_add(1, Relaxed);
    let owned = RUN.with(|run| !run.fiber.get().is_null() || !run.helper.get().is_null());
    let mut shared = lock();
    if !owned {
        shared.foreign = true;
    }
    shared.push(fiber);
}

/// The blocking depth counter of the calling context, when it is one the
/// budget counts (a fiber or a helping thread).
fn blocking_depth() -> Option<&'static AtomicU32> {
    let (fiber, helper) = RUN.with(|run| (run.fiber.get(), run.helper.get()));
    // The records outlive the region: a fiber's is held by the thread
    // running it, a helper's by its registration.
    if !fiber.is_null() {
        Some(unsafe { &(*fiber).blocking })
    } else if !helper.is_null() {
        Some(unsafe { &(*helper).blocking })
    } else {
        None
    }
}

/// The calling context is about to block its thread in the operating
/// system.
#[inline(never)]
pub(crate) fn blocking_enter() {
    let Some(depth) = blocking_depth() else {
        return;
    };
    if depth.fetch_add(1, Relaxed) != 0 {
        return;
    }
    let mut shared = lock();
    shared.blocked += 1;
    // This thread stops taking fibers. If one is queued with no thread
    // on its way to it, the allowance that just grew starts a worker.
    if shared.unclaimed() {
        shared.rouse();
    }
}

/// The calling context's thread is back from the operating system.
#[inline(never)]
pub(crate) fn blocking_exit() {
    let Some(depth) = blocking_depth() else {
        return;
    };
    if depth.fetch_sub(1, Relaxed) != 1 {
        return;
    }
    let mut shared = lock();
    shared.blocked -= 1;
    // The worker that stood in is surplus once it is idle: let an idle
    // one find that out now.
    if shared.workers > shared.allowed() && shared.idle > shared.roused {
        shared.roused += 1;
        IDLE.notify_one();
    }
}

/// Run `body` with the calling thread registered as a helping thread.
pub(crate) fn program_thread<R>(body: impl FnOnce() -> R) -> R {
    let helper = Arc::new(Helper {
        token: AtomicBool::new(false),
        wake: Condvar::new(),
        blocking: AtomicU32::new(0),
    });
    lock().helpers += 1;
    RUN.with(|run| run.helper.set(Arc::as_ptr(&helper)));
    let result = body();
    RUN.with(|run| run.helper.set(null()));
    lock().helpers -= 1;
    release_spare();
    result
}

/// The lowest stack address at which the calling fiber may still take
/// more work onto its stack (half of the stack remains above it), or
/// `None` when the caller is not a fiber.
#[inline(never)]
pub(crate) fn help_floor() -> Option<usize> {
    let fiber = RUN.with(|run| run.fiber.get());
    if fiber.is_null() {
        return None;
    }
    let base = unsafe { *(*fiber).stack.get() };
    Some(base + STACK_BYTES / 2)
}

pub(super) fn stats() -> Stats {
    let shared = lock();
    Stats {
        fibers: true,
        workers: shared.workers,
        idle: shared.idle,
        queued: shared.run.len(),
        blocked: shared.blocked,
        timers: shared.timers.len(),
        spawned: SPAWNED.load(Relaxed),
        switches: SWITCHES.load(Relaxed),
        threads_started: THREADS.load(Relaxed),
        stacks_mapped: MAPPED.load(Relaxed),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One context that records each time it is switched in.
    struct Probe {
        own: usize,
        other: usize,
        seen: Vec<usize>,
    }

    extern "C" fn probe_main(arg: usize) -> ! {
        let probe = arg as *mut Probe;
        unsafe {
            for round in [0, 1, 2, 99] {
                (*probe).seen.push(round);
                switch::switch(&raw mut (*probe).own, (*probe).other);
            }
        }
        unreachable!("a finished context was resumed");
    }

    #[test]
    fn a_context_runs_on_its_own_stack_and_resumes_where_it_left() {
        let base = map_stack().expect("a stack");
        let probe = Box::into_raw(Box::new(Probe { own: 0, other: 0, seen: Vec::new() }));
        unsafe {
            (*probe).own = switch::prepare(base + STACK_BYTES, probe_main, probe as usize);
            for round in 0..4 {
                // Callee-saved state on this side survives each round trip.
                let kept = std::hint::black_box(round as f64 * 1.5);
                switch::switch(&raw mut (*probe).other, (*probe).own);
                assert_eq!((*probe).seen.len(), round + 1);
                assert_eq!(kept, round as f64 * 1.5);
            }
            assert_eq!((*probe).seen, [0, 1, 2, 99]);
            // The context's frames were on the mapped stack, above its band.
            assert!((base + stack::GUARD_BYTES..base + STACK_BYTES).contains(&(*probe).own));
            drop(Box::from_raw(probe));
        }
        unmap_stack(base);
    }
}
