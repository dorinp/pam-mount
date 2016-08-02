all:
	cargo build --release

clean:
	cargo clean

strip:
	strip  target/release/libpam_mount.so
# sstrip

install: all strip
	sudo cp target/release/libpam_mount*.so /lib/security/pam_mymount.so
