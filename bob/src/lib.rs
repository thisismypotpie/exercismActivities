pub fn reply(message: &str) -> &str {
    //unimplemented!("have Bob reply to the incoming message: {}", message)
    let mut has_letters = false;
    //The following line was found at: https://exercism.io/tracks/rust/exercises/bob/solutions/57b76f4c0ac440388fb8171156131aa8
    //DISCLAIMER:  While I did look at a solution for JUST THIS ONE LINE, the rest was
    // created by me and I have been working on this exercise for a long period of time.
    let mut peekable = message.chars().filter(|c| c.is_alphabetic()).peekable();

    if peekable.peek().is_some() {
        has_letters = true;
    }

    if message.trim().ends_with('?') {
        if message.to_uppercase() == message && has_letters{
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    } else if message.trim().is_empty() {
        return "Fine. Be that way!";
    } else if message.trim().to_uppercase() == message && has_letters{
        return "Whoa, chill out!";
    }
    "Whatever."
}
