use std::io::BufferedReader;
use std::io::{File, IoResult};

fn read(user: &str, file: &str) -> Option<(String)> {
	use mdo::option::{bind, ret, mzero};
	// use mdo::result::{bind, ret};

	// let l = mdo! {
 //        z <- File::open(&Path::new(file)).ok();
 //        line <- file.lines();
 //        ret ret(line.to_string())
 //    };

	let f: IoResult<File> = File::open(&Path::new(file));

	if f.is_err() { None} else {
		let mut file = BufferedReader::new(f);
		file.lines().filter_map(|x| {
			mdo! {
		        line <- x.ok();
		        when !(line.starts_with("#"));
		        let h = line.words().collect::<Vec<&str>>();
		        when h.len() >= 2 && h[0]==user;
		        ret ret(h[1].to_string())
		    }
		}).next()
	}
}

pub fn container_for(user: &str, file: &str) -> Option<String> {
	read(user, file)
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