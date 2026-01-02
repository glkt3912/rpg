use std::fmt;

/// rpgライブラリのエラー型
#[derive(Debug, Clone, PartialEq)]
pub enum RpgError {
    /// すべての文字種が無効化されている
    NoCharacterSetsEnabled,
    /// 不正なパスワード長（0以下）
    InvalidLength(usize),
    /// パスワード長が大きすぎる（メモリ安全性のため）
    LengthTooLarge(usize),
    /// その他のエラー
    Other(String),
}

impl fmt::Display for RpgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RpgError::NoCharacterSetsEnabled => {
                write!(f, "Error: At least one character set must be enabled")
            }
            RpgError::InvalidLength(len) => {
                write!(f, "Error: Invalid password length: {}", len)
            }
            RpgError::LengthTooLarge(len) => {
                write!(f, "Error: Password length {} is too large (max: 1024)", len)
            }
            RpgError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for RpgError {}

/// rpgライブラリのResult型エイリアス
pub type Result<T> = std::result::Result<T, RpgError>;
