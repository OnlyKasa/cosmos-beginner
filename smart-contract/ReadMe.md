```bash
cargo build --target wasm32-unknown-unknown --release
```

Note: Thêm vào .cargo/config.toml để chạy câu lệnh 
```
[alias]
wasm = "build --target wasm32-unknown-unknown --release"
wasm-debug = "build --target wasm32-unknown-unknown"
```