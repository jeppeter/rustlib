extern crate addtype;
extern crate funccall;

use addtype::{print_func_name,print_all_links,call_list_all};
use funccall::{FuncName,call_functions};
use lazy_static::lazy_static;


fn bob_help(_k :&ExtKeyParse) -> String {
	return "bob help".to_string();
}

fn bob_json_set(_ns :NameSpaceEx,_k :ExtKeyParse,_v :Value) -> Result<(),Box<dyn Error>> {
	println!("bob_json_set");
	Ok(())
}

fn bob_value_set(_ns :NameSpaceEx,_i :i32,_k :ExtKeyParse, _params :Vec<String>) -> Result<i32,Box<dyn Error>> {
	println!("bob value set");
	return Ok(1);
}

fn bobparser_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _parser :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("bob parser handler");
	Ok(())
}

fn bobcall_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _parser :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("bob call handler");
	Ok(())
}


#[extargs_map_function(opthelp=bob_help,jsonfunc=bob_json_set,actfunc=bob_value_set,callbackfunc=bobparser_handler,bobcall_handler)]
pub fn bob_func() {
	let bcc = "hello_world_2";
	let cc = String::from("get_a_reply_2");
	return;
}