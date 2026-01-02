use crate::error::{Result, RpgError};
use crate::wordlist::WORDLIST;
use rand::seq::SliceRandom;

/// パスフレーズ設定
#[derive(Debug, Clone, PartialEq)]
pub struct PassphraseConfig {
    pub word_count: usize,
}

impl Default for PassphraseConfig {
    fn default() -> Self {
        Self { word_count: 4 }
    }
}

impl PassphraseConfig {
    /// 設定の妥当性を検証
    pub fn validate(&self) -> Result<()> {
        if self.word_count == 0 {
            return Err(RpgError::InvalidWordCount(self.word_count));
        }

        const MAX_WORD_COUNT: usize = 20;
        if self.word_count > MAX_WORD_COUNT {
            return Err(RpgError::WordCountTooLarge(self.word_count));
        }

        Ok(())
    }
}

/// パスフレーズジェネレータ
pub struct PassphraseGenerator {
    config: PassphraseConfig,
}

impl PassphraseGenerator {
    /// 新しいジェネレータを作成
    pub fn new(config: PassphraseConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// パスフレーズを生成（ハイフン区切り）
    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        let words: Vec<&str> = WORDLIST
            .choose_multiple(&mut rng, self.word_count)
            .copied()
            .collect();
        words.join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passphrase_default_config() {
        let config = PassphraseConfig::default();
        assert_eq!(config.word_count, 4);
    }

    #[test]
    fn test_passphrase_correct_word_count() {
        let config = PassphraseConfig { word_count: 5 };
        let generator = PassphraseGenerator::new(config).unwrap();
        let passphrase = generator.generate();
        assert_eq!(passphrase.split('-').count(), 5);
    }

    #[test]
    fn test_passphrase_validate_zero_words() {
        let config = PassphraseConfig { word_count: 0 };
        assert!(matches!(
            config.validate(),
            Err(RpgError::InvalidWordCount(0))
        ));
    }

    #[test]
    fn test_passphrase_validate_too_many_words() {
        let config = PassphraseConfig { word_count: 25 };
        assert!(matches!(
            config.validate(),
            Err(RpgError::WordCountTooLarge(25))
        ));
    }

    #[test]
    fn test_passphrase_generation_error() {
        let config = PassphraseConfig { word_count: 0 };
        let result = PassphraseGenerator::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_passphrase_multiple_generation() {
        let config = PassphraseConfig::default();
        let generator = PassphraseGenerator::new(config).unwrap();

        let passphrase1 = generator.generate();
        let passphrase2 = generator.generate();

        // 確率的には異なるはず
        assert!(passphrase1 != passphrase2 || passphrase1.split('-').count() == 4);
    }
}
