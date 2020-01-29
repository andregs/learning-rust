pub trait Draw {
    fn draw(&self);
}

// Screen holds a vector of components made of instances of a 'trait object' Draw.
// that means Screen components can hold a Box<Button> and Box<TextField> at the same time.

pub struct Screen {
    // a vector of any type (inside a Box) that implements Draw
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// All components in a Screen2 must be of type Button or all of type TextField,
// otherwise the code does not compile.

pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}

#[derive(Debug)]
struct TextField {
    pub label: String,
    pub placeholder: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button {:?}", &self);
    }
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing text field {:?}", &self);
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // content of a post is whatever the current state says it is.
    // as_ref because we want a reference to the value inside the Option, i.e. Option<&Box<dyn State>>
    // we can unwrap without panic because we know Post methods always finish with Some value in self.state
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // this public 'request_review' calls a private 'request_review' from State,
    // that consumes the current State and returns a new one
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // state of a Post when approved is whatever the current state says it is when that state is approved
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_test() {
        // run with "cargo test -- --nocapture"

        let screen = Screen {
            components: vec![
                Box::new(TextField {
                    label: String::from("Username"),
                    placeholder: String::from("Enter a username"),
                }),
                Box::new(Button {
                    width: 50,
                    height: 20,
                    label: String::from("Submit"),
                }),
            ],
        };

        screen.run();
    }
    
    #[test]
    fn blog_test() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());
        
        post.request_review();
        assert_eq!("", post.content());
        
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
