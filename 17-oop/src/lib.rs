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
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // moves self to the new instance, so we cannot keep using draft after review is requested
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {

    // moves self to the new instance, like request_review method did
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
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
        // assert_eq!("", post.content()); // content method does not exist in DraftPost
        
        let post = post.request_review();
        // post = post.request_review(); // does not compile because previous var is DraftPost
        // assert_eq!("", post.content()); // content method does not exist in PendingReviewPost
        
        let post = post.approve();
        // post = post.approve(); // does not compile because previous var is PendingReviewPost
        
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
