use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub idnum: i32,
    pub description: String,
    pub completed: bool,
}

fn get_task_file_path() -> io::Result<PathBuf> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Cannot determine executable directory",
        )
    })?;
    Ok(exe_dir.join("tasks.json"))
}

fn write_tasks(tasks: &[Task]) -> io::Result<()> {
    let path = get_task_file_path()?;
    let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;
    write!(file, "{}", serde_json::to_string(tasks)?)?;
    Ok(())
}

pub fn reset_database() -> io::Result<()> {
    let path = get_task_file_path()?;
    // Create the file and write an empty list to it
    let mut file = File::create(&path)?;
    write!(file, "[]")?;
    Ok(())
}

pub fn read_tasks() -> io::Result<Vec<Task>> {
    let path = get_task_file_path()?;

    // Check if the file exists
    if !Path::new(&path).exists() {
        // Create the file and write an empty list to it
        let mut file = File::create(&path)?;
        write!(file, "[]")?;
    }

    // Now, open the file for reading
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tasks = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

pub fn add_task(description: &str) -> io::Result<()> {
    let mut tasks = read_tasks()?;
    let mut idnum = 1;

    if tasks.len() > 0 {
        for task in tasks.clone() {
            if task.idnum == idnum {
                idnum = task.idnum + 1;
            }
        }
    }

    tasks.push(Task {
        description: description.to_string(),
        idnum,
        completed: false,
    });
    write_tasks(&tasks)
}

fn mark_task(idnum: &i32, complete: bool, tasks: &mut Vec<Task>) {
    let idx: usize = *idnum.max(&0) as usize;
    if tasks.len() >= idx {
        tasks[idx - 1].completed = complete;
    } else {
        println!("Warning: task {} doesn't exist.", idx);
    }
}

fn check_task(idnum: &i32, tasks: &mut Vec<Task>) -> Result<(), ()> {
    if *idnum <= 0 {
        println!("Warning: task index must be positive.");
        return Err(());
    }

    let idx: usize = *idnum as usize;
    if idx > 0 && idx <= tasks.len() {
        tasks[idx - 1].completed = !tasks[idx - 1].completed;
        Ok(())
    } else {
        Err(())
    }
}

pub fn complete_task(idnums: &Vec<i32>) -> io::Result<()> {
    let mut taskl = read_tasks()?;
    for idnum in idnums {
        mark_task(&idnum, true, &mut taskl);
    }
    write_tasks(&taskl)
}

pub fn check_task_list(idnums: &Vec<i32>) -> Result<Vec<i32>, io::Error> {
    let tasks = read_tasks();
    match tasks {
        Ok(mut taskl) => {
            let mut idgood = Vec::<i32>::new();
            for idnum in idnums {
                match check_task(&idnum, &mut taskl) {
                    Ok(()) => idgood.push(*idnum),
                    Err(()) => println!("Warning: task {} doesn't exist.", *idnum),
                }
            }
            match write_tasks(&taskl) {
                Ok(()) => Ok(idgood),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn delete_tasks(idnums: &Vec<i32>) -> Result<Vec<i32>, io::Error> {
    let mut tasks_fin: Vec<Task> = Vec::new();
    let tasks = read_tasks();
    match tasks {
        Ok(tasks_init) => {
            let mut idx = 0;
            let mut idgood: Vec<i32> = Vec::<i32>::new();
            for mut task in tasks_init {
                let tasknum = task.idnum;
                if !idnums.contains(&tasknum) {
                    idx += 1;
                    task.idnum = idx;
                    tasks_fin.push(task);
                } else {
                    idgood.push(tasknum);
                }
            }
            match write_tasks(&tasks_fin) {
                Ok(()) => Ok(idgood),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
