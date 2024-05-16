pub mod insert;
pub mod buffscrean;
pub mod fileio;
pub mod memory;
pub mod about;
use std::fs;
use std::io::prelude::*;
extern crate termion;
use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::env::*;

fn main()
{

    #[allow(unused_mut)]
    #[allow(unused_variables)]
    let mut insert = false;
    let _args: Vec<String> = args().collect();
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    let mut buff:Vec<String> = vec![];
    
    if (_args.len() >= 3) && (_args[1] == "--read" || _args[1] == "-r") { // 読み込み・表示

         match  fs::File::open(_args[2].to_string()){
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
         
     }else if _args.len() == 2{ // lenが2以上のときの処理たち

        if _args[1] == "-v" || _args[1] == "--version"{ // バージョン表示
            about::version();
            return;
        }else if _args[1] == "-h" || _args[1] == "--help"{ // ヘルプ
            about::help(_args[0].to_string());
            return;
        }

        fileio::fileio(_args[1].to_string()); // ファイル編集モード
        
    }
    
    else{
        //ここに普通のvimの処理を入れていく
        let _ = buffscrean::newbuff(); //新しいバッファを作る

        let mut _buffer: Vec<Vec<char>> = Vec::new();
        

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



       
     }
    
    }
