test:
	echo "Remember why you started!"

# the appliaction depnds on rust, seaorm cli and deno
install-toolchain:

run-server:
	 cargo run --manifest-path utils-server/Cargo.toml 
	 
build-server:

watch-server:
	 @cd utils-server && cargo shuttle run --port 5000



build-cli:




