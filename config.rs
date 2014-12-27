use std::io::BufferedReader;
use std::io::{File, IoResult};

fn read(user: &str, file: &str) -> Option<(String)> {
	let f: IoResult<File> = File::open(&Path::new(file));
	if f.is_err() {None} else {
		let mut file = BufferedReader::new(f);
		file.lines().filter_map(|x| {
			if x.is_ok() {
				let line = x.unwrap();
				if line.starts_with("#") {None} else {
					let h: Vec<&str> = line.words().collect();
					if h.len() >= 2 && h[0]==user {
						Some(h[1].to_string())
					} else {None} 
				}
			} else {None}
		}).next()
	}
}

pub fn container_for(user: &str, file: &str) -> Option<String> {
	read(user, file)
}

#[allow(dead_code)]
fn main() {
	println!("{}", read("user", "pam_mount.conf"));
	println!("{}", read("user2", "pam_mount.conf"));
	println!("{}", read("user", "nonexistant.file"));
}