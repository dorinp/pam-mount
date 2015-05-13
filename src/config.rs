use std::io::BufReader;
use std::io;
use std::fs::File;
use std::io::prelude::*;

fn read(user: &str, file: &str) -> io::Result<String> {
	let f = try!(File::open(file));

	let file = BufReader::new(f);
	let mut xx = file.lines().filter_map(|line|  {
// if !line.unwrap().starts_with("#")
        let u = line.unwrap();
        let h = u.split_whitespace().collect::<Vec<&str>>();
        if h.len() >= 2 && h[0]==user { Some(h[1].to_string()) } else { None }
	});

    Ok(xx.next().unwrap())
}

pub fn container_for(user: &str, file: &str) -> Option<String> {
	read(user, file).ok()
}

#[cfg(test)]
mod tests {
	use config::read;
    #[test]
    fn kaboom() {
    	assert_eq!(read("user", "src/pam_mount.conf"), Some("/dev/sdo".to_string()));
    	assert_eq!(read("user2", "src/pam_mount.conf"), Some("hello".to_string()));
    	assert_eq!(read("user", "nonexistant.file"), None);
    }
}