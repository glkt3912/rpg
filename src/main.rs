use clap::Parser;
use rpg::{
    CliArgs, PassphraseConfig, PassphraseGenerator, PasswordConfig, PasswordGenerator, RpgError,
    clipboard, colorize,
};
use std::process;

fn main() {
    let args = CliArgs::parse();

    // 生成個数の検証
    if args.number == 0 {
        eprintln!("{}", RpgError::InvalidGenerationCount(0));
        process::exit(1);
    }

    // パスフレーズモードと通常モードで分岐
    if args.passphrase {
        run_passphrase_mode(&args);
    } else {
        run_password_mode(&args);
    }
}

fn run_password_mode(args: &CliArgs) {
    let config = PasswordConfig::from(args.clone());
    let generator = match PasswordGenerator::new(config) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let enable_color = !args.no_color && atty::is(atty::Stream::Stdout);
    let mut last_password = String::new();

    for _ in 0..args.number {
        let password = generator.generate();
        last_password = password.clone();

        if !args.copy {
            let output = colorize::colorize_password(&password, enable_color);
            println!("{}", output);
        }
    }

    if args.copy {
        if let Err(e) = clipboard::copy_to_clipboard(&last_password) {
            eprintln!("{}", e);
            process::exit(1);
        }
        println!("Copied to clipboard!");
    }
}

fn run_passphrase_mode(args: &CliArgs) {
    let config = PassphraseConfig {
        word_count: args.words,
    };
    let generator = match PassphraseGenerator::new(config) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let enable_color = !args.no_color && atty::is(atty::Stream::Stdout);
    let mut last_passphrase = String::new();

    for _ in 0..args.number {
        let passphrase = generator.generate();
        last_passphrase = passphrase.clone();

        if !args.copy {
            let output = colorize::colorize_passphrase(&passphrase, enable_color);
            println!("{}", output);
        }
    }

    if args.copy {
        if let Err(e) = clipboard::copy_to_clipboard(&last_passphrase) {
            eprintln!("{}", e);
            process::exit(1);
        }
        println!("Copied to clipboard!");
    }
}
