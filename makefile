MAKE	= make -r

lib:
	cargo +nightly build --release
	rm nodejs/libsuter_lib.dylib
	cp target/release/libsuter_lib.dylib nodejs/libsuter_lib.dylib	


clean:
	cargo clean