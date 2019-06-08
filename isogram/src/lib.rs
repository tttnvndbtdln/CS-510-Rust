use std::collections::HashSet;

pub fn check(word: &str) -> bool {
    let mut repeat: HashSet<char> = HashSet::new();
    for element in word.to_lowercase().chars() {
        if !element.is_alphanumeric()
	{ 
	  continue 
	}
        if repeat.contains(&element) 
	{ 
	  return false 
	}
        repeat.insert(element);
    }
    true
}
