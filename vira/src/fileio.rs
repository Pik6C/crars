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

            },
        }
    },
    Err(_) => {

    },
}
}