use crate::config::PasswordConfig;

/// 文字セット定義
pub struct CharacterSets;

impl CharacterSets {
    pub const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
    pub const DIGITS: &'static str = "0123456789";
    pub const SYMBOLS: &'static str = "!@#$%^&*()_+-=[]{}|;:,.<>?";
}

/// 設定に基づいて使用する文字セットを構築
pub fn build_charset(config: &PasswordConfig) -> String {
    let mut charset = String::new();

    if config.use_uppercase {
        charset.push_str(CharacterSets::UPPERCASE);
    }
    if config.use_lowercase {
        charset.push_str(CharacterSets::LOWERCASE);
    }
    if config.use_digits {
        charset.push_str(CharacterSets::DIGITS);
    }
    if config.use_symbols {
        charset.push_str(CharacterSets::SYMBOLS);
    }

    charset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_charset_all_enabled() {
        let config = PasswordConfig {
            length: 16,
            use_uppercase: true,
            use_lowercase: true,
            use_digits: true,
            use_symbols: true,
        };

        let charset = build_charset(&config);
        assert!(charset.contains('A'));
        assert!(charset.contains('a'));
        assert!(charset.contains('0'));
        assert!(charset.contains('!'));
    }

    #[test]
    fn test_build_charset_only_lowercase() {
        let config = PasswordConfig {
            length: 16,
            use_uppercase: false,
            use_lowercase: true,
            use_digits: false,
            use_symbols: false,
        };

        let charset = build_charset(&config);
        assert_eq!(charset, CharacterSets::LOWERCASE);
    }

    #[test]
    fn test_build_charset_uppercase_and_digits() {
        let config = PasswordConfig {
            length: 16,
            use_uppercase: true,
            use_lowercase: false,
            use_digits: true,
            use_symbols: false,
        };

        let charset = build_charset(&config);
        assert!(charset.contains('A'));
        assert!(!charset.contains('a'));
        assert!(charset.contains('0'));
        assert!(!charset.contains('!'));
    }

    #[test]
    fn test_build_charset_no_charsets() {
        let config = PasswordConfig {
            length: 16,
            use_uppercase: false,
            use_lowercase: false,
            use_digits: false,
            use_symbols: false,
        };

        let charset = build_charset(&config);
        assert!(charset.is_empty());
    }
}
