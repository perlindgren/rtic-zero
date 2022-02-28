use crate::syntax::*;

use std::collections::HashMap;

pub type ResourceToPriority = HashMap<Resource, u8>;

pub fn resource_ceiling(task_set: &TaskSet) -> ResourceToPriority {
    let mut tasks = task_set.tasks.clone();

    if let Some(idle) = &task_set.idle {
        tasks.push({
            Task {
                id: "idle".into(),
                priority: 0,
                binds: "".into(),
                shared: idle.shared.clone(),
                local: vec![],
            }
        })
    }
    let mut rtp = ResourceToPriority::new();

    for t in tasks {
        for r in &t.shared {
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
    let rtp = resource_ceiling(&task_set);
    println!("ceilings {:?}", rtp);
}
