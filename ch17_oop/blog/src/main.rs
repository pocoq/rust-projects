use blog_lib::Post;

fn main(){
	let mut post = Post::new();
	post.add_text("I ate a salad for lunch today");
	println!("Draft: {}", post.get_content());

	post.request_review();
	println!("Request review: {}", post.get_content());

	post.approve();
	println!("Published: {}", post.get_content());
}