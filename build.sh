#!/bin/bash

# mini
RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release
upx --ultra-brute ./target/release/ak_monitor_client_rs     
