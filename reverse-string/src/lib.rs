use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::from("");
    let char_iter = input.graphemes(true);

    for ch in char_iter {
        reversed = format!("{}{}", ch, reversed);
    }

    return String::from(reversed)
}
