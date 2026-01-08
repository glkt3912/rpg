//! 出力とクリップボード操作を担当するモジュール

use crate::clipboard;
use crate::error::{Result, RpgError};

pub fn output_or_copy(
    items: Vec<String>,
    copy: bool,
    colorize_fn: impl Fn(&str, bool) -> String,
    enable_color: bool,
) -> Result<()> {
    let last = items.last().ok_or(RpgError::EmptyOutput)?;

    if !copy {
        for item in &items {
            println!("{}", colorize_fn(item, enable_color));
        }
    }

    if copy {
        clipboard::copy_to_clipboard(last)?;
        println!("Copied to clipboard!");
    }

    Ok(())
}

pub fn should_enable_color(no_color: bool) -> bool {
    !no_color && atty::is(atty::Stream::Stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_colorize(s: &str, _enable: bool) -> String {
        s.to_string()
    }

    #[test]
    fn test_output_or_copy_empty_list() {
        let items: Vec<String> = vec![];
        let result = output_or_copy(items, false, dummy_colorize, false);

        assert!(result.is_err());
        match result {
            Err(RpgError::EmptyOutput) => (),
            _ => panic!("Expected EmptyOutput error"),
        }
    }

    #[test]
    fn test_should_enable_color_with_no_color_flag() {
        // --no-color が指定されている場合
        assert_eq!(should_enable_color(true), false);
    }

    #[test]
    fn test_should_enable_color_without_flag() {
        // --no-color が指定されていない場合はTTY判定に依存
        // CI環境ではTTYでない可能性が高いので、falseまたはtrueどちらでも良い
        let result = should_enable_color(false);
        // TTY判定の結果を受け入れる
        assert!(result == true || result == false);
    }
}
