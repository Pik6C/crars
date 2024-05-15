#[allow(unused_imports)]
pub mod insert;
pub mod buffscrean;
pub mod fileio;
pub mod memory;
#[allow(unused_imports)]
use std::{fs, io};
use std::io::prelude::*;
extern crate termion;
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
#[allow(unused_imports)]
use termion::event::{self, Event, Key};
use termion::input::TermRead;
#[allow(unused_imports)]
use termion::raw::IntoRawMode;
#[allow(unused_imports)]
use crossterm::cursor::{Hide, MoveTo, Show};
#[allow(unused_imports)]
use crossterm::terminal::{disable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
#[allow(unused_imports)]
use crossterm::{execute, queue, style::PrintStyledContent};
use clap::*;

/// terminal text editor vira
#[derive(Parser)]
struct Args{
    /// File name to edit
    filename: String,
    /// Display the contents of the file
    #[arg(short='r', long)]
    read: String
}

fn main()
{
    let args = Args::parse();
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    let mut insert = false;
    let _args: Vec<String> = std::env::args().collect();
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    let mut buff:Vec<String> = vec![];
    
    if args.read == args.read{ // 読み込み・表示

         match  fs::File::open(args.read){
             Ok(mut file) =>{
                 // 文字列のベクターを準備
                 let mut contents = String::new();
 
                 // ファイルの内容を文字列に読み込む
                 match file.read_to_string(&mut contents){
 
                     Ok(_) => println!("{}",contents),
                     Err(_) => eprintln!("\x1b[31m\x1b[1mFailed to load file\x1b[0m\x1b[0m\n\x1b[31mFile loading failed for unknown reason.\nPlease report this issue on the github repository\nURL:https://github.com/Pik6C/vira\x1b[0m"),
 
                     
                 }
             },
             Err(_) => {
                 eprintln!("\x1b[31m\x1b[1mFetal Error\x1b[0m\x1b[0m\nfile not found\nPlease check if the file exists and try again");
 
 
             },
              
         }; 
         
     }else{
        //ここにvimの処理を入れていく
        let _ = buffscrean::newbuff(); //新しいバッファを作る

        let mut buffer: Vec<Vec<char>> = Vec::new();
        

        let stdin = stdin();

        // Rawモードにする
        // unwrapだからエラー時にはpanicを起こす（いずれ治す）
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut stdout = stdout().into_raw_mode().unwrap();

        for event in stdin.events(){

            match event.unwrap(){
                
                
                Event::Key(Key::Esc) => {
                    
                    let _ = buffscrean::closebuff();
                    
                }
                // とりあえずctrl+cでやめれるようにする
                Event::Key(Key::Ctrl('c')) => {
                    let _ = buffscrean::closebuff();
                    return;
                }


                _ => {}
                }

            }



       if _args.len() >= 3{ //ファイルを編集するときのvim処理

            fileio::fileio(_args[2].to_string());
            
        }
     }
    
    }
