mod clients;
mod tasks;
mod tools;
mod subcommands;

use clap::App;

fn main() {

    let matches = App::new("Tugas")
        .version("0.1.0")
        .author("Ahmad Alkadri")
        .about("Simple to-do list CLI app")
        .subcommand(subcommands::comm_add())
        .subcommand(subcommands::comm_list())
        .subcommand(subcommands::comm_done())
        .subcommand(subcommands::comm_check())
        .subcommand(subcommands::comm_del())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(task) = matches.value_of("TASK") {
            tasks::add_task(task).expect("Failed to add task");
            println!("Task added: {}", task);
        }
    }

    if matches.subcommand_matches("list").is_some() {
        let task_list = tasks::read_tasks();
        match task_list {
            Ok(taskl) => clients::printtasks(&taskl),
            Err(_) => println!("No todolist database found; generating one."),
        }
    }

    // Handling the complete subcommand
    if let Some(matches) = matches.subcommand_matches("complete") {
        if let Some(tasks) = matches.values_of("TASK-IDS") {
            let task_ids: Vec<i32> = tasks.filter_map(|s| s.parse::<i32>().ok()).collect();

            match tasks::complete_task(&task_ids) {
                Ok(_) => println!("Marking tasks as completed"),
                Err(_) => println!(
                    "Error trying to mark tasks as completed. Verify if the database is corrupted."
                ),
            }
        }
    }

    // Handling the check subcommand
    if let Some(matches) = matches.subcommand_matches("check") {
        if let Some(tasks) = matches.values_of("TASKS") {
            let task_ids: Vec<i32> = tasks.filter_map(|s| s.parse::<i32>().ok()).collect();
            match tasks::check_task_list(&task_ids) {
                Ok(idsuccess) => {
                    if idsuccess.len() > 0 {
                        println!("Checking tasks {}", tools::formatvec_i32(&idsuccess));
                    } else {
                        println!("No task checked.");
                    }
                }
                Err(_) => println!(
                    "IO Error when trying to check tasks. Verify if the database is corrupted."
                ),
            }
        }
    }

    // Handling the delete subcommand
    if let Some(matches) = matches.subcommand_matches("delete") {
        if let Some(tasks) = matches.values_of("TASKS") {
            let input = tasks.collect::<Vec<_>>().join(",");
            if input == "all" {
                match tasks::reset_database() {
                    Ok(_) => println!("All tasks deleted"),
                    Err(e) => {
                        println!("IO Error when trying to delete all tasks. Details:");
                        println!("{}", e);
                    }
                }
            } else {
                let task_ids: Vec<i32> = input
                    .split(',')
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();
                match tasks::delete_tasks(&task_ids) {
                    Ok(idsuccess) => {
                        if idsuccess.len() > 0 {
                            println!("Deleting tasks {}", tools::formatvec_i32(&idsuccess));
                        } else {
                            println!("No task deleted.");
                        }
                    }
                    Err(_) => println!(
                        "IO Error when trying to check tasks. Verify if the database is corrupted."
                    ),
                }
            }
        }
    }
}
