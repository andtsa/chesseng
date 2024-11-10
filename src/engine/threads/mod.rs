// use std::cell::RefCell;
// use std::sync::Arc;
// use std::sync::Mutex;
// use std::thread::JoinHandle;
//
// use anyhow::anyhow;
// use anyhow::Result;
// use crossbeam_deque::Injector;
// use crossbeam_deque::Steal;
// use crossbeam_deque::Stealer;
// use crossbeam_deque::Worker;
// use rayon::ThreadPoolBuilder;
//
// use crate::search::process::run_thread;
// use crate::search::Node;
//
// /// Number of threads to use for search
// pub const THREAD_COUNT: usize = 8;
// /// stack size for each thread in bytes. 32MB
// pub const THREAD_STACK_SIZE: usize = 32 * 1024 * 1024;
//
// #[derive(Debug)]
// pub struct SandPool {
//     // pv_injector: Arc<Injector<PvNode>>,
//     node_injector: Arc<Injector<Node>>,
// }
//
// /// # Initialise the work stealing resources and the engine's threads
// /// 1. Create the principal queue, to share [`Node`]s between threads
// pub fn create_threads() -> Result<SandPool> {
//     let node_injector: Arc<Injector<Node>> = Arc::new(Injector::new());
//
//     let stealers = Arc::new(Mutex::new(vec![]));
//     let pool = ThreadPoolBuilder::new()
//         .num_threads(THREAD_COUNT)
//         .stack_size(THREAD_STACK_SIZE)
//         .exit_handler(exit_thread)
//         .build()?;
//
//     let handles: Vec<JoinHandle<()>> = vec![];
//
//     for i in 0..THREAD_COUNT {
//         let mut worker: Worker<Node> = Worker::new_fifo();
//         let mut stealer = worker.stealer();
//         stealers
//             .lock()
//             .and_then(|mut x| {
//                 x.push(stealer);
//                 Ok(())
//             })
//             .map_err(|e| anyhow!("poison error: {e:?}"))?;
//         let queue_ref = node_injector.clone();
//         let stealer_ref = stealers.clone();
//
//         pool.spawn(move || run_thread(queue_ref, worker, stealer_ref))
//     }
//
//     todo!()
// }
//
// pub fn exit_thread(idx: usize) {
//     todo!()
// }
