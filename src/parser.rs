use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use serde_json::Value;


pub fn parse_song_names_from_string(input: &str) -> Result<Vec<String>, Error> {
    // Extract JSON string from the input
    let json_start = input.find("{").ok_or_else(|| anyhow!("Could not find JSON start"))?;
    let json_end = input.rfind("}").ok_or_else(|| anyhow!("Could not find JSON end"))?;
    let json_str = &input[json_start..=json_end];

    // Parse JSON into a Value
    let json_value: Value = serde_json::from_str(json_str)?;

    // Extract the song names
    let songs = json_value["songs"].as_array().ok_or_else(|| anyhow!("Could not find songs array"))?;
    let mut song_names = Vec::new();
    for song in songs {
        let song_name = song["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Could not find song name as string"))?
            .to_string();
        song_names.push(song_name);
    }

    Ok(song_names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let str = r#"
Sure here is song names:

```
{
    "songs":  [
        {
            "name":  "despacito"
        }
    ]
}
```"#;

        let song_names = parse_song_names_from_string(str).unwrap();

        assert_eq!(song_names.len(), 1);
        assert_eq!(song_names[0], "despacito");
    }

    #[test]
    fn test_parser2() {
        let str = r#"
Sure here is song names:

{
    "songs":  [
        {
            "name":  "despacito"
        }
    ]
}"#;

        let song_names = parse_song_names_from_string(str).unwrap();

        assert_eq!(song_names.len(), 1);
        assert_eq!(song_names[0], "despacito");
    }

    #[test]
    fn test_parser3() {
        let str = r#"
Sure thing! Here's Eminem's "Lose Yourself". Enjoy!

https://www.youtube.com/watch?v=_Yhyp-_hX2s
"#;
            
        let song_names = parse_song_names_from_string(str).unwrap();

        assert_eq!(song_names.len(), 1);
        assert_eq!(song_names[0], "https://www.youtube.com/watch?v=_Yhyp-_hX2s");
    }
}