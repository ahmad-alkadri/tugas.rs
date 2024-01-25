use crate::tasks;
use fancy::printcoln;

pub fn printtasks(tasklist: &Vec<tasks::Task>) {
    println!("List of tasks:");
    println!("---");
    for tas in tasklist {
        if tas.completed {
            printcoln!("[strikethrough]{}. {}", tas.idnum, tas.description);
        } else {
            println!("{}. {}", tas.idnum, tas.description);
        }
    };
}