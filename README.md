pam-mount
=========

A Linux PAM module that automatically mounts a LUKS encrypted container at login 

Uses the login password as the key to unlock the LUKS container and mounts it as the home directory

Written in the  [Rust](http://www.rust-lang.org/) language.

run `make install` to compile and copy the binary to /lib/security

The pam config files need to be edited so that the module is called for `auth` and `session` callbacks
