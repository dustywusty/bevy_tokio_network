use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

pub fn start_worker_thread() -> NetworkResource {
}