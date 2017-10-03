extern crate markov;

use markov::Chain;
use std::process::Command;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("movie_lines.txt");
	let file = Path::new("lol.txt");

    let mut chain = Chain::new();
    chain.feed_file(path);
    
    let display = file.display();
    let mut file = match File::create(file) {
        Err(why) => panic!("couldn't create {}: {}",
        	display,
            why.description()),
        Ok(file) => file,
    };

    for x in 1..4 {
    	let status: String = chain.generate_str();

    	match file.write(format!("{} {}", status, "\n").as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => print!(""),
    }
    }

	let output = if cfg!(target_os = "windows") {
    	Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
	} else {
    	Command::new("sh")
            .arg("-c")
            .arg("cat lol.txt | pbcopy")
            .output()
            .expect("failed to execute process")
		};
}