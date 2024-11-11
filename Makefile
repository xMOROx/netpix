build:
	@cd client && trunk build 
	cd ../
	@cp -r client/dist dist
	@cargo build 

clean: 
	cargo clean --manifest-path Cargo.toml
	cargo clean --manifest-path client/Cargo.toml
	cargo clean --manifest-path common/Cargo.toml
	rm -rf dist client/dist client/target
