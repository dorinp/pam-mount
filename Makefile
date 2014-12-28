all: clean src/pam_mount.rs
	cargo build --release

clean:
	cargo clean

install: all
	sudo cp target/release/libpam_mount*.so /lib/security/pam_mymount.so
