use std::error::{Error};
use std::fs::File;
use std::{io, env};
use std::io::{ErrorKind, Write};
use std::process;

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn filecheck(filename:&str) -> Result<&str, io::Error>{
        let f = File::options().append(true).open(filename);

    let mut f: File  = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    println!("file opened.{:?}",filename);


    // f.write_all(b"Hello, world!");

    // fn write_all(&mut self, buf: &[u8]) -> Result<()>
    // match f.write_all(b"Hello, world!\n") {
    //     Ok(_) => {},
    //     Err(e) => { panic!("Problem write_all: {:?}", e) },
    // }
    // f.write_all(b"Hello, world!\n").expect("Problem sync_data\n");
    f.write_all(b"Hello, world!\n")?;
    println!("write_all ok.");

    // f.sync_data().expect("Problem sync_data:");
    f.sync_data()?; // '?'は、Resultのエラーを伝播する場合に利用する。現在の関数戻り値がResultの場合にOK。
    // match f.sync_data() {
    //     Ok(_) => {},
    //     Err(e) => { panic!("Problem sync_data: {:?}", e) },
    // }

    println!("sync_data ok.");
    return Ok("sync_data ok.");
    
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 1 {
        println!("usage:csvexam filename");
        process::exit(1);
    }
    let filename = &args[1];

    // let fname = "test.txt";
    match filecheck(&filename) {

        Ok(mes)=>{println!("mes:[{}]",mes)}
        Err(e) => { panic!("Problem filecheck: {:?}", e) },
    }
    
    
    // if let Err(err) = example() {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }
}