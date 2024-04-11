use once_cell::sync::Lazy;
use regex::{Captures, Regex};

#[cfg_attr(feature = "napi", crate::export)]
pub fn nyaify(text: &str, lang: Option<&str>) -> String {
    let mut to_return = text.to_owned();

    {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?i-u)(non)([bcdfghjklmnpqrstvwxyz])").unwrap());
        to_return = RE
            .replace_all(&to_return, |caps: &Captures<'_>| {
                format!(
                    "{}{}",
                    match &caps[1] {
                        "non" => "nyan",
                        "Non" => "Nyan",
                        "NON" => "NYAN",
                        _ => &caps[1],
                    },
                    &caps[2]
                )
            })
            .to_string();
    }

    {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"다([.．。…?？!！\s]|$)").unwrap());
        to_return = RE.replace_all(&to_return, r"다냥$1").to_string();
    }

    {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"야([?？\s]|$)").unwrap());
        to_return = RE.replace_all(&to_return, r"냥$1").to_string();
    }

    {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([나-낳])").unwrap());
        to_return = RE
            .replace_all(&to_return, |caps: &Captures<'_>| {
                format!(
                    "{}",
                    char::from_u32(
                        caps[0].chars().next().unwrap() as u32 + 56 /* = '냐' - '나' */
                    )
                    .unwrap()
                )
            })
            .to_string();
    }

    if lang.is_some() && lang.unwrap().starts_with("zh") {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[妙庙描渺瞄秒苗藐廟]").unwrap());
        to_return = RE.replace_all(&to_return, "喵").to_string();
    }

    let simple_rules = [
        ("な", "にゃ"),
        ("ナ", "ニャ"),
        ("ﾅ", "ﾆｬ"),
        ("na", "nya"),
        ("NA", "NYA"),
        ("Na", "Nya"),
        ("morning", "mornyan"),
        ("Morning", "Mornyan"),
        ("MORNING", "MORNYAN"),
        ("everyone", "everynyan"),
        ("Everyone", "Everynyan"),
        ("EVERYONE", "EVERYNYAN"),
        ("να", "νια"),
        ("ΝΑ", "ΝΙΑ"),
        ("Να", "Νια"),
    ];

    simple_rules.into_iter().for_each(|(from, to)| {
        to_return = to_return.replace(from, to);
    });

    to_return
}

#[cfg(test)]
mod unit_test {
    use super::nyaify;

    #[test]
    fn can_nyaify() {
        assert_eq!(nyaify("Hello everyone!", Some("en")), "Hello everynyan!");
        assert_eq!(nyaify("Nonbinary people", None), "Nyanbinyary people");
        assert_eq!(nyaify("1分鐘是60秒", Some("zh-TW")), "1分鐘是60喵");
        assert_eq!(nyaify("1分間は60秒です", Some("ja-JP")), "1分間は60秒です");
        assert_eq!(nyaify("あなたは誰ですか", None), "あにゃたは誰ですか");
        assert_eq!(nyaify("Ναυτικός", Some("el-GR")), "Νιαυτικός");
        assert_eq!(nyaify("일어나다", None), "일어냐다냥");
    }
}
