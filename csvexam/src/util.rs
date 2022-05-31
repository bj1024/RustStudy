use chrono::{NaiveDateTime, DateTime,Local, ParseError, TimeZone, LocalResult};



pub fn toYMD_HMS_ToLocalTime(dateStr:&str )->Result<DateTime<Local>,String>{
    // timezoneがない場合は、Naive〜を利用する。
    
    // format!("parse date error.[{:?}]", error).as_str()

    // let nontimezone = NaiveDateTime::parse_from_str(dateStr, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|error| {
    //   Err("parse date error");
		// });
    let nontimezone = match NaiveDateTime::parse_from_str(dateStr, "%Y-%m-%d %H:%M:%S"){
      Ok(v) =>  v,

      Err(e) => {
        let estr = format!("parse date error.[{:?}]", e);

        //String::from(estr)と同じ rust1.9以降？ &str を String に変換する4つの方法 - Qiita https://qiita.com/uasi/items/3b08a5ba81fede837531
        // return Err(String::from(estr));
        return Err(estr.to_string());
      },
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
