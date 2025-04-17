use json::JsonValue;
use clap::Parser;
use std::fs;
use prettytable::Table;
use prettytable::row;

/// Simple todo program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // parse the main command
    #[arg(index=1)]
    command: String,

    // parse the main value
    #[arg(index=2, default_value="")]
    mainvalue: String,

    // parse the subcommand
    #[arg(short, long, action)]
    all: bool,

    #[arg(short, long, action)]
    done: bool,

    #[arg(short, long, action)]
    category: Option<String>,
}

// function to create the data file if it doesn't exist
fn ensure_data_file(main_file: &str) -> std::io::Result<()> {
    if !std::path::Path::new(main_file).exists() {
        #[allow(unused_variables)]
        let file = fs::File::create(main_file)?;
        let initial_data: String = "{\"data\": []}".to_owned();
        fs::write(main_file, &initial_data)?;
    }
    Ok(())
}

fn main() {
    // parse arguments
    let args = Args::parse();

    // results table
    let mut result_table = Table::new();
    result_table.add_row(row!["DONE", "TASK ID", "TASK NAME", "CATEGORY"]);
   
    // main data file name
    let main_file: String = "data.json".to_owned();
    let _ = ensure_data_file(&main_file);
    let current_string: String = fs::read_to_string(&main_file)
        .expect("File is opened to store data.");
    let current_data: &mut JsonValue = &mut json::parse(&current_string).unwrap()["data"];
    // let current_data: &mut JsonValue = &mut json::parse(&current_string).unwrap();

    // for list commands
    if args.command == "list" {
        if args.all {
            // todo list --all --category _category_
            if args.category != None {
                let category__: String = args.category.to_owned().unwrap();
                // println!("You are trying to list all tasks in category: {}", category__);
                for index_ in 0..current_data.len() {
                    let curr: JsonValue = current_data[index_].clone();
                    let done_: bool = json::stringify(curr["done"].clone()).parse().expect("Parsing a true-false value");
                    let task_id_: String = json::stringify(curr["task_id"].clone()).replace("\"","");
                    let task_name_: String = json::stringify(curr["task_name"].clone()).replace("\"", "");
                    let category_: String = json::stringify(curr["category"].clone()).replace("\"", "");
                    if category_ != category__ { continue; } else {
                        result_table.add_row(row![done_, task_id_, task_name_, category_]);
                    }
                }
                result_table.printstd();
            } 
            // todo list --all
            else {    
                // println!("You are trying to print all tasks!");
                for index_ in 0..current_data.len() {
                    let curr: JsonValue = current_data[index_].clone();
                    let done_: bool = json::stringify(curr["done"].clone()).parse().expect("Parsing a true-false value");
                    let task_id_: String = json::stringify(curr["task_id"].clone());
                    let task_name_: String = json::stringify(curr["task_name"].clone());
                    let category_: String = json::stringify(curr["category"].clone());
                    result_table.add_row(row![done_, task_id_, task_name_, category_]);
               }
                result_table.printstd();
            }
        }
        else if args.done {
            // todo list --done --category _category_
            if args.category != None {
                let category__: String = args.category.to_owned().unwrap();
                // println!("You have a category {}", category__);
                for index_ in 0..current_data.len() {
                    let curr: JsonValue = current_data[index_].clone();
                    let done_: bool = json::stringify(curr["done"].clone()).parse().expect("Parsing a true-false value");
                    let task_id_: String = json::stringify(curr["task_id"].clone()).replace("\"","");
                    let task_name_: String = json::stringify(curr["task_name"].clone()).replace("\"", "");
                    let category_: String = json::stringify(curr["category"].clone()).replace("\"", "");
                    if !done_ { continue; } else {
                        if category_ != category__ { continue; } else {
                            result_table.add_row(row![done_, task_id_, task_name_, category_]);
                        }
                    }
                }
                result_table.printstd();
            }
            // todo list --done
            else {
                // println!("You are trying to print completed tasks!");
                for index_ in 0..current_data.len() {
                    let curr: JsonValue = current_data[index_].clone();
                    let done_: bool = json::stringify(curr["done"].clone()).parse().expect("Parsing a true-false value");
                    let task_id_: String = json::stringify(curr["task_id"].clone()).replace("\"","");
                    let task_name_: String = json::stringify(curr["task_name"].clone()).replace("\"", "");
                    let category_: String = json::stringify(curr["category"].clone()).replace("\"", "");
                    if !done_ { continue; } else {
                        result_table.add_row(row![done_, task_id_, task_name_, category_]);
                    }
                }
                result_table.printstd();
            }
        }
        else {
            // todo list --category _category_
            if args.category != None {
                let category__: String = args.category.to_owned().unwrap();
                // println!("You a printing default taks with category {}", category__);
                for index_ in 0..current_data.len() {
                    let curr: JsonValue = current_data[index_].clone();
                    let done_: bool = json::stringify(curr["done"].clone()).parse().expect("Parsing a true-false value");
                    let task_id_: String = json::stringify(curr["task_id"].clone()).replace("\"","");
                    let task_name_: String = json::stringify(curr["task_name"].clone()).replace("\"", "");
                    let category_: String = json::stringify(curr["category"].clone()).replace("\"", "");
                    if done_ { continue; } else {
                        if category_ != category__ { continue; } else {
                            result_table.add_row(row![done_, task_id_, task_name_, category_]);
                        }
                    }
                }
                result_table.printstd();
            }
            // todo list
            else {
                // printing with prettytable
                for index_ in 0..current_data.len() {
                    let curr: JsonValue = current_data[index_].clone();
                    let done_: bool = json::stringify(curr["done"].clone()).parse().expect("Parsing a true-false value");
                    let task_id_: String = json::stringify(curr["task_id"].clone());
                    let task_name_: String = json::stringify(curr["task_name"].clone());
                    let category_: String = json::stringify(curr["category"].clone());
                    if done_ { continue; } else {
                        result_table.add_row(row![done_, task_id_, task_name_, category_]);
                    }
               }
                result_table.printstd();
            }
        }
        
    }
    // for add command
    else if args.command == "add" {
        // todo add --category _category_ _task_
        // variable to hold category string
        let mut category_: String = "default".to_owned();
        if args.category != None {
            // if category is provided, change default to provided category
            category_ = args.category.to_owned().unwrap();
        }
        // todo add _task_
        let mut next_task_id: u32 = 1;
        for index_ in 0..current_data.len() {
            let task_id_string: String = json::stringify(current_data[index_]["task_id"].clone());
            let current_task_id: u32 = task_id_string.parse().unwrap();
            if current_task_id >= next_task_id {
                next_task_id += 1;
            }
        }
        let taskname: String = args.mainvalue;
        let new_data_string: String = format!("{{\"task_id\": {}, \"task_name\": \"{}\", \"category\": \"{}\", \"done\": {}}}", next_task_id, taskname, category_, false);
        let new_data: JsonValue = json::parse(&new_data_string).unwrap();
        let _ = current_data.push(new_data);
        let current_data_string: String = json::stringify(current_data.clone());
        let final_data_string: String = format!("{{\"data\": {} }}", current_data_string);
        let _ = fs::write(main_file, final_data_string);
        println!("You added the task!");
        }
    // for did command
    else if args.command == "did" {
        // todo did _task_id_
        let task_id: u32= args.mainvalue.parse::<u32>().unwrap();
        for index_ in 0..current_data.len() {
            let current_task_id_string: String = json::stringify(current_data[index_]["task_id"].clone());
            let current_task_id: u32 = current_task_id_string.parse().unwrap();
            if current_task_id == task_id {
                current_data[index_]["done"] = json::JsonValue::Boolean(true);
            }
        }
        let current_data_string: String = json::stringify(current_data.clone());
        let final_data_string: String = format!("{{\"data\": {} }}", current_data_string);
        let _ = fs::write(main_file, final_data_string);
        println!("Marked task: {} as finished.", task_id);
    }
    else if args.command == "undid" {
        // todo undid _task_id_
        let task_id: u32= args.mainvalue.parse::<u32>().unwrap();
        for index_ in 0..current_data.len() {
            let current_task_id_string: String = json::stringify(current_data[index_]["task_id"].clone());
            let current_task_id: u32 = current_task_id_string.parse().unwrap();
            if current_task_id == task_id {
                current_data[index_]["done"] = json::JsonValue::Boolean(false);
            }
        }
        let current_data_string: String = json::stringify(current_data.clone());
        let final_data_string: String = format!("{{\"data\": {} }}", current_data_string);
        let _ = fs::write(main_file, final_data_string);
        println!("Marked task: {} as unfinished.", task_id);
    }
    else if args.command == "update" {
        // todo update --category _category_ _task_id_
        let mut new_category: String = "default".to_owned();
        let mut category_: String = "".to_owned();
        let task_id: u32= args.mainvalue.parse::<u32>().unwrap();
        if args.category != None {
            new_category = args.category.to_owned().unwrap();
        }
        // todo update _task_id_
        for index_ in 0..current_data.len() {
            let current_task_id_string: String = json::stringify(current_data[index_]["task_id"].clone());
            let current_task_id: u32 = current_task_id_string.parse().unwrap();
            if current_task_id == task_id {
                category_ = json::stringify(current_data[index_]["category"].clone()).replace("\"", "");
                current_data[index_]["category"] = json::JsonValue::String(new_category.clone());
            }
        } 
        let current_data_string: String = json::stringify(current_data.clone());
        let final_data_string: String = format!("{{\"data\": {} }}", current_data_string);
        let _ = fs::write(main_file, final_data_string);
        println!("Updated the category of task: {} from {} to {}", task_id, category_, new_category);
    }
    else if args.command == "delete" {
        // todo delete _task_id_
        let task_id: u32= args.mainvalue.parse::<u32>().unwrap();
        let mut remove_index_: usize = 9999;
        for index_ in 0..current_data.len() {
            let current_task_id_string: String = json::stringify(current_data[index_]["task_id"].clone());
            let current_task_id: u32 = current_task_id_string.parse().unwrap();
            if current_task_id == task_id {
                remove_index_ = index_;
            }
        }
        let _ = current_data.array_remove(remove_index_);
        let current_data_string: String = json::stringify(current_data.clone());
        let final_data_string: String = format!("{{\"data\": {} }}", current_data_string);
        let _ = fs::write(main_file, final_data_string);
        println!("You have deleted task: {}", task_id);
    }
    else {
        // When commands aren't from known list
        println!("Unknown command!\nUse --help for more information!");
    }
}
