// TODO https://doc.rust-lang.org/book/ch13-02-iterators.html

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_closure.value(intensity),
        );
        println!(
            "Next, do {} situps",
            expensive_closure.value(intensity),
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated.");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity),
            );
        }
    }
}

struct Cacher<F, V> where
    F: Fn(V) -> V,
    V: Hash + Eq + Copy + Debug
{
    calculation: F,
    value: HashMap<V, V>,
}

impl<F, V> Cacher<F, V> where
    F: Fn(V) -> V,
    V: Hash + Eq + Copy + Debug
{
    fn new(calculation: F) -> Cacher<F, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: V) -> V {
        if !self.value.contains_key(&arg) {
            self.value.insert(arg, (self.calculation)(arg));
        }
        
        self.value[&arg]
    }
}
