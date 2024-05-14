pub mod test;

use std::fs::File;
use std::io;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;
use std::sync::mpsc;
use std::thread;

fn main()
{
    let _args: Vec<String> = std::env::args().collect();
    let mut buff:Vec<String> = vec![];

    if _args.len() < 2{ // IndexOutOfBounse起きないように最初からarsgが2以上か調べる

        println!("Usage: {} <command> <file>",_args[0]);
        return;
        
    }else if (_args.len() >= 3 && (_args[1] == "-r" || _args[1] == "--read")) || 
    (_args.len() >= 3 && _args[1] == "-r"){ // 読み込み・表示
 
         let filepath = _args[2].to_string();
 
         match  fs::File::open(filepath){
             Ok(mut file) =>{
                 // 文字列のベクターを準備
                 let mut contents = String::new();
 
                 // ファイルの内容を文字列に読み込む
                 match file.read_to_string(&mut contents){
 
                     Ok(contents) => println!("{}", contents),
 
                     Err(_) => eprintln!("\x1b[31m\x1b[1mFailed to load file\x1b[0m\x1b[0m"),
 
                     
                 }
             },
             Err(_) => {
                 eprintln!("\x1b[31m\x1b[1mFetal Error\x1b[0m\x1b[0m\nfile not found\nPlease check if the file exists and try again");
 
 
             },
              
         }; 
         
     }else{
        if _args.len() >= 2{ //ここにvimの処理を入れていく

            





        }else if _args.len() >= 3{ //ファイルを編集するときのvim処理

            let filepath = _args[2].to_string();

            match fs::File::open(filepath) { //ファイル読み込み。matchで失敗してもpanicを起こさないようにする。ファイルパッチは_args[2]。
                Ok(mut str) => { //成功処理。strがファイル型になる
                     
                    let mut filevec = String::new(); //ファイルの中身をいれるString

                    match str.read_to_string(&mut filevec){ //matchで成功と失敗を判別してpanicが起らないようにする

                        Ok(filevec) => { //成功処理。filevecにファイルの内容をいれる。

                            println!("{}",filevec); //ファイルの中身をprintlnで出力する。(一時的)
                        },
                        Err(_) => {

                        },
                    }
                },
                Err(_) => {

                },
            }



        }
     }
    
    }
