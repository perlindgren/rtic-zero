use crate::syntax::*;

use std::collections::HashMap;

type ResourceToPriority = HashMap<Resource, u8>;

pub fn resource_ceiling(tasks: Vec<Task>) -> ResourceToPriority {
    let mut rtp = ResourceToPriority::new();

    for t in tasks {
        for r in t.shared {
            if let Some(curr_prio) = rtp.get_mut(&r) {
                if t.priority > *curr_prio {
                    *curr_prio = t.priority;
                }
            } else {
                rtp.insert(r.clone(), t.priority);
            }
        }
    }

    rtp
}

#[test]
fn cielings() {
    let task_set = task_set();
    let rtp = resource_ceiling(task_set.tasks);
    println!("ceilings {:?}", rtp);
}
