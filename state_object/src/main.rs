use state_object::Post;

#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
    let mut new_post = Post::new();
    new_post.add_text("new post should be drsfted");
}
