use rand::Rng; // rand::thread_rng().gen_range() 関数を使えるようにする
use std::cmp::Ordering; // std::cmp::Ordering 型をスコープに導入
use std::io::{self, Write}; // 標準入出力関数を使えるようにする

// Rust プログラムエントリーポイント
fn main() {
    println!("数当てゲームを始めます！");
    
    // 秘密の数字: secret_number 変数に 1～100 の乱数を束縛
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // 無限ループ処理
    loop {
        // ユーザ入力を guess 変数に束縛（文字列型 => 整数型に変換）
        let mut guess: String = String::new(); // std::io::stdin().read_line() でデータ書き込みできるように mutable で宣言

        print!("数を予想して入力してください：");
        io::stdout().flush().unwrap(); // 標準出力をフラッシュして、次行の標準入力と前行の標準出力を一行で行う
        io::stdin()
            .read_line(&mut guess) // io::stdin::read_line は指定されたポインタ位置に文字列データを書き込むため &mut String 参照を渡す
            .expect("読み込み失敗");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Ok 列挙子に格納されている値（数値）をそのまま返す
            Err(_) => continue, // 以降の処理をすべてスキップし、ループの最初に戻る
        };

        // ユーザ入力値と秘密の数字を比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("秘密の数字より小さいです"),
            Ordering::Greater => println!("秘密の数字より大きいです"),
            Ordering::Equal => { // 式は {} で囲むことで複数処理を記述することができる
                println!("当たりです！");
                break; // ループを抜ける
            },
        };
    };
}