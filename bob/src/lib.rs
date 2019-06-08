pub fn reply(message: &str) -> &str {
  let new_message = message.trim();
  let make_sense = new_message.chars().any(|c| c.is_alphabetic());
  let question = new_message.ends_with('?');
  let yell = new_message == new_message.to_uppercase();

  match new_message 
  {
    "" => "Fine. Be that way!",
    _ if make_sense && question && yell => "Calm down, I know what I'm doing!",
    _ if make_sense && yell => "Whoa, chill out!",
    _ if question => "Sure.",
    _ => "Whatever."
  }
}
