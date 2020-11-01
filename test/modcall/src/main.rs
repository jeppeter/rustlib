mod modcall;
fn main() {
	let s = String::from("xxx");
	let cs :String;
    modcall::call_1(&s);
    cs = modcall::nest::call_3(20);
    println!("{}", cs);

}
