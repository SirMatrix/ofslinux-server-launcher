use curl::easy::Easy;
use std::io::{stdout, Write};


pub fn reader(URL: &str){
let mut easy = Easy::new();
    easy.url(&URL).unwrap();
    easy.write_function(|data|{
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    println!("{}", easy.response_code().unwrap());

}

