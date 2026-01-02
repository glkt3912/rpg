use clap::Parser;
use rpg::{CliArgs, PasswordConfig, PasswordGenerator};
use std::process;

fn main() {
    // CLI引数をパース
    let args = CliArgs::parse();

    // 設定に変換
    let config = PasswordConfig::from(args);

    // ジェネレータを作成
    let generator = match PasswordGenerator::new(config) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // パスワードを生成
    let password = generator.generate();

    // 出力
    println!("{}", password);
}
