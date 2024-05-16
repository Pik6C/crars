extern crate colored;
use colored::*;

pub fn version()
{
    println!("{}","vira".blue());
    println!("{}{}","version:".yellow(),"0.1.0");
}

pub fn help(ぬ:String){
    println!("\nwelcome to {}!\n","vira".blue().bold());
    println!("{}{} <option> <file>\n","Usage:".yellow(),ぬ);
    println!("{}","help:".yellow());
    println!("{}",ぬ);
    println!("  -Starts a text editor without specifying a file");
    println!("");
    println!("");
    println!("{} <file>",ぬ);
    println!("  -Loads the specified file and launches a text editor to edit it.");
    println!("\n");
    println!("{} -r <file>",ぬ);
    println!("  -Outputs the contents of the specified file");
    println!("\n");
    
}