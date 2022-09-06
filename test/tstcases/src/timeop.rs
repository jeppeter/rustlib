
use chrono::prelude::*;
use std::error::Error;

pub fn timesec_to_tm(tsec :u64) -> Result<String,Box<dyn Error>> {
	let n : NaiveDateTime=  NaiveDateTime::from_timestamp(tsec as i64, 0);	
	let dt :DateTime<Utc> = DateTime::from_utc(n,Utc);
	let s = format!("{}",dt.format("%Y-%m-%d %H:%M:%S"));
	Ok(s)
}
