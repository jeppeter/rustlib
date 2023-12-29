
use chrono::prelude::*;
use std::error::Error;

#[allow(deprecated)]
pub fn timesec_to_tm(tsec :u64) -> Result<String,Box<dyn Error>> {
	//let n : NaiveDateTime=  NaiveDateTime::from_timestamp(tsec as i64, 0);	
	let n : NaiveDateTime=  NaiveDateTime::from_timestamp_opt(tsec as i64, 0).unwrap();	
	let dt :DateTime<Utc> = DateTime::from_utc(n,Utc);
	let s = format!("{}",dt.format("%Y-%m-%d %H:%M:%S"));
	Ok(s)
}
