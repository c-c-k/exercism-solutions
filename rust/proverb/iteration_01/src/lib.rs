fn _format_body(object: &str, subject: &str) -> String {
    format!("For want of a {} the {} was lost.", object, subject)
}

fn _format_tail(object: &str) -> String {
    format!("And all for the want of a {}.", object)
}

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut proverb_lines: Vec<String> = list
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, item)| _format_body(list[index - 1], item))
        .collect();
    proverb_lines.push(_format_tail(list[0]));

    proverb_lines.join("\n")
}
