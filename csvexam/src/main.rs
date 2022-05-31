use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{ErrorKind, Write};
use std::process;
use std::{env, fmt, io};

use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc, NaiveDateTime};

mod util;

struct User {
    no: i32,
    name: String,
    kana: String,
    gender: String,
    phone: String,
    birth: DateTime<Local>,
}

impl User {
    fn new(
        no: i32,
        name: String,
        kana: String,
        gender: String,
        phone: String,
        birth: DateTime<Local>,
    ) -> Self {
        Self {
            no,
            name,
            kana,
            gender,
            phone,
            birth,
        }
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "no:{} name:{} kana:{} gender:{} phone:{} birth:{}",
            &self.no, 
            &self.name, 
            &self.kana, 
            &self.gender,
             &self.phone, 
             &self.birth.format("%Y-%m-%d %H:%M:%S.%3f").to_string()
        )?;

        Ok(())
    }
}

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

// void的な戻り値の関数
// '()' は Empty tuple ( ()は、unitと呼ばれる)
// The tuple without any values, (), is a special type that has only one value, also written ().
// The type is called the unit type and the value is called the unit value.
// Expressions implicitly return the unit value if they don’t return any other value.
// https://doc.rust-lang.org/book/ch03-02-data-types.html
fn funcret01() -> Result<(), io::Error> {
    Ok(())
}

fn filecheck(filename: &str) -> Result<&str, io::Error> {
    let f = File::options().append(true).open(filename);

    let mut f: File = match f {
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
    println!("file opened.{:?}", filename);

    // f.write_all(b"Hello, world!");

    // fn write_all(&mut self, buf: &[u8]) -> Result<()>

    // 'match' 構文 Result内容により分岐する。
    // match f.write_all(b"Hello, world!\n") {
    //     Ok(_) => {}, ←　成功のときは何もしていない。
    //     Err(e) => { panic!("Problem write_all: {:?}", e) },
    // }

    // 'expect' 構文. Shortcuts for Panic on Error: unwrap and expect
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#shortcuts-for-panic-on-error-unwrap-and-expect
    // match文のショートカット版
    // f.write_all(b"Hello, world!\n").expect("Problem sync_data\n");

    // '?'構文
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

fn read_file(filename: &str) -> Result<(), io::Error> {
    // let f = File::options().read(true).write(false).open(filename);
    let f = File::open(filename).expect(format!("file open error.[{}]", filename).as_str());

    let mut reader = BufReader::new(f);

    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let line_trimed = line.trim_end();
        println!("line = [{}]", line_trimed);
        line.clear(); // read_line はappendするので１行ずつの場合はクリアする。
    }

    Ok(())
}

fn read_csv(filename: &str) -> Result<(), io::Error> {
    // let f = File::options().read(true).write(false).open(filename);
    let f = File::open(filename).expect(format!("file open error.[{}]", filename).as_str());

    let mut reader = BufReader::new(f);
    let mut csvrdr = csv::Reader::from_reader(reader);

    let mut row_number = 0;

    let mut users: Vec<User> = Vec::new();

    for result in csvrdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("[{}] {:?}", row_number + 1, record);

        let no = users.push(User {
            no: record[0].parse().expect("no parse error."),
            name: record[1].to_string(),
            kana: record[2].to_string(),
            gender: record[3].to_string(),
            phone: record[4].to_string(),
            birth: Local::now(),   // TODO:
        });
        row_number += 1;
    }

    println!("users={:?}", users);

    Ok(())
}

fn research_datetime() {
    // 現在日時
    let local: DateTime<Local> = Local::now();

    println!("now={:?}", local); // now=2022-05-31T10:09:38.930586+09:00

    // フォーマット
    // chrono::format::strftime - Rust https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers
    println!(
        "now=[{}]",
        local.format("%Y-%m-%d %H:%M:%S.%3f").to_string()
    ); // now=[2022-05-31 10:18:17.871]

    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9 * 3600));

    // NG String -> datetime 
    // DateTime::parse_from_str ヘルプによると、文字中にtimezoneが必要。
    // Note that this method *requires a timezone* in the string
    // let parsed_dt = DateTime::parse_from_str("2022-05-31 10:21:34", "%Y-%m-%d %H:%M:%S")
    //     .expect("parse date error.");

    // timezoneがない場合は、Naive〜を利用する。
    let parsed_dt_nontimezone = NaiveDateTime::parse_from_str("2022-05-31 10:21:34", "%Y-%m-%d %H:%M:%S")
    .expect("parse date error.");
    println!("parsed_dt_nontimezone=[{:?}]",parsed_dt_nontimezone); // parsed_dt_nontimezone=[2022-05-31T10:21:34] 

    // Naive -> DateTime（TimeZone付き）　に変換する
    let parsed_dt = Local.from_local_datetime(&parsed_dt_nontimezone).unwrap();
    println!("parsed_dt=[{:?}]",parsed_dt); // parsed_dt=[2022-05-31T10:21:34+09:00]
    
    // UTCにする場合はこちら
    let parsed_utcdt = Utc.from_local_datetime(&parsed_dt_nontimezone).unwrap();
        println!("parsed_utcdt=[{:?}]",parsed_utcdt); // parsed_utcdt=[2022-05-31T10:21:34Z]

    // TimeZone時間指定の場合ははこちら
    let parsed_offsetdt = FixedOffset::east(9*3600).from_local_datetime(&parsed_dt_nontimezone).unwrap();
    println!("parsed_offsetdt=[{:?}]",parsed_offsetdt); // parsed_offsetdt=[2022-05-31T10:21:34+09:00]


    // util.rsに関数化
    let localdt = util::toYMD_HMS_ToLocalTime("2022-05-32 10:21:34").unwrap();
    println!("toYMD_HMS_ToLocalTime=[{:?}]",localdt); 


}


fn main() {
    // DateTimeの扱いの検証
    research_datetime();

    // 引数の扱いの検証
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 1 {
        println!("usage:csvexam filename");
        process::exit(1);
    }
    let filename = &args[1];

    // let fname = "test.txt";
    // match filecheck(&filename) {
    //     Ok(mes)=>{println!("mes:[{}]",mes)}
    //     Err(e) => { panic!("Problem filecheck: {:?}", e) },
    // }

    // void的な関数
    let _ = funcret01();

    // file read
    let _ = read_file(&filename);

    // csv read
    let _ = read_csv(&filename);

    // if let Err(err) = example() {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }
}
