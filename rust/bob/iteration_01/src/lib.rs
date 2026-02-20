pub fn reply(message: &str) -> &str {
    let message = message.trim_end();
    let is_silence = message.is_empty();
    if is_silence {
        return "Fine. Be that way!";
    };

    let is_shout = message.find(|c: char| c.is_uppercase()).is_some()
        && message.find(|c: char| c.is_lowercase()).is_none();
    let is_question = message.ends_with("?");

    if is_shout && is_question {
        "Calm down, I know what I'm doing!"
    } else if is_question {
        "Sure."
    } else if is_shout {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
