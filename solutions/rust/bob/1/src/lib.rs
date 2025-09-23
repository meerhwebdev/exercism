enum MessageType {
    Question,
    Yelling,
    YellingQuestion,
    Silence,
    Whatever
}

pub fn reply(message: &str) -> &str {
    match bod_reply(message.trim()) {
        MessageType::Question => "Sure.",
        MessageType::Yelling => "Whoa, chill out!",
        MessageType::YellingQuestion=> "Calm down, I know what I'm doing!",
        MessageType::Silence=> "Fine. Be that way!",
        MessageType::Whatever => "Whatever."
    }
}

fn bod_reply(message:&str) -> MessageType {
    if message.is_empty() { 
        MessageType::Silence
    }
    else if message.to_uppercase() == message && !is_non_alphabetic_only(message) {
        if message.ends_with("?") {
            MessageType::YellingQuestion
        }else {
            MessageType::Yelling
        }
    }
    else if message.ends_with("?")  {
        MessageType::Question
    } 
    else {
        MessageType::Whatever
    }
}

fn is_non_alphabetic_only(s: &str) -> bool {
    let trimmed = s.strip_suffix('?').unwrap_or(s);
    !trimmed.is_empty() && trimmed.chars().all(|c| !c.is_alphabetic())
}