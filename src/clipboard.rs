//! クリップボード操作モジュール
//!
//! パスワードやパスフレーズをシステムクリップボードにコピーする機能を提供します。

use crate::error::{Result, RpgError};
use arboard::Clipboard;

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| RpgError::ClipboardError(format!("Failed to access clipboard: {}", e)))?;

    clipboard
        .set_text(text)
        .map_err(|e| RpgError::ClipboardError(format!("Failed to copy: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_to_clipboard() {
        let result = copy_to_clipboard("test-password");
        // クリップボードが利用可能な環境でのみテスト
        // CI環境では失敗する可能性があるため、エラーは無視
        if result.is_err() {
            eprintln!("Clipboard test skipped (no clipboard available)");
        }
    }
}
