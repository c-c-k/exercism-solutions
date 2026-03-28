use std::iter::once;

fn _format_body(pair: &[&str]) -> String {
    format!("For want of a {} the {} was lost.", pair[0], pair[1])
}

fn _format_tail(object: &&str) -> String {
    format!("And all for the want of a {}.", object)
}

pub fn build_proverb(list: &[&str]) -> String {
    // if list.is_empty() {
    //     return String::new();
    // }
    list.windows(2)
        .map(_format_body)
        .chain(once(list.first().map_or(String::new(), _format_tail)))
        .collect::<Vec<String>>()
        .join("\n")
}
