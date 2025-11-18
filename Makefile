build: clean rebuild

rebuild:
	@rm -rf dist client/dist client/pkg
	@cd client && wasm-pack build --target web --out-dir pkg
	@mkdir -p client/dist
	@cp -r client/pkg/* client/dist/
	@cp client/index.html client/dist/
	@cp -r client/dist dist
	@cargo build 

release: clean
	@rm -rf dist client/dist client/pkg
	@cd client && wasm-pack build --release --target web --out-dir pkg
	@mkdir -p client/dist
	@cp -r client/pkg/* client/dist/
	@cp client/index.html client/dist/
	@cp -r client/dist dist
	@cargo build --release

clippy-common: 
	cargo clippy --manifest-path common/Cargo.toml
clippy-client: 
	cargo clippy --manifest-path client/Cargo.toml --target wasm32-unknown-unknown
clippy-server:
	cargo clippy --manifest-path Cargo.toml

clean: 
	cargo clean --manifest-path Cargo.toml
	cargo clean --manifest-path client/Cargo.toml
	cargo clean --manifest-path common/Cargo.toml
	rm -rf dist client/dist client/target client/pkg

tests: 
	cargo test --manifest-path Cargo.toml
	cargo test --manifest-path common/Cargo.toml