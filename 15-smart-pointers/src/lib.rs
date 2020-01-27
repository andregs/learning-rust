pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T>
    where T: Messenger
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            max,
            value: 0,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let usage = self.value as f64 / self.max as f64;
        if usage > 1.0 {
            let msg = format!("Error: you are over your quota! {}/{}", self.value, self.max);
            self.messenger.send(&msg);
        } else if usage >= 0.9 {
            let msg = format!("Urgent warning: You've used up over 90% of your quota! {}/{}", self.value, self.max);
            self.messenger.send(&msg);
        } else if usage >= 0.75 {
            let msg = format!("Warning: You've used up over 75% of your quota! {}/{}", self.value, self.max);
            self.messenger.send(&msg);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
        // sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger { sent_messages: vec![] }
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // we would get compilation error here because &self is immutable
            // self.sent_messages.push(String::from(msg));

            // but the RefCell wrapper move that compile-time check to runtime
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn when_over_75_percent_should_warn() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
