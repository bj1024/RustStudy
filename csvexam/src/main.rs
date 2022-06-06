#[macro_use]
extern crate log;
extern crate simplelog;

use std::path::Path;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::io::{ErrorKind, Write};
use std::process::{self};
use std::str::FromStr;
use std::{env, fmt, io};

use chrono::{Date, DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};

// use env_logger::Env;
use lazy_static::lazy_static;
use log::{Level, LevelFilter};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

use simplelog::{
    format_description, Color, ColorChoice, CombinedLogger, Config, ConfigBuilder, TermLogger,
    TerminalMode, WriteLogger,
};

use crate::util::ymd_to_localdate;

mod util;

macro_rules! log_divider {
    ($prefix:literal) => {
        info!("");
        info!("---------- {} ----------", $prefix);
    };
}

#[derive(Serialize, Deserialize)]
struct MyPrimitive {
    no: i32,
    name: String,
}

// mod my_date_format {
//     use chrono::{Date, Local};
//     use serde::{self, Deserialize, Deserializer, Serializer};

//     pub fn serialize<S>(value: &Date<Local>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Date<Local>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//     }
// }

// mod my_optdate_format {
//     use chrono::{Date, Local, TimeZone, Utc};
//     use serde::{Deserialize, Deserializer, Serialize, Serializer};

//     const FORMAT: &'static str = "%Y-%m-%d";

//     // The signature of a serialize_with function must follow the pattern:
//     //
//     //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//     //    where
//     //        S: Serializer
//     //
//     // although it may also be generic over the input types T.
//     pub fn serialize<S>(value: &Option<Date<Local>>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         #[derive(Serialize)]
//         struct Helper<'a>(#[serde(with = "my_date_format")] &'a Date<Local>);

//         value.as_ref().map(Helper).serialize(serializer)
//     }

//     // The signature of a deserialize_with function must follow the pattern:
//     //
//     //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//     //    where
//     //        D: Deserializer<'de>
//     //
//     // although it may also be generic over the output types T.
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date<Local>>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         struct Helper(#[serde(with = "my_date_format")] Date<Local>);

//         let helper = Option::deserialize(deserializer)?;
//         Ok(helper.map(|Helper(external)| external))
//     }
// }

mod my_optdate_format {
    use chrono::{Date, Local};
    use serde::{self, Deserialize, Deserializer, Serializer};

    use crate::util::ymd_to_localdate;

    const FORMAT: &'static str = "%Y-%m-%d";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &Option<Date<Local>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(v) => {
                let s = format!("{}", v.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date<Local>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        match Option::<String>::deserialize(deserializer) {
            Ok(v) => {
                debug!("deserialize - Ok [{:?}]", v);
                match v {
                    Some(v) => {
                        match ymd_to_localdate(v.as_str()) {
                            Ok(v) => Ok(Some(v)),
                            Err(e) => {
                                // Err(Error::custom(e.to_string()))
                                warn!("❗️Warning [{}] {}", v, e);
                                Ok(None)
                            }
                        }
                        // Ok(Some().unwrap())),
                    }
                    None => Ok(None),
                }
            }
            Err(e) => {
                error!("deserialize - error [{:?}]", e);
                Err(Error::custom("error"))
            }
        }
        // Ok(Some(ymd_to_localdate("2023-12-31").unwrap()))
        // match String::deserialize(deserializer) {
        //     Ok(s) => match to_localdate_with_format(&s, FORMAT) {
        //         Ok(v) => Ok(Some(v)),
        //         Err(e) => Err(Error::custom("error")),
        //     },
        //     Err(e) => {
        //         println!("error {:?}", e);
        //         return Ok(None);
        //     }
        // }

        // Utc.datetime_from_str(&s, FORMAT)
        //     .map_err(serde::de::Error::custom)
    }
}

// JSONはDateTimeの形式を定義していないが、javascriptの 2012-04-23T18:25:43.511Z を利用した方が良い。
//  javascript - What is the "right" JSON date format? - Stack Overflow https://stackoverflow.com/questions/10286204/what-is-the-right-json-date-format

//
// Option<Date<Local>>は ChronoがSerialize Deserializeを提供していない。実装が必要。
#[derive(Debug, Serialize, Deserialize)]
struct MyDateStruct {
    no: i32,
    #[serde(with = "my_optdate_format")]
    dt: Option<Date<Local>>,
}

// #[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct User {
    no: i32,
    name: String,
    kana: String,
    gender: String,
    phone: String,

    // #[serde(with = "my_optdate_format")]
    birth: Option<Date<Local>>, // Option None,またはTを格納したオプショナル
}

impl User {
    /// Creates a new [`User`].
    #[allow(dead_code)] // suppress "function is never used" warning.
    fn new(
        no: i32,
        name: String,
        kana: String,
        gender: String,
        phone: String,
        birth: Option<Date<Local>>,
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
        // let birthstr = &self.birth.unwrap_or(0) ();
        // .format("%Y-%m-%d").to_string()
        // ;
        // let birthstr =  match &self.birth{
        //     None => String::from_str(""),
        // };

        // let birthstr = &self.birth.unwrap_or("").format("%Y-%m-%d").to_string();
        // let birthstr;
        // match &self.birth{
        //     Some(v) => {  birthstr = v.format("%Y-%m-%d").to_string() } ,
        //     None => {birthstr = "".to_string() },
        // }

        let birthstr = match &self.birth {
            Some(v) => v.format("%Y-%m-%d").to_string(),
            None => "".to_string(),
        };

        writeln!(
            f,
            "no:{} name:{} kana:{} gender:{} phone:{} birth:{}",
            &self.no, &self.name, &self.kana, &self.gender, &self.phone, birthstr
        )?;

        Ok(())
    }
}

#[allow(dead_code)] // suppress "function is never used" warning.
fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        debug!("{:?}", record);
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

#[allow(dead_code)] // suppress "function is never used" warning.
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
    debug!("file opened.{:?}", filename);

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

    debug!("write_all ok.");

    // f.sync_data().expect("Problem sync_data:");
    f.sync_data()?; // '?'は、Resultのエラーを伝播する場合に利用する。現在の関数戻り値がResultの場合にOK。
                    // match f.sync_data() {
                    //     Ok(_) => {},
                    //     Err(e) => { panic!("Problem sync_data: {:?}", e) },
                    // }

    debug!("sync_data ok.");
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

fn read_csv(filename: &str) -> Result<Vec<User>, io::Error> {
    // let f = File::options().read(true).write(false).open(filename);
    let f = File::open(filename).expect(format!("file open error.[{}]", filename).as_str());

    let reader = BufReader::new(f);

    //
    // csv::cookbook - Rust https://docs.rs/csv/1.1.6/csv/cookbook/index.html
    let mut csvrdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(reader);

    let mut row_number = 0;

    let mut users: Vec<User> = Vec::new();

    for result in csvrdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        debug!("[{}] {:?}", row_number + 1, record);

        // zero dateはout of rangeとなって利用できない。
        // let zeronaive = NaiveDate::from_ymd(0,0,0);
        // let zerodate = Local.from_local_date(&zeronaive).unwrap();
        users.push(User {
            no: record[0].parse().expect("no parse error."),
            name: record[1].to_string(),
            kana: record[2].to_string(),
            gender: record[3].to_string(),
            phone: record[4].to_string(),
            birth: util::to_localdate_with_format_opt(record[5].to_string().as_str(), "%Y/%m/%d"),
        });
        row_number += 1;
    }

    debug!("users={:?}", users);

    Ok(users)
}

fn research_datetime() {
    // 現在日時
    let local: DateTime<Local> = Local::now();

    debug!("now={:?}", local); // now=2022-05-31T10:09:38.930586+09:00

    // フォーマット
    // chrono::format::strftime - Rust https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers
    debug!(
        "now=[{}]",
        local.format("%Y-%m-%d %H:%M:%S.%3f").to_string()
    ); // now=[2022-05-31 10:18:17.871]

    // 時刻の生成(UTC)
    let _ = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    // let fixed_dt = dt.with_timezone(&FixedOffset::east(9 * 3600));

    // NG String -> datetime
    // DateTime::parse_from_str ヘルプによると、文字中にtimezoneが必要。
    // Note that this method *requires a timezone* in the string
    // let parsed_dt = DateTime::parse_from_str("2022-05-31 10:21:34", "%Y-%m-%d %H:%M:%S")
    //     .expect("parse date error.");

    // timezoneがない場合は、Naive〜を利用する。
    let parsed_dt_nontimezone =
        NaiveDateTime::parse_from_str("2022-05-31 10:21:34", "%Y-%m-%d %H:%M:%S")
            .expect("parse date error.");
    debug!("parsed_dt_nontimezone=[{:?}]", parsed_dt_nontimezone); // parsed_dt_nontimezone=[2022-05-31T10:21:34]

    // Naive -> DateTime（TimeZone付き）　に変換する
    let parsed_dt = Local.from_local_datetime(&parsed_dt_nontimezone).unwrap();
    debug!("parsed_dt=[{:?}]", parsed_dt); // parsed_dt=[2022-05-31T10:21:34+09:00]

    // UTCにする場合はこちら
    let parsed_utcdt = Utc.from_local_datetime(&parsed_dt_nontimezone).unwrap();
    debug!("parsed_utcdt=[{:?}]", parsed_utcdt); // parsed_utcdt=[2022-05-31T10:21:34Z]

    // TimeZone時間指定の場合ははこちら
    let parsed_offsetdt = FixedOffset::east(9 * 3600)
        .from_local_datetime(&parsed_dt_nontimezone)
        .unwrap();
    debug!("parsed_offsetdt=[{:?}]", parsed_offsetdt); // parsed_offsetdt=[2022-05-31T10:21:34+09:00]

    // util.rsに関数化(DateTime)
    let localdttime = util::ymdhms_to_localdatetime("2022-05-31 10:21:34").unwrap();
    debug!("toYMD_HMS_ToLocalTime=[{:?}]", localdttime);

    // util.rsに関数化(Date)
    let localdt = ymd_to_localdate("2022-05-31").unwrap();
    debug!("toYMD_ToLocalDate=[{:?}]", localdt);
}

#[allow(dead_code)] // suppress "function is never used" warning.
fn sort_users(mut users: Vec<User>) -> Vec<User> {
    // users.sort_by(|a, b|  { b.no.cmp(&a.no) });
    users.sort_by(|a, b| a.birth.cmp(&b.birth));
    return users;
}

// function by reference
fn sort_users_ref(users: &mut Vec<User>) {
    // users.sort_by(|a, b|  { a.birth.cmp(&b.birth) });
    users.sort_by(|a, b| a.birth.cmp(&b.birth));
}

fn regexp_exam() {
    // static condition
    lazy_static! {
        static ref RE_YMD: Regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    }

    // Match
    let re_match = RE_YMD.is_match("2014-01-01");
    debug!("re.is_match={:?}", re_match);

    log_divider!("   ");

    // caputure
    let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    for cap in RE_YMD.captures_iter(text) {
        debug!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    }

    log_divider!("   ");
    // replace
    let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    // Cow<str> という構造が返る。
    // Cow in alloc::borrow - Rust https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html
    //
    let after = re.replace_all(before, "$m/$d/$y");
    debug!("replace_all before=[{}]", before);
    debug!("replace_all after =[{}]", after);
    assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");
}

fn hashmap_exam(users: Vec<User>) -> HashMap<String, User> {
    let mut map_users: HashMap<String, User> = HashMap::new();

    for user in users {
        debug!("[{:2}],[{}]", user.no, user.name);
        map_users.insert(user.name.clone(), user);
    }

    let oneuser = map_users.get("ＴＥＳＴ　ＴＡＲＯＵ");
    debug!("hashmap_get = {:?}", oneuser);

    let oneuser2 = map_users.get("hogehoge");
    debug!("hashmap_get(not found) = {:?}", oneuser2);

    map_users
}

// fn json_exam_users(_users: Vec<User>) {}

fn json_exam_mystruct() {
    // let john = json!(users);

    // debug!("users json = [{:?}]", john);

    let datas: Vec<MyPrimitive> = vec![
        MyPrimitive {
            no: 1,
            name: String::from("test"),
        },
        MyPrimitive {
            no: 2,
            name: String::from("test2"),
        },
        MyPrimitive {
            no: 3,
            name: String::from("test3"),
        },
    ];
    let data_json = json!(datas);
    debug!("MyPrimitive json = [{:?}]", data_json);
    debug!("MyPrimitive json string = [{:?}]", data_json.to_string());
    debug!(
        "MyPrimitive json string_pretty = [{:?}]",
        serde_json::to_string_pretty(&data_json).unwrap()
    );
}

fn json_exam_datetime() {
    let datas = Local.ymd(2022, 05, 31).and_hms(12, 29, 9);

    let data_json = json!(datas);
    debug!("datetime json = [{:?}]", data_json);
    debug!("datetime json string = [{:?}]", data_json.to_string());
    debug!(
        "datetime json string_pretty = [{:?}]",
        serde_json::to_string_pretty(&data_json).unwrap()
    );
}

// fn json_exam_date() {
//     let datas = ymd_to_localdate("2022-05-31").unwrap();

//     let data_json = json!(datas);
//     debug!("datetime json = [{:?}]", data_json);
//     debug!("datetime json string = [{:?}]", data_json.to_string());
//     debug!(
//         "datetime json string_pretty = [{:?}]",
//         serde_json::to_string_pretty(&data_json).unwrap()
//     );
// }

fn json_exam_datestruct() {
    // let datas = ymd_to_localdate("2022-05-31").unwrap();
    let datas: Vec<MyDateStruct> = vec![
        MyDateStruct {
            no: 1,
            dt: Some(ymd_to_localdate("2022-05-31").unwrap()),
        },
        MyDateStruct { no: 2, dt: None },
    ];

    let data_json = json!(datas);
    debug!("datestruct json = [{:?}]", data_json);
    debug!("datestruct json string = [{:?}]", data_json.to_string());
    debug!(
        "datestruct json string_pretty = [{:?}]",
        serde_json::to_string_pretty(&data_json).unwrap()
    );

    // let srcstr = "[{\"dt\":\"2022-12-31\",\"no\":10},{\"dt\":null,\"no\":11}]";
    let srcstr = r#"[
        {"dt":"2022-12-31","no":10},
        {"dt":null,"no":11},
        {"dt":"2022-12-32","no":12},
        {"dt":"","no":13}
        ]"#;

    let v: Vec<MyDateStruct> = serde_json::from_str(srcstr).unwrap();
    debug!("datestruct json string_pretty = [{:?}]", v);
}

// in_filename の拡張子を.outにして、書き出すサンプル。
fn fileread_write(in_fname: &str) -> Result<(), Box<dyn Error>> {
    debug!("fileread_write in:[{}]", in_fname);

    // infile
    let f_in = File::options().read(true).open(in_fname)?;
    let mut reader = BufReader::new(f_in);

    // out
    let mut out_fname = String::from(in_fname);
    out_fname += ".out";
    debug!("fileread_write out:[{}]", out_fname);
    let f_out = File::create(out_fname)?;

    let mut writer = BufWriter::new(f_out);

    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let line_trimed = line.trim_end();
        debug!("line = [{}]", line_trimed);
        writer.write(line.as_bytes())?;
        line.clear(); // read_line はappendするので１行ずつの場合はクリアする。
    }

    Ok(())
}

fn path_exam() {
    // Path - Rust By Example 日本語版 https://doc.rust-jp.rs/rust-by-example-ja/std_misc/path.html

    // Create a `Path` from an `&'static str`
    // `&'static str`から`Path`を作成
    let path = Path::new(".");

    debug!("Path::new(\".\")={:?}", path);

    // The `display` method returns a `Display`able structure
    // `display`メソッドは`Display`可能な構造体を返す。
    let _display = path.display();
    debug!("path.display()={:?}", _display);

    // std::path::absolute はまだExperimental（Rust 1.61）
    debug!("path ={:?}", path);

    debug!("current dir ={:?}", env::current_dir().unwrap());
    debug!("current OS ={:?}", env::consts::OS); // "macos"

    let current_exe = env::current_exe().unwrap();
    debug!("current exe ={:?}", current_exe);
    debug!("current extension ={:?}", current_exe.extension());
    debug!("current file_name ={:?}", current_exe.file_name());
    debug!("current file_stem ={:?}", current_exe.file_stem());
    debug!("current has_root ={:?}", current_exe.has_root());
    debug!("current has_root ={:?}", current_exe.parent());
    debug!("current is_file={:?}", current_exe.is_file());
    debug!("current is_dir={:?}", current_exe.is_dir());

    let current_exe_metadata = current_exe.metadata().unwrap(); // Timestamp など。
    debug!("current metadata={:?}", current_exe_metadata);
    debug!("current metadata(len)={:?}", current_exe_metadata.len());

    debug!("has_root(./) ={:?}", Path::new("./").has_root());

    let passwd_path = Path::new("/etc").join("passwd");

    // Convert the path into a string slice
    // パスを文字列のスライスに変換する。
    match passwd_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => debug!("new path is {}", s),
    }
}

fn main() {
    // 時刻のフォーマットは、ここを参照。
    // Format description - Time https://time-rs.github.io/book/api/format-description.html
    let log_config = ConfigBuilder::new()
        // .set_time_format_rfc3339() // 2022-06-03T06:25:12.001052Z
        // .set_time_format_rfc2822()   // Fri, 03 Jun 2022 06:24:54 +0000
        .set_time_format_custom(format_description!(
            "[hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .build();

    let app_config = config::Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("app.toml"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    // println!(
    //     "{:?}",
    //     settings
    //         .try_deserialize::<HashMap<String, String>>()
    //         .unwrap(),
    // );

    // .intoは型推論で目的の型にCASTする。
    let log_dir = app_config
        .get_string("log_dir")
        .unwrap_or_else(|_| String::from(""));

    let log_level = app_config
        .get_string("log_level")
        .unwrap_or_else(|_| String::from(""))
        .to_lowercase();

    // .get_bool("debug")
    // .unwrap_or_else(|_| false.into());
    // let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());

    println!("config log_dir=[{}]", log_dir);
    println!("config log_level=[{}]", log_level);

    let logfname = format!(
        "{}/app{}.log",
        log_dir,
        Local::now().format("%Y%m%d").to_string()
    );

    let level_filter = match log_level.as_str() {
        "" => log::LevelFilter::Info,
        _ => log::LevelFilter::from_str(log_level.as_str()).expect("illegal config log string."),
    };

    CombinedLogger::init(vec![
        TermLogger::new(
            level_filter,
            log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            level_filter,
            log_config.clone(),
            File::create(logfname).unwrap(),
        ),
    ])
    .unwrap();

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");

    // DateTimeの扱いの検証
    research_datetime();

    // 引数の扱いの検証
    let args: Vec<String> = env::args().collect();
    debug!("{:?}", args);

    if args.len() == 1 {
        info!("usage:csvexam filename");
        process::exit(1);
    }
    let filename = &args[1];

    // let fname = "test.txt";
    // match filecheck(&filename) {
    //     Ok(mes)=>{println!("mes:[{}]",mes)}
    //     Err(e) => { panic!("Problem filecheck: {:?}", e) },
    // }

    log_divider!("");

    // void的な関数

    let _ = funcret01();

    log_divider!("file read");

    // file read
    let _ = read_file(&filename);

    log_divider!("csv read");

    // csv read
    let mut users = read_csv(&filename).unwrap();

    log_divider!("sort");
    debug!("before sort users = {:?}", users);
    // let users_sorted = sort_users(users);
    // println!("after sort users = {:?}",users_sorted);

    // rust - Passing a Vec into a function by reference - Stack Overflow https://stackoverflow.com/questions/24102615/passing-a-vec-into-a-function-by-reference
    sort_users_ref(&mut users);
    debug!("after sort(ref) users = {:?}", users);

    // Regular expression examine.
    log_divider!("Regular expression");
    regexp_exam();

    // HashMap examine.
    log_divider!("HashMap");
    // HashMapにmoveされるので、to_vecでcloneを作っておく。
    let map_users = hashmap_exam(users.to_vec());
    debug!("map_users={:?}", map_users);

    // JSON examine.
    log_divider!("JSON");
    json_exam_mystruct();
    json_exam_datetime();
    json_exam_datestruct();

    // File read write examine.
    log_divider!("File read/write.");
    match fileread_write(filename) {
        Ok(v) => v,
        Err(e) => {
            error!("fileread_write error {:?}", e)
        }
    }

    log_divider!("Path");

    path_exam();

    // if let Err(err) = example() {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }
}
