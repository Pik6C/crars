extern crate colored;
use colored::*;


/// バージョンを表示します
pub fn version()
{
    println!("{}","vira".color("blue").bold());
    println!("{}{}","version:".bright_yellow(),"0.1.0");
}

/// ヘルプを表示します
pub fn help(ぬ:String){
    println!("\n{} {}","terminal text editor".bright_cyan(),"vira".color("blue").bold());
    println!("{}{} [arguments] [file]\n","Usage:".yellow(),ぬ);
    println!("  or {} [file]",ぬ);
    println!("      (Loads the specified file and launches a text editor to edit it.)\n");
    println!("  or {}",ぬ);
    println!("      (Starts a text editor without specifying a file)\n");
    println!("");
    println!("Arguments:\n");
    println!("-r             Outputs the contents of the specified file to standard output");
    println!("");
    println!("\n");
    
}