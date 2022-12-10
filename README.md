# Rustテスト導入

## 概要

***

## 環境構築

### インストール
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
export PATH="$HOME/.cargo/bin:$PATH"
```

### バージョン確認
```sh
rustc -V
cargo -V
rustup -V
```

### コンパイル
```sh
rustc index.rs
```

### 実行
```sh
./index
```

***

## 参考サイト

### 公式サイト
https://www.rust-lang.org/ja

### とほほのRust入門
https://www.tohoho-web.com/ex/rust.html
