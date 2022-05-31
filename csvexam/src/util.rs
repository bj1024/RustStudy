use chrono::{NaiveDateTime, Date ,DateTime,Local, TimeZone, LocalResult, NaiveDate};



pub fn toYMD_HMS_ToLocalDateTime(dateStr:&str )->Result<DateTime<Local>,String>{
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


pub fn toYMD_ToLocalDateOption(dateStr:&str)->Option<Date<Local>>{
  toYMD_ToLocalDate(dateStr).ok()
}

pub fn toYMD_ToLocalDate(dateStr:&str)->Result<Date<Local>,String>{
  toYMD_ToLocalDate_with_format(dateStr,"%Y-%m-%d")
}

pub fn toYMD_ToLocalDateOption_with_format(dateStr:&str,fortmat:&str )->Option<Date<Local>>{
  toYMD_ToLocalDate_with_format(dateStr,fortmat).ok()
}

pub fn toYMD_ToLocalDate_with_format(dateStr:&str,fortmat:&str )->Result<Date<Local>,String>{
  let nontimezone = match NaiveDate::parse_from_str(dateStr, fortmat){
    Ok(v) =>  v,

    Err(e) => {
      let estr = format!("parse date error.[{:?}]", e);

      //String::from(estr)と同じ rust1.9以降？ &str を String に変換する4つの方法 - Qiita https://qiita.com/uasi/items/3b08a5ba81fede837531
      // return Err(String::from(estr));
      return Err(estr.to_string());
    },
  };
  
  let local_dt = Local.from_local_date(&nontimezone);
  let v = match local_dt {
    LocalResult::None => return Err("from_local_datetime none.".to_string()),
    LocalResult::Single(v) => v,
    LocalResult::Ambiguous(_, _) => return Err("from_local_datetime ambiguous.".to_string()),
  };
  
  Ok(v)
}

// pub fn toYMD_ToOptionLocalDate(dateStr:&str ) -> Option<Date<Local>>{
  
//   match toYMD_ToLocalDate(dateStr){
//     Ok(v) => return Some(v) ,
//     Err(e) => {
//       println!("parse error [{}] {:?}",dateStr,e);
//       return None
//     }
//   }



// }
