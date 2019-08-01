// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.

    // Equivalent to `Status::Poor`.
    let status = Status::Poor;
    // Equivalent to `Work::Civilian`.
    let work = Work::Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Status::Rich => println!("The rich have lots of money!"),
        Status::Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Work::Civilian => println!("Civilians work!"),
        Work::Soldier  => println!("Soldiers fight!"),
    }
}
