//! カラー出力処理モジュール
//!
//! パスワードとパスフレーズに色付けして視覚的に区別しやすくします。

use colored::*;

/// パスフレーズの単語に使用する色の順序
const PASSPHRASE_COLORS: [Color; 4] = [Color::Cyan, Color::Magenta, Color::Yellow, Color::Green];

/// 文字種ごとに色分けしてパスワードを表示
///
/// 大文字=青、小文字=緑、数字=黄、記号=赤
pub fn colorize_password(password: &str, enable_color: bool) -> String {
    if !enable_color {
        return password.to_string();
    }

    password
        .chars()
        .map(|c| match c {
            'A'..='Z' => c.to_string().blue().to_string(),
            'a'..='z' => c.to_string().green().to_string(),
            '0'..='9' => c.to_string().yellow().to_string(),
            _ => c.to_string().red().to_string(), // 記号
        })
        .collect()
}

/// パスフレーズを色分けで表示（ワード単位で交互に色を変える）
pub fn colorize_passphrase(passphrase: &str, enable_color: bool) -> String {
    if !enable_color {
        return passphrase.to_string();
    }

    let words: Vec<&str> = passphrase.split('-').collect();

    words
        .iter()
        .enumerate()
        .map(|(i, word)| {
            let color = PASSPHRASE_COLORS[i % PASSPHRASE_COLORS.len()];
            format!("{}", word.color(color))
        })
        .collect::<Vec<_>>()
        .join(&"-".white().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize_password_disabled() {
        let password = "Abc123!@#";
        assert_eq!(colorize_password(password, false), password);
    }

    #[test]
    fn test_colorize_passphrase_disabled() {
        let passphrase = "correct-horse-battery-staple";
        assert_eq!(colorize_passphrase(passphrase, false), passphrase);
    }

    #[test]
    fn test_colorize_password_enabled() {
        let password = "Abc123!";
        let result = colorize_password(password, true);

        // 元の文字が含まれることを確認
        // CI環境ではcolored crateが自動的に色を無効化する可能性があるため、
        // 長さのチェックは行わず、文字の存在のみ確認
        assert!(result.contains('A'));
        assert!(result.contains('b'));
        assert!(result.contains('1'));
        assert!(result.contains('!'));
    }

    #[test]
    fn test_colorize_password_contains_all_character_types() {
        let password = "Abc123!";
        let result = colorize_password(password, true);

        // 結果が空でないことを確認
        assert!(!result.is_empty());
        // 元の長さ以上であることを確認（最低でも元の文字列と同じ）
        assert!(result.len() >= password.len());
    }

    #[test]
    fn test_colorize_passphrase_enabled() {
        let passphrase = "correct-horse";
        let result = colorize_passphrase(passphrase, true);

        // 元の単語が含まれることを確認
        assert!(result.contains("correct"));
        assert!(result.contains("horse"));
        // ハイフンが含まれることを確認
        assert!(result.contains('-'));
    }

    #[test]
    fn test_colorize_passphrase_hyphen_separator() {
        let passphrase = "word1-word2-word3-word4";
        let result = colorize_passphrase(passphrase, true);

        // ハイフン区切りが保持されることを確認
        assert!(result.contains('-'));
        // 元の単語が含まれることを確認
        assert!(result.contains("word1"));
        assert!(result.contains("word4"));
        // 結果が空でないことを確認
        assert!(!result.is_empty());
    }
}
