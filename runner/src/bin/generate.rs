// use std::collections::{HashMap, HashSet};
use runner::common::*;
use std::collections::HashSet;

fn main() {
   
    // Predetermined task set.
    // To change task set, comment out the rows from 10-119, and insert your own task set below.
    let t1 = Task {
        id: "T1".to_string(),
        prio: 1,
        deadline: 600,
        inter_arrival: 600,
        trace: Trace {
            id: "T1".to_string(),
            start: 0,
            end: 30,
            inner: vec![],
        },
    };

    let t2 = Task {
        id: "T2".to_string(),
        prio: 2,
        deadline: 400,
        inter_arrival: 400,
        trace: Trace {
            id: "T2".to_string(),
            start: 0,
            end: 30,
            inner: vec![
                Trace {
                    id: "R1".to_string(),
                    start: 10,
                    end: 20,
                    inner: vec![Trace {
                        id: "R3".to_string(),
                        start: 12,
                        end: 16,
                        inner: vec![],
                    }],
                },
                Trace {
                    id: "R2".to_string(),
                    start: 22,
                    end: 23,
                    inner: vec![],
                },
            ],
        },
    };

    let t3 = Task {
        id: "T3".to_string(),
        prio: 2,
        deadline: 300,
        inter_arrival: 300,
        trace: Trace {
            id: "T3".to_string(),
            start: 0,
            end: 40,
            inner: vec![
                Trace {
                    id: "R3".to_string(),
                    start: 10,
                    end: 18,
                    inner: vec![Trace {
                        id: "R1".to_string(),
                        start: 12,
                        end: 16,
                        inner: vec![],
                    }],
                },
                Trace {
                    id: "R1".to_string(),
                    start: 32,
                    end: 34,
                    inner: vec![],
                },
            ],
        },
    };

    let t4 = Task {
        id: "T4".to_string(),
        prio: 3,
        deadline: 70,
        inter_arrival: 70,
        trace: Trace {
            id: "T4".to_string(),
            start: 0,
            end: 15,
            inner: vec![Trace {
                id: "R1".to_string(),
                start: 10,
                end: 14,
                inner: vec![],
            }],
        },
    };

    let t5 = Task {
        id: "T5".to_string(),
        prio: 3,
        deadline: 180,
        inter_arrival: 180,
        trace: Trace {
            id: "T5".to_string(),
            start: 0,
            end: 20,
            inner: vec![Trace {
                id: "R3".to_string(),
                start: 8,
                end: 12,
                inner: vec![],
            }],
        },
    };

    // builds a vector of tasks t1, t2, t3, t4, t5
    let tasks: Tasks = vec![t1, t2, t3, t4, t5];
    

    let (ip, tr, tm) = pre_analysis(&tasks);
    println!("Task prio: {:?}", ip);
    println!("TaskResources: {:?}", tr);
    println!("TaskMaps: {:?}", tm);
    println!("CPU Load Factor: {}", load_factor(&tasks));

    //Predetermines the method used for preemtion calculations, set the string value inside String::from() to either "exact" or "estimate"
    let str:String = String::from("estimate");

    result(&tasks, &ip, &tr, &tm, str);

}

// Function that calculates the load factor for the CPU.
// Calculates the value for seperate tasks and the total load on the CPU.
// Includes a error if the load is greater than 1.
fn load_factor (tasks: &Vec<Task>) -> f32 {
    let mut loadFactor = 0.0;
    for t in tasks {
        let load = (t.trace.end - t.trace.start) as f32 / t.inter_arrival as f32;
        loadFactor += load;
        println!("{} load ratio: {:?}", t.id, load);
    }
    match errorHandler::load_factor(loadFactor) {
        Err(e) => panic!("{:?}", e),
        Ok(_load) => (),
    }
    loadFactor
}

// Function that calculates the Worst Case Execution Time (WCET) for the given task. 
fn blocked_time(task: &Task, ip: &IdPrio, tr: &TaskResources) -> f32 {
    let mut tot_block = 0.0;
    let res = tr.get(&task.id);
    if res.is_some() {
        for (k, v) in ip {
            if task.prio > *v {
                if tr.get(k).is_some() {
                    for (k_rs, _v_rs) in res.unwrap() {
                        for (k_ex, v_ex) in tr.get(k).unwrap() {
                            if (k_rs == k_ex) && (tot_block < *v_ex as f32) {
                                tot_block = *v_ex as f32;
                            }
                        }
                    }
                }
            }
        }
    }
    tot_block
}

// Function that uses match with recieved string value to determine the estimated or exact preemtion time.
// Returns the total preemtion time 
fn preemtion_time(task: &Task, ip: &IdPrio, tasks: &Vec<Task>, task_handler: &mut PrioHandler,
    tr: &TaskResources, tm: &TimeMaps, st: String) -> f32{
    let mut total = 0.0;
    total = match st.clone().as_str() {
        "exact" => {preem_exact(&task, &ip, &tasks, &mut task_handler.clone(),
            &tr, &tm, st.clone(), total)}
        "estimate" => {preem_esti(&task, &tasks, &tm, total)},
        _ => total
    };
    total 
}

// Function that calculates the exact preemtion time
fn preem_exact(task: &Task, ip: &IdPrio, tasks: &Vec<Task>, task_handler: &mut PrioHandler,
    tr: &TaskResources, tm: &TimeMaps, st: String, mut total: f32) -> f32{
    for t in tasks {
        if task.prio < t.prio && task.id != t.id {
            let rsp = response_time(&t, &ip, &tasks, &mut task_handler.clone(), &tr, &tm, st.clone());
            let vec = tm.get(&task.id);
            let arr = vec.unwrap().get(0).unwrap();
            let deq = rsp / *arr as f32;
            let wcet = vec.unwrap().get(1).unwrap();
            total += deq.ceil() * *wcet as f32;
        } else if task.prio == t.prio && task.id != t.id {
            if !task_handler.contains(&t.id.to_string()) || task_handler.is_empty() {
                task_handler.insert(task.id.clone());
                let rsp = response_time(&t, &ip, &tasks, &mut task_handler.clone(), &tr, &tm, st.clone());
                let vec = tm.get(&t.id);
                let arr = vec.unwrap().get(0).unwrap();
                let deq = rsp / *arr as f32;
                let wcet = vec.unwrap().get(1).unwrap();
                total += deq.ceil() * *wcet as f32;
            }
        }
    }
    total
}

// Function that calculates the estimated preemtion time
fn preem_esti(task: &Task, tasks: &Vec<Task>, tm: &TimeMaps, mut total: f32) -> f32{
    for t in tasks {
        if task.prio <= t.prio && task.id != t.id {
            let vec = tm.get(&t.id);
            let arr = vec.unwrap().get(0).unwrap();
            let deq = task.deadline as f32 / *arr as f32;
            let wcet = vec.unwrap().get(1).unwrap();
            total += deq.ceil() * *wcet as f32;
        }
    }
    total
}

// Function that calculates the response time for the given task.
// Includes an error handler checking the response time to the tasks deadline. If it doesn't meet the deadline, the function throws an error.
fn response_time(task: &Task, ip: &IdPrio, tasks: &Vec<Task>, mut task_handler: &mut PrioHandler,
    tr: &TaskResources, tm: &TimeMaps, st: String) -> f32 {
    let vec = tm.get(&task.id);
    let wcet = vec.unwrap().get(1).unwrap();
    let bl = blocked_time(&task, &ip, &tr);
    let pr = preemtion_time(&task, &ip, &tasks, &mut task_handler, &tr, &tm, st.clone());
    let rsp = *wcet as f32 + bl + pr;
    match errorHandler::deadline_handler(task.deadline, rsp) {
        Err(e) => panic!("{:?}", e),
        Ok(_rsp) => (),
    }
    rsp
}

// The result function, it iterates over each task and prints out a vector holding all desired information.
// Order of info in vector is: Task, Response Time, Worst Case Execution Time, Blocked Time, and Preemtion Time (exact or estimated)
fn result(tasks: &Vec<Task>, ip: &IdPrio, tr: &TaskResources, tm: &TimeMaps, st: String) {
    for t in tasks {
        let mut task_handler: PrioHandler = HashSet::new();
        let mut sep_handler: PrioHandler = HashSet::new();
        let bl = blocked_time(&t, &ip, &tr);
        let pr = preemtion_time(&t, &ip, &tasks, &mut task_handler, &tr, &tm, st.clone());
        let rt = response_time(&t, &ip, &tasks, &mut sep_handler, &tr, &tm, st.clone());
        let temp_vec = tm.get(&t.id);
        let worstcase = temp_vec.unwrap().get(1).unwrap();

        let vec = vec![
            ResultVec::Task(t.clone()),
            ResultVec::ResponseTime(rt),
            ResultVec::WorstCase(*worstcase),
            ResultVec::BlockTime(bl),
            ResultVec::PreemtionTime(pr),
        ];  

        println!("Result: {:?}", vec);
    } 
}