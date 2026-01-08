use clap::Parser;
use rpg::{
    colorize, error::Result, output, CliArgs, PassphraseConfig, PassphraseGenerator,
    PasswordConfig, PasswordGenerator, RpgError,
};
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = CliArgs::parse();

    // 生成個数の検証
    validate_args(&args)?;

    // パスフレーズモードと通常モードで分岐
    if args.passphrase {
        run_passphrase_mode(&args)
    } else {
        run_password_mode(&args)
    }
}

fn validate_args(args: &CliArgs) -> Result<()> {
    if args.number == 0 {
        return Err(RpgError::InvalidGenerationCount(0));
    }
    Ok(())
}

fn run_password_mode(args: &CliArgs) -> Result<()> {
    let config = PasswordConfig::from(args.clone());
    let generator = PasswordGenerator::new(config)?;

    // 指定された個数のパスワードを生成
    let passwords: Vec<String> = (0..args.number).map(|_| generator.generate()).collect();

    // カラー出力の判定
    let enable_color = output::should_enable_color(args.no_color);

    // 出力またはクリップボードにコピー
    output::output_or_copy(
        passwords,
        args.copy,
        colorize::colorize_password,
        enable_color,
    )
}

fn run_passphrase_mode(args: &CliArgs) -> Result<()> {
    let config = PassphraseConfig {
        word_count: args.words,
    };
    let generator = PassphraseGenerator::new(config)?;

    // 指定された個数のパスフレーズを生成
    let passphrases: Vec<String> = (0..args.number).map(|_| generator.generate()).collect();

    // カラー出力の判定
    let enable_color = output::should_enable_color(args.no_color);

    // 出力またはクリップボードにコピー
    output::output_or_copy(
        passphrases,
        args.copy,
        colorize::colorize_passphrase,
        enable_color,
    )
}
