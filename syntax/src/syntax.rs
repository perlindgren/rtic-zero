// syntax

use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Task {
    pub id: String,
    pub priority: u8,
    pub binds: Option<String>,
    pub shared: Vec<Resource>,
    pub local: Vec<ResourceInit>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Init {
    pub local: Vec<ResourceInit>,
    pub late: (), // To be determined
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Resource {
    pub id: String,
    pub ty: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ResourceInit {
    pub id: String,
    pub ty: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct TaskSet {
    pub device: String,
    pub shared: Vec<Resource>,
    pub local: Vec<Resource>,
    pub init: Init,
    pub tasks: Vec<Task>,
}

#[cfg(test)]
pub fn task_set() -> TaskSet {
    let r1 = Resource {
        id: "r1".into(),
        ty: "Special<u32>".into(),
    };
    let r2 = Resource {
        id: "r2".into(),
        ty: "u8".into(),
    };
    let r3 = Resource {
        id: "r3".into(),
        ty: "u8".into(),
    };

    let ri1 = ResourceInit {
        id: "ri1".into(),
        ty: "u32".into(),
        value: "32".into(),
    };

    let ri2 = ResourceInit {
        id: "ri2".into(),
        ty: "u64".into(),
        value: "64".into(),
    };

    TaskSet {
        device: "some_dev".into(),
        shared: vec![],
        local: vec![],
        init: Init {
            local: vec![],
            late: (),
        },
        tasks: vec![
            Task {
                id: "t1".into(),
                priority: 1,
                binds: Some("EXTI0".into()),
                shared: vec![r1.clone(), r2.clone(), r3.clone()],
                local: vec![ri1.clone(), ri2.clone()],
            },
            Task {
                id: "t2".into(),
                priority: 2,
                binds: Some("EXTI1".into()),
                shared: vec![r1.clone(), r2.clone()],
                local: vec![],
            },
            Task {
                priority: 3,
                id: "t3".into(),
                binds: None,
                shared: vec![r2.clone(), r3.clone()],
                local: vec![],
            },
        ],
    }
}
