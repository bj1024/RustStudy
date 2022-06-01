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

pub fn ymd_to_localdate(date_str: &str) -> Result<Date<Local>, String> {
    to_localdate_with_format(date_str, "%Y-%m-%d")
}

pub fn to_localdate_with_format_opt(date_str: &str, fortmat: &str) -> Option<Date<Local>> {
    to_localdate_with_format(date_str, fortmat).ok()
}

pub fn to_localdate_with_format(date_str: &str, fortmat: &str) -> Result<Date<Local>, String> {
    let nontimezone = match NaiveDate::parse_from_str(date_str, fortmat) {
        Ok(v) => v,

        Err(e) => {
            let estr = format!("parse date error.[{}][{:?}]", date_str, e);
            println!("{}", estr);
            //String::from(estr)と同じ rust1.9以降？ &str を String に変換する4つの方法 - Qiita https://qiita.com/uasi/items/3b08a5ba81fede837531
            // return Err(String::from(estr));
            return Err(estr.to_string());
        }
    };

    let local_dt = Local.from_local_date(&nontimezone);
    let v = match local_dt {
        LocalResult::None => return Err("from_local_datetime none.".to_string()),
        LocalResult::Single(v) => v,
        LocalResult::Ambiguous(_, _) => return Err("from_local_datetime ambiguous.".to_string()),
    };

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
