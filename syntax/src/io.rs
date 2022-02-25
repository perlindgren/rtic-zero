// syntax

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::syntax::*;

pub fn save_task_set(task_set: &TaskSet, path: &Path) {
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the string to `file`, returns `io::Result<()>`
    let serialized = serde_json::to_string(&task_set).unwrap();
    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

pub fn load_tasks(path: &Path) -> TaskSet {
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => serde_json::from_str(&s).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let task_set = task_set();

        println!("task_set {:?}", task_set);
        let serialized = serde_json::to_string(&task_set).unwrap();
        println!("task_set {:?}", serialized);
        let deserialized: TaskSet = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
        assert_eq!(task_set, deserialized);
    }

    #[test]
    fn save_load() {
        let task_set = task_set();
        let path = Path::new("rtic.json");
        save_task_set(&task_set, &path);
        let task_set_loaded = load_tasks(&path);
        assert_eq!(task_set, task_set_loaded);
    }

    #[test]
    fn load() {
        let task_set = task_set();
        let path = Path::new("rtic.json");

        let task_set_loaded = load_tasks(&path);
        assert_eq!(task_set, task_set_loaded);
    }
}
