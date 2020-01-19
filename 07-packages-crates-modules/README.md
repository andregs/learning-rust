https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

"cargo new" creates a new package.
a package contains the Cargo.toml file.
src/main.rs is the root of a binary crate.
src/lib.rs is the root of a library crate.
you can have one lib crate at max in a package.
each file in src/bin is a separate binary crate.

"cargo new --lib restaurant"
