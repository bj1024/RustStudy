use chrono::{NaiveDateTime, DateTime,Local, ParseError, TimeZone, LocalResult};



pub fn toYMD_HMS_ToLocalTime(dateStr:&str )->Result<DateTime<Local>,&'static str>{
    // timezoneがない場合は、Naive〜を利用する。
    
    // format!("parse date error.[{:?}]", error).as_str()

    // let nontimezone = NaiveDateTime::parse_from_str(dateStr, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|error| {
    //   Err("parse date error");
		// });
    let nontimezone = match NaiveDateTime::parse_from_str(dateStr, "%Y-%m-%d %H:%M:%S"){
      Ok(v) =>  v,
      Err(e) => return Err("parse date error"),
    };
    
    // Naive -> DateTime（TimeZone付き）　に変換する
  //   // let local_dt = Local.from_local_datetime(&nontimezone);
    // let local_dt = match Local.from_local_datetime(&nontimezone){
    //   Ok(v) =>  v,
    //   Err(e) => return Err("parse date error"),
    // }

    let local_dt = Local.from_local_datetime(&nontimezone);
    let v = match local_dt {
      LocalResult::None => return Err("from_local_datetime none."),
      LocalResult::Single(v) => v,
      LocalResult::Ambiguous(_, _) => return Err("from_local_datetime ambiguous."),
    };
    
    Ok(v)
}
