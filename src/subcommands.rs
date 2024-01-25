use clap::{Arg, Command};

pub fn comm_add() -> Command<'static> {
    Command::new("add")
        .about("Add a task to the to-do list")
        .arg(
            Arg::with_name("TASK")
                .long_help(
                    "Task description. Enclose in quotation mark (\"\")
                        \nEXAMPLE:
                        \n$ tugas add \"Pay the supplier\"
                        \n$ tugas add \"Send the letter to Emily\"",
                )
                .required(true)
                .index(1),
        )
}

pub fn comm_list() -> Command<'static> {
    Command::new("list").about("Display the list of tasks")
}

pub fn comm_done() -> Command<'static> {
    Command::new("complete")
        .about("Mark tasks as complete from the to-do list")
        .arg(
            Arg::with_name("TASK-IDS")
                .help("List of tasks' numbers")
                .required(true)
                .multiple(true) // Allows multiple values
                .use_delimiter(true) // Use a delimiter for separating values
                .value_delimiter(','), // Specify the delimiter
        )
}

pub fn comm_check() -> Command<'static> {
    Command::new("check")
        .about("Check tasks from the to-do list")
        .arg(
            Arg::with_name("TASKS")
                .help("List of tasks' numbers")
                .required(true)
                .multiple(true) // Allows multiple values
                .use_delimiter(true) // Use a delimiter for separating values
                .value_delimiter(','), // Specify the delimiter
        )
}

pub fn comm_del() -> Command<'static> {
    Command::new("delete")
        .about("Delete tasks from the to-do list")
        .arg(
            Arg::with_name("TASKS")
                .help("List of task IDs to delete, or 'all' to delete all tasks")
                .required(true)
                .validator(|v| {
                    if v == "all" || v.split(',').all(|s| s.parse::<i32>().is_ok()) {
                        Ok(())
                    } else {
                        Err(String::from("Invalid input"))
                    }
                }),
        )
}
