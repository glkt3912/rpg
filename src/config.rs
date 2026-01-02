use crate::error::{Result, RpgError};
use clap::Parser;

/// パスワード生成の設定
#[derive(Debug, Clone, PartialEq)]
pub struct PasswordConfig {
    /// パスワードの長さ
    pub length: usize,
    /// 大文字を含む
    pub use_uppercase: bool,
    /// 小文字を含む
    pub use_lowercase: bool,
    /// 数字を含む
    pub use_digits: bool,
    /// 記号を含む
    pub use_symbols: bool,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            length: 16,
            use_uppercase: true,
            use_lowercase: true,
            use_digits: true,
            use_symbols: true,
        }
    }
}

impl PasswordConfig {
    /// 設定の妥当性を検証
    pub fn validate(&self) -> Result<()> {
        // 長さのチェック
        if self.length == 0 {
            return Err(RpgError::InvalidLength(self.length));
        }

        // 最大長のチェック（メモリ安全性）
        const MAX_LENGTH: usize = 1024;
        if self.length > MAX_LENGTH {
            return Err(RpgError::LengthTooLarge(self.length));
        }

        // 少なくとも1つの文字種が有効かチェック
        if !self.use_uppercase && !self.use_lowercase && !self.use_digits && !self.use_symbols {
            return Err(RpgError::NoCharacterSetsEnabled);
        }

        Ok(())
    }
}

/// CLI引数定義
#[derive(Parser, Debug)]
#[command(
    name = "rpg",
    version = "0.1.0",
    about = "A secure command-line password generator",
    long_about = None
)]
pub struct CliArgs {
    /// Length of the password
    #[arg(short, long, default_value = "16")]
    pub length: usize,

    /// Exclude uppercase letters
    #[arg(long, default_value = "false")]
    pub no_uppercase: bool,

    /// Exclude lowercase letters
    #[arg(long, default_value = "false")]
    pub no_lowercase: bool,

    /// Exclude digits
    #[arg(long, default_value = "false")]
    pub no_digits: bool,

    /// Exclude symbols
    #[arg(long, default_value = "false")]
    pub no_symbols: bool,
}

impl From<CliArgs> for PasswordConfig {
    fn from(args: CliArgs) -> Self {
        Self {
            length: args.length,
            use_uppercase: !args.no_uppercase,
            use_lowercase: !args.no_lowercase,
            use_digits: !args.no_digits,
            use_symbols: !args.no_symbols,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PasswordConfig::default();
        assert_eq!(config.length, 16);
        assert!(config.use_uppercase);
        assert!(config.use_lowercase);
        assert!(config.use_digits);
        assert!(config.use_symbols);
    }

    #[test]
    fn test_validate_valid_config() {
        let config = PasswordConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_zero_length() {
        let config = PasswordConfig {
            length: 0,
            ..Default::default()
        };
        assert!(matches!(config.validate(), Err(RpgError::InvalidLength(0))));
    }

    #[test]
    fn test_validate_too_large() {
        let config = PasswordConfig {
            length: 2000,
            ..Default::default()
        };
        assert!(matches!(
            config.validate(),
            Err(RpgError::LengthTooLarge(2000))
        ));
    }

    #[test]
    fn test_validate_no_charsets() {
        let config = PasswordConfig {
            length: 16,
            use_uppercase: false,
            use_lowercase: false,
            use_digits: false,
            use_symbols: false,
        };
        assert!(matches!(
            config.validate(),
            Err(RpgError::NoCharacterSetsEnabled)
        ));
    }

    #[test]
    fn test_cli_args_to_config() {
        let args = CliArgs {
            length: 20,
            no_uppercase: true,
            no_lowercase: false,
            no_digits: false,
            no_symbols: true,
        };
        let config = PasswordConfig::from(args);
        assert_eq!(config.length, 20);
        assert!(!config.use_uppercase);
        assert!(config.use_lowercase);
        assert!(config.use_digits);
        assert!(!config.use_symbols);
    }
}
