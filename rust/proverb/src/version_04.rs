use std::iter::once;

fn _format_body(pair: &[&str]) -> String {
    format!("For want of a {} the {} was lost.", pair[0], pair[1])
}

fn _format_tail(object: &str) -> String {
    format!("And all for the want of a {}.", object)
}

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        1 => _format_tail(list[0]),
        _ => list
            .windows(2)
            .map(_format_body)
            .chain(once(_format_tail(list[0])))
            .collect::<Vec<String>>()
            .join("\n"),
    }
}
