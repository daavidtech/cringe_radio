use regex::Regex;
use std::collections::HashMap;

pub fn parse_songs_from_string(input: &str) -> (Vec<String>, String) {
    let re = Regex::new(r"<SONG>(.*?)</SONG>").unwrap();
    let songs: Vec<String> = re
        .captures_iter(input)
        .map(|cap| cap[1].to_string())
        .collect();
    let modified_text = re.replace_all(input, "$1").to_string();
    (songs, modified_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_songs_from_string() {
        let input = "Here's a song that might help you feel less lonely:\n\n<SONG>Maroon 5 - Memories</SONG>\n\nCongratulations on coming out! Here are a few songs that celebrate love and LGBTQ+ pride:\n\n<SONG>Madonna - Vogue</SONG>\n<SONG>Lady Gaga - Born This Way</SONG>\n<SONG>Frank Ocean - Thinkin Bout You</SONG>";
        let (songs, modified_text) = parse_songs_from_string(input);

        assert_eq!(songs, vec![
            "Maroon 5 - Memories",
            "Madonna - Vogue",
            "Lady Gaga - Born This Way",
            "Frank Ocean - Thinkin Bout You",
        ]);

        let expected_modified_text = "Here's a song that might help you feel less lonely:\n\nMaroon 5 - Memories\n\nCongratulations on coming out! Here are a few songs that celebrate love and LGBTQ+ pride:\n\nMadonna - Vogue\nLady Gaga - Born This Way\nFrank Ocean - Thinkin Bout You";
        assert_eq!(modified_text, expected_modified_text);
    }
}
