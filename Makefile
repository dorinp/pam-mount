all: pam_mount.rs
	rustc pam_mount.rs

clean:
	rm -f libpam*.so