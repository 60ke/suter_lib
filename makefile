MAKE	= make -r

lib:
	cargo +nightly build --release
	rm -fr nodejs/libsuter_lib*
	cp target/release/libsuter_lib* nodejs/	


clean:
	cargo clean
