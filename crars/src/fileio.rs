use std::fs;
use std::io::prelude::*;

pub fn fileio(filepath:String){

match fs::File::open(filepath) { //ファイル読み込み。matchで失敗してもpanicを起こさないようにする。ファイルパッチは_args[2]。
    Ok(mut str) => { //成功処理。strがファイル型になる
        let mut filevec = String::new(); //ファイルの中身をいれるString

        match str.read_to_string(&mut filevec){ //matchで成功と失敗を判別してpanicが起らないようにする

            Ok(_) => { //成功処理。filevecにファイルの内容をいれる。

                println!("{}",filevec); //ファイルの中身をprintlnで出力する。(一時的)
            },
            Err(_) => {
                eprintln!("\x1b[31mFetal Error \x1b[1mFailed to load file\x1b[0m\x1b[0m\n\x1b[31mFile loading failed for unknown reason.\nPlease report this issue on the github repository\nURL:https://github.com/Pik6C/vira\x1b[0m")
            },
        }
    },
    Err(_) => {
        eprintln!("\x1b[31m\x1b[1mError\x1b[0m\x1b[0m\nfile not found(by:fileio.rs)\nPlease check if the file exists and try again");
    },
}
}

/*
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
         
         
 */