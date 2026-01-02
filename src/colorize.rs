use colored::*;

/// 文字種ごとに色分けしてパスワードを表示
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
    let colors = [Color::Cyan, Color::Magenta, Color::Yellow, Color::Green];

    words
        .iter()
        .enumerate()
        .map(|(i, word)| {
            let color = colors[i % colors.len()];
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
        // カラーコードが含まれるため、元の文字列より長くなる
        assert!(result.len() > password.len());
    }

    #[test]
    fn test_colorize_passphrase_enabled() {
        let passphrase = "correct-horse";
        let result = colorize_passphrase(passphrase, true);
        // カラーコードが含まれるため、元の文字列より長くなる
        assert!(result.len() > passphrase.len());
    }
}
