use std::error::Error;

use chrono::{Date, DateTime, Local, LocalResult, NaiveDate, NaiveDateTime, TimeZone};

pub fn ymdhms_to_localdatetime(date_str: &str) -> Result<DateTime<Local>, String> {
    // timezoneがない場合は、Naive〜を利用する。

    // format!("parse date error.[{:?}]", error).as_str()

    // let nontimezone = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|error| {
    //   Err("parse date error");
    // });
    let nontimezone = match NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        Ok(v) => v,

        Err(e) => {
            let estr = format!("parse date error.[{:?}]", e);

            //String::from(estr)と同じ rust1.9以降？ &str を String に変換する4つの方法 - Qiita https://qiita.com/uasi/items/3b08a5ba81fede837531
            // return Err(String::from(estr));
            return Err(estr.to_string());
        }
    };

    // Naive -> DateTime（TimeZone付き）　に変換する
    //   // let local_dt = Local.from_local_datetime(&nontimezone);
    // let local_dt = match Local.from_local_datetime(&nontimezone){
    //   Ok(v) =>  v,
    //   Err(e) => return Err("parse date error"),
    // }

    let local_dt = Local.from_local_datetime(&nontimezone);
    let v = match local_dt {
        LocalResult::None => return Err("from_local_datetime none.".to_string()),
        LocalResult::Single(v) => v,
        LocalResult::Ambiguous(_, _) => return Err("from_local_datetime ambiguous.".to_string()),
    };

    Ok(v)
}

// pub fn ymd_to_localdate_opt(date_str: &str) -> Option<Date<Local>> {
//     ymd_to_localdate(date_str).ok()
// }

pub fn ymd_to_localdate(date_str: &str) -> Result<Date<Local>, Box<dyn Error>> {
    return match to_localdate_with_format(date_str, "%Y-%m-%d"){
        Ok(v)=> Ok(v),
        Err(_)=>to_localdate_with_format(date_str, "%Y/%m/%d")
    }

}

pub fn to_localdate_with_format_opt(date_str: &str, fortmat: &str) -> Option<Date<Local>> {
    to_localdate_with_format(date_str, fortmat).ok()
}

// 文字の日付をDate<Local>に変換する。
// （エラー文字で返すバージョン）
// pub fn to_localdate_with_format(date_str: &str, fortmat: &str) -> Result<Date<Local>, String> {
//     let nontimezone = match NaiveDate::parse_from_str(date_str, fortmat) {
//         Ok(v) => v,

//         Err(e) => {
//             let estr = format!("parse date error.[{}][{:?}]", date_str, e);
//             println!("{}", estr);
//             //String::from(estr)と同じ rust1.9以降？ &str を String に変換する4つの方法 - Qiita https://qiita.com/uasi/items/3b08a5ba81fede837531
//             // return Err(String::from(estr));
//             return Err(estr.to_string());
//         }
//     };

//     let local_dt = Local.from_local_date(&nontimezone);
//     let v = match local_dt {
//         LocalResult::None => return Err("from_local_datetime none.".to_string()),
//         LocalResult::Single(v) => v,
//         LocalResult::Ambiguous(_, _) => return Err("from_local_datetime ambiguous.".to_string()),
//     };

//     Ok(v)
// }

// 文字の日付をDate<Local>に変換する。
// （下位エラーをBox<dyn Error>で返すバージョン）
// 　下位の複数種別のエラーを返す方法。
//   Box: ヒープへの参照を持つ構造。
//   dyn: dynamic　格納するサイズが動的な時に指定？
//   Error : std::error::Error
pub fn to_localdate_with_format(
    date_str: &str,
    fortmat: &str,
) -> Result<Date<Local>, Box<dyn Error>> {
    let nontimezone = NaiveDate::parse_from_str(date_str, fortmat)?;

    let local_dt = Local.from_local_date(&nontimezone);

    let v = local_dt.single().unwrap(); // ココは、LocalResult::None になることはないので、万一の場合はpanicとする。

    // let v = match Local.from_local_date(&nontimezone) {
    //     LocalResult::None => return Err("from_local_datetime none."),
    //     LocalResult::Single(v) => v,
    //     LocalResult::Ambiguous(_, _) => return Err("from_local_datetime ambiguous.".to_string()),
    // };

    Ok(v)
}

// pub fn toYMD_ToOptionLocalDate(date_str:&str ) -> Option<Date<Local>>{

//   match toYMD_ToLocalDate(date_str){
//     Ok(v) => return Some(v) ,
//     Err(e) => {
//       println!("parse error [{}] {:?}",date_str,e);
//       return None
//     }
//   }

// }




#[test]
fn test_to_localdate_with_format() {
    assert_eq!(
        to_localdate_with_format("2022-05-31", "%Y-%m-%d").unwrap(),
        Local.ymd(2022, 5, 31)
    );
}

#[test]
fn test_to_localdate_with_format_err() {
    let e = to_localdate_with_format("2022-05-32", "%Y-%m-%d")
        .err()
        .unwrap();
    assert!(e.is::<chrono::ParseError>());

    // assert_eq!(.map_err(|e| e.kind()),
    // Err(chrono::ParseError::OutOfRange));
}
