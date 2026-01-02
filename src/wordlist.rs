/// EFF Wordlistから厳選した単語リスト
/// 各単語は4-8文字、覚えやすく発音しやすいもの
/// 現在は簡易版（256語）、後で2048語に拡張予定
pub const WORDLIST: &[&str] = &[
    "able", "acid", "aged", "also", "area", "army", "away", "baby",
    "back", "ball", "band", "bank", "base", "bath", "bear", "beat",
    "been", "beer", "bell", "belt", "bent", "best", "bird", "bite",
    "blow", "blue", "boat", "body", "boil", "bold", "bolt", "bomb",
    "bond", "bone", "book", "boom", "boot", "born", "boss", "both",
    "bowl", "bulk", "bull", "burn", "burst", "bush", "busy", "cafe",
    "cage", "cake", "call", "calm", "came", "camp", "card", "care",
    "case", "cash", "cast", "cave", "cell", "chat", "chef", "chip",
    "city", "clay", "clean", "clear", "clip", "clock", "close", "cloud",
    "club", "clue", "coal", "coat", "code", "coin", "cold", "come",
    "cook", "cool", "cope", "copy", "core", "corn", "cost", "cosy",
    "crab", "crew", "crop", "crow", "cube", "cute", "dame", "damp",
    "dare", "dark", "dash", "data", "date", "dawn", "days", "dead",
    "deaf", "deal", "dean", "dear", "debt", "deck", "deed", "deep",
    "deer", "deny", "desk", "dial", "diet", "dime", "dine", "dirt",
    "disc", "dish", "dive", "dock", "does", "doll", "dome", "done",
    "door", "dose", "down", "drag", "draw", "drew", "drop", "drug",
    "drum", "dual", "duck", "dull", "dumb", "dump", "dune", "dunk",
    "dusk", "dust", "duty", "each", "earl", "earn", "ease", "east",
    "easy", "echo", "edge", "edit", "else", "emit", "epic", "even",
    "ever", "evil", "exam", "exit", "face", "fact", "fade", "fail",
    "fair", "fake", "fall", "fame", "fare", "farm", "fast", "fate",
    "fear", "feat", "feed", "feel", "feet", "fell", "felt", "file",
    "fill", "film", "find", "fine", "fire", "firm", "fish", "fist",
    "five", "flag", "flat", "fled", "flee", "flew", "flip", "flow",
    "folk", "fond", "font", "food", "fool", "foot", "ford", "fork",
    "form", "fort", "foul", "four", "fowl", "free", "from", "fuel",
    "full", "fund", "funk", "fury", "fuse", "gain", "gale", "game",
    "gang", "gate", "gave", "gear", "gene", "gift", "girl", "give",
    "glad", "glow", "glue", "goal", "goat", "goes", "gold", "golf",
    "gone", "good", "grab", "grad", "gray", "grew", "grey", "grid",
    "grim", "grin", "grip", "grow", "gulf", "guru", "half", "hall",
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_wordlist_not_empty() {
        assert!(!WORDLIST.is_empty());
    }

    #[test]
    fn test_wordlist_uniqueness() {
        let unique: HashSet<_> = WORDLIST.iter().collect();
        assert_eq!(unique.len(), WORDLIST.len(), "Wordlist contains duplicates");
    }

    #[test]
    fn test_wordlist_word_length() {
        for word in WORDLIST.iter() {
            assert!(word.len() >= 3 && word.len() <= 8, "Word '{}' has invalid length", word);
        }
    }
}
