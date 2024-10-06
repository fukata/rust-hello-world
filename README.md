# rust-hello-world

これはwasmを試す練習用のリポジトリです。

## インストール

```bash
$ cargo install wasm-bindgen-cli
$ cargo install wasm-pack
```

## ビルド

```bash
$ wasm-pack build --target web
```

## 実行

```bash
$ simple-http-server 8000
```

http://localhost:8000/index.html