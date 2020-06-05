use std::collections::{HashMap, HashSet};

pub mod errorHandler {
    #[derive(Debug)]

    // Constructor for the errors.
    pub enum ComingError {
        DeadlineMiss,
        CPUOutOfBounds,
    }

    pub type ErrorResult = Result<f32, ComingError>;

    //Error handler for load factor
    pub fn load_factor(l: f32) -> ErrorResult {
        if l > 1.0 {
            Err(ComingError::CPUOutOfBounds)
        } else {
            Ok(l)
        }
    }

    //Error handler for a deadline miss
    pub fn deadline_handler(dl: u32, rsp: f32) -> ErrorResult {
        if rsp > dl as f32 {
            Err(ComingError::DeadlineMiss)
        } else {
            Ok(rsp)
        }
    }
}

// common data structures
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub prio: u8,
    pub deadline: u32,
    pub inter_arrival: u32,
    pub trace: Trace,
}

#[derive(Debug, Clone)]
pub struct Trace {
    pub id: String,
    pub start: u32,
    pub end: u32,
    pub inner: Vec<Trace>,
}

pub type ResponseTime = f32;
pub type BlockTime = f32;
pub type PreemtionTime = f32;
pub type WorstCase = u32;

#[derive(Debug)]
pub enum ResultVec {
    Task(Task),
    ResponseTime(f32),
    BlockTime(f32),
    PreemtionTime(f32),
    WorstCase(u32),
}


// uselful types

// Our task set
pub type Tasks = Vec<Task>;

// Hash set for used tasks.
pub type PrioHandler = HashSet<String>;

// A map from Task/Resource identifiers to priority
pub type IdPrio = HashMap<String, u8>;

// A map from Task identifiers to a set of Resource identifiers
pub type TaskResources = HashMap<String, HashMap<String, u32>>;

// A map for wcet and interarrival for each task.
pub type TimeMaps = HashMap<String, Vec<u32>>;


// Derives the above maps from a set of tasks
pub fn pre_analysis(tasks: &Tasks) -> (IdPrio, TaskResources, TimeMaps) {
    let mut ip = HashMap::new();
    let mut tr: TaskResources = HashMap::new();
    let mut tm: TimeMaps = HashMap::new();
    for t in tasks {
        update_prio(t.prio, &t.trace, &mut ip);
        update_tm(t.id.clone(), t.inter_arrival, &t.trace, &mut tm);
        for i in &t.trace.inner {
            update_tr(t.id.clone(), i, &mut tr);
        }
    }
    (ip, tr, tm)
}

// helper functions
fn update_tm(t_id: String, t_arr: u32, trace: &Trace, tm: &mut TimeMaps) {
    let t_wcet = trace.end - trace.start;
    let vec = vec![t_arr, t_wcet];
    tm.insert(t_id, vec);
}

fn update_prio(prio: u8, trace: &Trace, hm: &mut IdPrio) {
    if let Some(old_prio) = hm.get(&trace.id) {
        if prio > *old_prio {
            hm.insert(trace.id.clone(), prio);
        }
    } else {
        hm.insert(trace.id.clone(), prio);
    } 
}

fn update_tr(s: String, trace: &Trace, trmap: &mut TaskResources) {
    if let Some(seen) = trmap.get_mut(&s) {
        if seen.get(&trace.id).is_some() {
            if seen.get(&trace.id).unwrap() < &(trace.end - trace.start) {
                seen.insert(trace.id.clone(), trace.end.clone() - trace.start.clone());
            }
        } else {
            seen.insert(trace.id.clone(), trace.end.clone() - trace.start.clone());
        }
    } else {
        let mut hm = HashMap::new();
        hm.insert(trace.id.clone(), trace.end.clone() - trace.start.clone());
        trmap.insert(s.clone(), hm);
    }
    for trace in &trace.inner {
        update_tr(s.clone(), trace, trmap);
    }
}