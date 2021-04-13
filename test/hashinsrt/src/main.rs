use std::collections::HashMap;
use serde_json::Value;


enum TypeVal {
	StrVal(Option<String>),
	BoolVal(Option<bool>),
	JsonVal(Option<Value>),
}

fn main() {
	let mut c :HashMap<String,TypeVal> = HashMap::new();
	let data = r#"
		{
			"data" : "ccs"
		}
	"#;
	let jsonv :Value = serde_json::from_str(data).unwrap();
	c.insert(String::from("cc"), TypeVal::StrVal(Some(String::from("cccb"))));
	c.insert(String::from("bb"),TypeVal::BoolVal(Some(false)));
	c.insert(String::from("json"),TypeVal::JsonVal(Some(jsonv)));
}
