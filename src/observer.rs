// A Logger needs to asynchronously gather and periodically
// record information on the evolutionary process.

use std::fmt::Debug;
use std::sync::{Arc, Mutex};
// a hack to make the imports more meaningful
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{JoinHandle, spawn};

use crate::configure::Configure;
use crate::evolution::{FitnessScalar, Phenome};

pub struct Observer<O: Phenome> {
    pub handle: JoinHandle<()>,
    tx: Sender<O>,
    // TODO: add a reporter struct field
}

pub type ReportFn<T> = Box<dyn Fn(&Vec<Option<T>>) -> () + Sync + Send + 'static>;

pub struct Window<O: Phenome> {
    pub frame: Vec<Option<O>>,
    i: usize,
    window_size: usize,
    report_fn: ReportFn<O>,
}

impl<O: Phenome> Window<O> {
    fn new(window_size: usize, report_fn: ReportFn<O>) -> Self {
        assert!(window_size > 0);
        Self {
            frame: vec![None; window_size],
            i: 0,
            window_size,
            report_fn,
        }
    }

    fn insert(&mut self, thing: O) {
        self.i = (self.i + 1) % self.window_size;
        self.frame[self.i] = Some(thing);
        if self.i == 0 {
            self.report();
        }
    }

    fn report(&self) {
        (self.report_fn)(&self.frame)
    }
}

impl<O: 'static + Phenome> Observer<O> {
    /// The observe method should take a clone of the observable
    /// and store in something like a sliding observation window.
    pub fn observe(&self, ob: O) {
        self.tx.send(ob).expect("tx failure");
    }

    pub fn spawn<C: Configure>(params: Arc<C>, report_fn: ReportFn<O>) -> Observer<O> {
        let (tx, rx): (Sender<O>, Receiver<O>) = channel();

        let window_size: usize = params.observer_window_size();

        let handle: JoinHandle<()> = spawn(move || {
            let mut window: Window<O> = Window::new(window_size, report_fn);
            for observable in rx {
                log::debug!("received observable {:?}", observable);
                window.insert(observable);
            }
        });

        Observer { handle, tx }
    }
}

/*
// Let's try a horrible macro
// Jackie, I'm sorry.
// TODO: try to replace this with generic programming trick
#[macro_export]
macro_rules! build_observation_mod {
    ($mod_name:ident, $observable:ty, $config: ty) => {
        mod $mod_name {
            use std::sync::mpsc::{channel, SendError, Sender};
            use std::thread::{spawn, JoinHandle};
            use std::fmt::Debug;

            use crate::observer::{ObservationWindow, Observe};

            use super::*;

            pub struct Observer {
                pub handle: JoinHandle<()>,
                tx: Sender<$observable>,
            }

            pub struct Window<O: Phenome + Send + Debug> {
                pub frame: Vec<Option<O>>,
                i: usize,
                window_size: usize,
            }

            impl ObservationWindow for Window<$observable> {

                fn insert(&mut self, thing: $observable) {
                    self.i = (self.i + 1) % self.window_size;
                    self.frame[self.i] = Some(thing);
                    if self.i == 0 {
                        self.report();
                    }
                }

                fn report(&self) {
                    let fitnesses: Vec<usize> = self
                        .frame
                        .iter()
                        .filter_map(|t| t.as_ref().and_then(<$observable>::fitness))
                        .collect();
                    let avg_fit = fitnesses.iter().sum::<usize>() as f32 / fitnesses.len() as f32;
                    log::info!("Average fitness: {}", avg_fit);
                }


                fn new<O: Phenome + Send>(window_size: usize) -> Self {
                    assert!(window_size > 0);
                    Self {
                        frame: vec![None; window_size],
                        i: 0,
                        window_size,
                    }
                }
            }

            impl Observe for Observer {
                type Observable = $observable;
                type Params = $config;

                type Window = Window<Self::Observable>;

                fn observe(&self, ob: Self::Observable) {
                    self.tx.send(ob).expect("tx failure");
                }

            }
        }
    };
}
*/
