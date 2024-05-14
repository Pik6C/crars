#[allow(unused_imports)]
pub mod insert;
pub mod buffscrean;
pub mod fileio;
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


fn main()
{
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    let mut insert = false;
    let _args: Vec<String> = std::env::args().collect();
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    let mut buff:Vec<String> = vec![];

    if (_args.len() >= 3 && (_args[1] == "-r" || _args[1] == "--read")) || 
    (_args.len() >= 3 && _args[1] == "-r"){ // 読み込み・表示
 
         let filepath = _args[2].to_string();
 
         match  fs::File::open(filepath){
             Ok(mut file) =>{
                 // 文字列のベクターを準備
                 let mut contents = String::new();
 
                 // ファイルの内容を文字列に読み込む
                 match file.read_to_string(&mut contents){
 
                     Ok(_) => println!("{}",contents),
                     Err(_) => eprintln!("\x1b[31m\x1b[1mFailed to load file\x1b[0m\x1b[0m"),
 
                     
                 }
             },
             Err(_) => {
                 eprintln!("\x1b[31m\x1b[1mFetal Error\x1b[0m\x1b[0m\nfile not found\nPlease check if the file exists and try again");
 
 
             },
              
         }; 
         
     }else{
        //ここにvimの処理を入れていく
        buffscrean::newbuff();

        let stdin = stdin();

/*
        // Rawモードにする
        // unwrapだからエラー時にはpanicを起こす（いずれ治す）
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut stdout = stdout().into_raw_mode().unwrap();
*/
        buffscrean::rawmode();

        for event in stdin.events(){

            match event.unwrap(){
                
                // とりあえずctrl+cでやめれるようにする
                Event::Key(Key::Esc) => {
                    
                    buffscrean::closebuff();
                    
                }
                Event::Key(Key::Ctrl('c')) => {
                    buffscrean::closebuff();
                    return;
                }


                _ => {}
                }

            }



       if _args.len() >= 3{ //ファイルを編集するときのvim処理

            



        }
     }
    
    }
