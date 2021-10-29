# Hello, world!!

`Hello, world!!` というテキストを画面に出力するプログラムを書く

Rust はコンパイル言語であるため、以下のような流れでプログラムを作成する

- `<プログラム名>.rs` という形式でプログラムファイルを作成する
- `<プログラム名>.rs` に以下のセルに書いたような `main` 関数を記述する
    - Rust においては、`main` 関数がプログラムのエントリーポイント（コンパイル後のプログラムで最初に自動実行される関数）となる
- ターミナルで `rustc <プログラム名>.rs` というコマンドを実行し、プログラムをコンパイルする
    - 実行ファイル `<プログラム名>` (Windows 上では `<プログラム名>.exe`) が生成される
- 生成された実行ファイルを実行する

## Environment

- OS:
    - Windows 11
        - Shell: PowerShell
    - Ubuntu 20.04
        - Shell: bash
- Editor: VSCode

***

## HelloWorld プログラムの作成

### プログラムファイルの作成
```bash
# VSCode で新規ファイル helloworld.rs を作成
$ code helloworld.rs
```

以下のように編集する

```rust:helloworld.rs
// helloworld.rs

fn main() {
    println!("Hello, world!!");
}
```

### プログラムのコンパイル
```bash
# helloworld.rs をコンパイルして実行ファイルを生成
$ rustc helloworld.rs

## => 以下のファイルが生成
### - Windows環境
###   - helloworld.exe -- 実行ファイル
###   - helloworld.pdb -- デバッグ情報ファイル
### - Ubuntu環境
###   - helloworld -- 実行ファイル
```

### プログラムの実行
```bash
# 生成した実行ファイルの起動
$ ./helloworld

Hello, world!!
```

***

## Rust プログラムの解剖

以下の行で Rust 関数（関数とは、何らかの入力を受け取り、何らかの出力を返す一連の処理を表す）を定義している

```rust
fn main() {
    
}
```

Rust において `main` 関数は特別な関数であり、常に全ての実行可能な Rust プログラムで走る最初のコードになる

1行目で、引数がなく（なんの入力も受け取らず）、何も返さない `main` という関数を宣言している

引数があるなら、かっこ（`()`）の内部に入力値を宣言することになる

また、関数の本体は波括弧（`{}`）に包まれて記述される

Rust では、全ての関数本体の周りに `{}` が必要になる

スペースを1つあけて、開き波括弧を関数宣言と同じ行に配置するスタイルが、Rust では良いスタイルとされている（言語によっては改行して次の行に開き波括弧を配置するのが良いスタイルとされている言語もある）

main 関数内には、以下のようなコードがある

```rust
    println!("Hello, world!");
```

この行が、この小さなプログラムの全作業を行っている（`Hello, world!` テキストを画面に出力する）

ここで気付くべき重要な事項が4つある

1. Rust のスタイルは、タブではなく、4スペースでインデントする
2. `println!` は Rust のマクロを呼び出す
    - 代わりに関数を呼ぶ場合は、`println` (`!`なし) と入力される
    - Rust のマクロについては後述するが、ここでは `!` を使用すると普通の関数ではなくマクロを呼んでいる、ということを知っておけば良い
3. `"Hello, world!"` という文字列を引数として `println!` に渡し、この文字列が画面に表示されている
4. この行をセミコロン(`;`)で終え、この式が終わり、次の式の準備ができていると示唆されている
    - Rust コードのほとんどの行はセミコロンで終わる

以上が Rust プログラムを記述する上での基礎的な知識となる

***

## Cargo

- **Cargo**:
    - Rust のビルドシステム兼、パッケージマネージャ
    - 通常 Rust プロジェクトの管理は Cargo を使って行う
    - Cargo は、コードのビルドや依存ライブラリのダウンロード・ビルド等多くの役割を果たす

前述した HelloWorld プログラムのようなコードには、依存するライブラリ（Rust では **クレート** と呼ぶ）がない

そのため、単純に `rustc` コマンドでコンパイルして実行するだけでも問題ない

しかし、プログラムが複雑化していき、依存クレート等が増えてくると、Cargo によるクレート管理やビルド支援が必須になってくる

### Cargo によるプロジェクト作成
前述の HelloWorld プログラムを、Cargo でプロジェクト化していく

プロジェクトの作成は `cargo new` コマンドで行う

```bash
# 新規プロジェクト `helloworld` 作成
$ cargo new helloworld

## => ./helloworld/ ディレクトリ生成
###    |_ src/        -- ソースコードディレクトリ
###    |  |_ main.rs  -- メインソースコード: `main` 関数が定義されている必要がある
###    |
###    |_ Cargo.toml  -- Cargo 設定ファイル (依存クレートやビルド方法等のプロジェクト設定)

# プロジェクトディレクトリに移動
$ cd ./helloworld/
```

### メインソースコードの記述
Cargo プロジェクトでは `src/main.rs` がメインソースコードととなり、このファイルに `main` 関数を定義する必要がある

```bash
# src/main.rs を VSCode で開いて編集
$ code ./src/main.rs
```

```rust:src/main.rs
// src/main.rs

fn main() {
    println!("Hello, world!!");
}
```

### プロジェクトのビルド・実行
`rustc` を使う場合は `rustc src/main.rs` のように直接コンパイルするソースコードを指定する必要があるが、`cargo` を使う場合は `cargo build` コマンドだけでビルドを行うことができる

```bash
# プロジェクトのビルド
$ cargo build

## => target/debug/ ディレクトリが生成され、その中に実行ファイルがビルドされる
###   - 実行ファイル名はプロジェクト名と同じ (今回の場合は `helloworld`) になる

# 実行ファイルの起動
$ ./target/debug/helloworld

Hello, world!!
```

Cargo の便利な機能として、`cargo run` コマンドでビルドと実行を一括で行うこともできる

```bash
# ソースコードをコンパイルして、生成した実行ファイルの起動も行う
$ cargo run

Hello, world!!
```

また、コンパイルを行わず、ソースコードが正しくコンパイル可能かどうかチェックするだけの `cargo check` というコマンドもある

```bash
# ソースコードのチェックを行う
$ cargo check

    Checking helloworld v0.1.0 (p/path/to/helloworld)
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
```

このように Cargo には便利なコマンドが多くあるため、基本的にプロジェクト管理は Cargo に任せてしまうのが楽である

### リリースビルドを行う
`cargo build` コマンドは、なんのオプションも指定しない場合デバッグビルドを行う

しかし、実際にプログラムをリリースする場合は、デバッグ情報等の余計な情報は不要である

そういった場合は `--release` オプションを指定してリリースビルドを行う

```bash
# リリースビルドを行う
$ cargo build --release

## => target/release/ ディレクトリが生成し、リリースビルドされた実行ファイルが中に保存される

# リリースビルドされた実行ファイルを起動
$ ./target/release/helloworld

Hello, world!!
```
