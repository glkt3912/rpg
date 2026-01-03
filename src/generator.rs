use crate::charset::build_charset;
use crate::config::PasswordConfig;
use crate::error::Result;
use rand::seq::SliceRandom;

/// パスワードジェネレータ
///
/// 文字セットは初期化時にキャッシュされます。
pub struct PasswordGenerator {
    config: PasswordConfig,
    charset: Vec<char>,
}

impl PasswordGenerator {
    pub fn new(config: PasswordConfig) -> Result<Self> {
        config.validate()?;
        let charset_str = build_charset(&config);
        let charset: Vec<char> = charset_str.chars().collect();
        Ok(Self { config, charset })
    }

    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();

        (0..self.config.length)
            .map(|_| *self.charset.choose(&mut rng).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_correct_length() {
        let config = PasswordConfig::default();
        let generator = PasswordGenerator::new(config.clone()).unwrap();
        let password = generator.generate();

        assert_eq!(password.len(), config.length);
    }

    #[test]
    fn test_generate_only_lowercase() {
        let config = PasswordConfig {
            length: 20,
            use_uppercase: false,
            use_lowercase: true,
            use_digits: false,
            use_symbols: false,
        };

        let generator = PasswordGenerator::new(config).unwrap();
        let password = generator.generate();

        assert_eq!(password.len(), 20);
        assert!(password.chars().all(|c| c.is_lowercase()));
    }

    #[test]
    fn test_generate_only_digits() {
        let config = PasswordConfig {
            length: 10,
            use_uppercase: false,
            use_lowercase: false,
            use_digits: true,
            use_symbols: false,
        };

        let generator = PasswordGenerator::new(config).unwrap();
        let password = generator.generate();

        assert_eq!(password.len(), 10);
        assert!(password.chars().all(|c| c.is_numeric()));
    }

    #[test]
    fn test_generate_no_charsets_error() {
        let config = PasswordConfig {
            length: 16,
            use_uppercase: false,
            use_lowercase: false,
            use_digits: false,
            use_symbols: false,
        };

        let result = PasswordGenerator::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_zero_length_error() {
        let config = PasswordConfig {
            length: 0,
            use_uppercase: true,
            use_lowercase: true,
            use_digits: true,
            use_symbols: true,
        };

        let result = PasswordGenerator::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_multiple_passwords_different() {
        let config = PasswordConfig::default();
        let generator = PasswordGenerator::new(config).unwrap();

        let password1 = generator.generate();
        let password2 = generator.generate();
        let password3 = generator.generate();

        // 確率的には異なるはず（理論上は同じ可能性もあるが極めて低い）
        assert!(password1 != password2 || password2 != password3);
    }
}
