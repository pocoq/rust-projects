// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, 
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes 
// “apple-hay”). Keep in mind the details about UTF-8 encoding!
// ref: https://codereview.stackexchange.com/questions/175906/convert-string-to-pig-latin-in-rust

use itertools::Itertools;

fn pigify_one(word: &str) -> String{
	let mut chars = word.chars();

	let first = match chars.next(){
		Some(c) => c,
		None => return String::new()
	};

	match first {
		'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
		_ => format!("{}-{}ay", chars.as_str(), first)
	}
}

// fn folder(mut current: String, next: String) -> String {
//     if !current.is_empty() {
//         current.push(' ');
//     }
//     current.push_str(&next);
//     current
// }

pub fn pigify(text: &str) -> String{
	text.split_whitespace()
	.map(pigify_one)
	// .fold(String::new(), folder)
	.join(" ")
}