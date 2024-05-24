use serde::{Deserialize, Serialize};

// TODO?: handle name collisions
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[crate::export(object, js_name = "NoteLikeForGetNoteSummary")]
pub struct NoteLike {
    pub file_ids: Vec<String>,
    pub text: Option<String>,
    pub cw: Option<String>,
    pub has_poll: bool,
}

#[crate::export]
pub fn get_note_summary(note: NoteLike) -> String {
    let mut buf: Vec<String> = vec![];

    if let Some(cw) = note.cw {
        buf.push(cw)
    } else if let Some(text) = note.text {
        buf.push(text)
    }

    match note.file_ids.len() {
        0 => (),
        1 => buf.push("📎".to_string()),
        n => buf.push(format!("📎 ({})", n)),
    };

    if note.has_poll {
        buf.push("📊".to_string())
    }

    buf.join(" ")
}

#[cfg(test)]
mod unit_test {
    use super::{get_note_summary, NoteLike};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_note_summary() {
        let note = NoteLike {
            file_ids: vec![],
            text: Some("Hello world!".to_string()),
            cw: None,
            has_poll: false,
        };
        assert_eq!(get_note_summary(note), "Hello world!");

        let note_with_cw = NoteLike {
            file_ids: vec![],
            text: Some("Hello world!".to_string()),
            cw: Some("Content warning".to_string()),
            has_poll: false,
        };
        assert_eq!(get_note_summary(note_with_cw), "Content warning");

        let note_with_file_and_cw = NoteLike {
            file_ids: vec!["9s7fmcqogiq4igin".to_string()],
            text: None,
            cw: Some("Selfie, no ec".to_string()),
            has_poll: false,
        };
        assert_eq!(get_note_summary(note_with_file_and_cw), "Selfie, no ec 📎");

        let note_with_files_only = NoteLike {
            file_ids: vec![
                "9s7fmcqogiq4igin".to_string(),
                "9s7qrld5u14cey98".to_string(),
                "9s7gebs5zgts4kca".to_string(),
                "9s5z3e4vefqd29ee".to_string(),
            ],
            text: None,
            cw: None,
            has_poll: false,
        };
        assert_eq!(get_note_summary(note_with_files_only), "📎 (4)");

        let note_all = NoteLike {
            file_ids: vec![
                "9s7fmcqogiq4igin".to_string(),
                "9s7qrld5u14cey98".to_string(),
                "9s7gebs5zgts4kca".to_string(),
                "9s5z3e4vefqd29ee".to_string(),
            ],
            text: Some("Hello world!".to_string()),
            cw: Some("Content warning".to_string()),
            has_poll: true,
        };
        assert_eq!(get_note_summary(note_all), "Content warning 📎 (4) 📊");
    }
}
