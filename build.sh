#!/bin/sh

rm *.so
rustc --crate-type=dylib pam_wrapper.rs
