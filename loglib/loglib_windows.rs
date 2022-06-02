use winapi::um::{debugapi};


fn win_output_debug(s :&str) {
	let wso : Option<Box<u16>> =  str_to_c_wstr(s);
	if wso.is_none() {
		return;
	}
	let ws = wso.unwrap();

    unsafe {
        debugapi::OutputDebugStringW(final_str);
    }
    return;
}