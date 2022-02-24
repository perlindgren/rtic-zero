//

use std::collections::{HashMap, HashSet};

use std::rc::Rc;

type R = Rc<String>;
type T = Rc<String>;
type RS = HashSet<R>;

type TasksToResources = HashMap<T, RS>;

fn ttr_add(ttr: &mut TasksToResources, t: &T, r: &R) {
    if let Some(s) = ttr.get_mut(t) {
        s.insert(r.clone());
    } else {
        let mut hs = RS::new();
        hs.insert(r.clone());
        ttr.insert(t.clone(), hs);
    }
}

type TasksToPriorities = HashMap<T, u8>;
type ResourceToPriorities = HashMap<R, u8>;

fn cielings(ttr: &TasksToResources, ttp: &TasksToPriorities) -> ResourceToPriorities {
    let mut rtp = ResourceToPriorities::new();

    for (t, r) in ttr {
        println!("task {:?}", t);
        let p = *ttp.get(t).unwrap();
        println!("priority {:?}", p);
        for r in r {
            println!("res {:?}", r);
            if let Some(curr_prio) = rtp.get_mut(r) {
                if p > *curr_prio {
                    *curr_prio = p;
                }
            } else {
                rtp.insert(r.clone(), p);
            }
        }
    }

    rtp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let r1 = R::new("r1".into());
        let r2 = R::new("r2".into());
        let r3 = R::new("r3".into());

        let t1 = T::new("t1".into());
        let t2 = T::new("t2".into());
        let t3 = T::new("t3".into());

        let mut ttr = TasksToResources::new();
        let mut ttp = TasksToPriorities::new();

        ttp.insert(t1.clone(), 1); // task 1, priority 1
        ttp.insert(t2.clone(), 2); // task 2, priority 2
        ttp.insert(t3.clone(), 3); // task 3, priority 3

        // task 1 = {r1, r2, r3}
        ttr_add(&mut ttr, &t1, &r1);
        ttr_add(&mut ttr, &t1, &r2);
        ttr_add(&mut ttr, &t1, &r3);

        // task 2 = {r1, r2}
        ttr_add(&mut ttr, &t2, &r1);
        ttr_add(&mut ttr, &t2, &r2);

        // task 3 = {r2, r3}
        ttr_add(&mut ttr, &t3, &r2);
        ttr_add(&mut ttr, &t3, &r3);

        println!("ttp {:?}", ttp);
        println!("ttr {:?}", ttr);

        let rtp = cielings(&ttr, &ttp);
        println!("rtp {:?}", rtp);
    }
}
