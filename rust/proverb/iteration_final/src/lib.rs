use std::iter::once;

fn _format_body(object: &str, subject: &str) -> String {
    format!("For want of a {object} the {subject} was lost.")
}

fn _format_tail(object: &str) -> String {
    format!("And all for the want of a {}.", object)
}

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        1 => _format_tail(list[0]),
        _ => list
            .iter()
            .skip(1)
            .zip(list)
            .map(|(subject, object)| _format_body(object, subject))
            .chain(once(_format_tail(list[0])))
            .collect::<Vec<String>>()
            .join("\n"),
    }
}
