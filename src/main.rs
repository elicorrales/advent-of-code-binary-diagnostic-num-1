use std::env::args;
use std::fs::File;
use std::process::exit;

fn main() {
    let args:Vec<String> = args().collect();
    //println!("{:?}", args);
    if args.len() < 2 {
        println!("\nNeed path to inputs file.\n");
        exit(1);
    }

    let path = &args[1];

    let err_msg = format!("\nFile {} was not found.\n", path);
    let file = File::open(path);
    
    match file {
        Ok(f) => { println!("Found file {:?}", f); },
        _ => { println!("{}", err_msg); }
    }
}
