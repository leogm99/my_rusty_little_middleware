use std::sync::Arc;
use std::thread::{self, JoinHandle};

/// Defines the entry point for executing an instance of an object
///
/// It's usable within the context of a type which implements the Runnable trait,
/// meaning that this is the code that will get called by a thread upon calling `run`
pub trait Startable {
    fn start(&self);
}

/// Utility that lets the type call the `start` method defined for it in a new thread.
///
/// Similar in spirit to Java's Runnable interface
/// NEEDS REFACTOR
pub trait Runnable {
    fn run<T: 'static + Startable + std::marker::Sync + std::marker::Send>(
        instance: &Arc<T>,
    ) -> Result<JoinHandle<()>, ()> {
        let instance = Arc::clone(&instance);
        Ok(thread::spawn(move || instance.start()))
    }
}
