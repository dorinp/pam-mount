use std::io::BufReader;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use syslog;

fn read(user: &str, file: &str) -> io::Result<String> {
	let f = try!(File::open(file));
	let file = BufReader::new(f);
	let mut xx = file.lines().filter_map(|l| {
        match l {
            Ok(line) => if !line.starts_with("#") {
                let h = line.split_whitespace().collect::<Vec<&str>>();
                if h.len() >= 2 && h[0]==user { Some(h[1].to_string()) } else { None }
            } else { None },
            Err(e)   => { syslog::err(format!("{}", e).as_str()); None }
        }
   	});

    xx.next().ok_or(Error::new(ErrorKind::Other, "oh no!"))
}

pub fn container_for(user: &str, file: &str) -> Option<String> {
	read(user, file).ok()
}

#[cfg(test)]
mod tests {
	use config::read;
    use std::io::{Error, ErrorKind};
    use std::io;

    #[test]
    fn kaboom() {
        // assert_eq!(Err(Error::new(ErrorKind::Other, "oh no!")), Ok("bzzz"));
        // println!("zzz ->>>>> {:?}", read("user", "pam_mount.conf"));
    	assert_eq(read("user", "src/pam_mount.conf"), Ok("/dev/sdo".to_string()));
    	assert_eq(read("user2", "src/pam_mount.conf"), Ok("hello".to_string()));
    	assert_eq(read("user", "nonexistant.file"), Err(Error::new(ErrorKind::NotFound, "No such file or directory (os error 2)")));
    }

    fn assert_eq(l: io::Result<String>, r: io::Result<String>) {
        assert_eq!(l.map_err(|e| e.to_string()), r.map_err(|e| e.to_string()));   
    }
}