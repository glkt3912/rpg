//! rpg - Rust Password Generator
//!
//! 暗号学的に安全なパスワードとパスフレーズを生成するCLIツール。

pub mod charset;
pub mod clipboard;
pub mod colorize;
pub mod config;
pub mod error;
pub mod generator;
pub mod output;
pub mod passphrase;
pub mod wordlist;

// 主要な型を再エクスポート
pub use config::{CliArgs, PasswordConfig};
pub use error::{Result, RpgError};
pub use generator::PasswordGenerator;
pub use passphrase::{PassphraseConfig, PassphraseGenerator};
