use std::thread;
use std::time::Duration;
use std::env;
use std::error::Error;

fn generate_workout(intensity :i32, rannum :i32) {
	let expectnum = |num| {
		println!("calculation ");
		thread::sleep(Duration::from_secs(2));
		num
	};

	if intensity >= 25 {
		println!("Today do {} pushups", expectnum(intensity));
		println!("Next do {} situps", expectnum(intensity));
	} else {
		if rannum == 3 {
			println!("Take a break Today");
		} else {
			println!("Today run {} minutes", expectnum(intensity));
		}
	}
}

fn main() -> Result<(),Box<dyn Error>> {
    let argv :Vec<String> = env::args().collect();
    if argv.len() >= 3 {
    	let intensity :i32 = argv[1].parse()?;
    	let rannum :i32 = argv[2].parse()?;
    	generate_workout(intensity,rannum);
    }
    Ok(())
}
