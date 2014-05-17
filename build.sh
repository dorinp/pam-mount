#!/bin/sh

rm *.so
rustc --crate-type=dylib pam_wrapper.rs

sudo cp libpam_wrapper*.so /lib/security/pam_mymount.so
