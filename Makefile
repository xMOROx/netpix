build: rebuild clean

rebuild:
	@rm -rf dist client/dist
	@cd client && trunk build 
	cd ../
	@cp -r client/dist dist
	@cargo build 

release: clean
	@rm -rf dist client/dist
	@cd client && trunk build --release
	cd ../
	@cp -r client/dist dist
	@cargo build --release

clippy-common: 
	cargo clippy --manifest-path common/Cargo.toml
clippy-client: 
	cargo clippy --manifest-path client/Cargo.toml
clippy-server:
	cargo clippy --manifest-path Cargo.toml

clean: 
	cargo clean --manifest-path Cargo.toml
	cargo clean --manifest-path client/Cargo.toml
	cargo clean --manifest-path common/Cargo.toml
	rm -rf dist client/dist client/target

tests: 
	cargo test --manifest-path Cargo.toml
	cargo test --manifest-path common/Cargo.toml