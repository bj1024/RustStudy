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

fn filecheck(filename:&str) {

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
    match f.write_all(b"Hello, world!\n") {
        Ok(_) => {},
        Err(e) => { panic!("Problem write_all: {:?}", e) },
    }

    println!("write_all ok.");

    match f.sync_data() {
        Ok(_) => {},
        Err(e) => { panic!("Problem sync_data: {:?}", e) },
    }

    println!("sync_data ok.");
    
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
    filecheck(&filename);
    
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}