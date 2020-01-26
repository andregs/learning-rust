use minigrep;
use minigrep::Config;

#[test]
fn search_one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive."],
        minigrep::search(query, contents),
    );
}

#[test]
fn isearch_two_results() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        minigrep::isearch(query, contents),
    );
}

#[test]
fn new_config_ok() {
    let args = ["/minigrep", "query", "contents"]
        .iter().map(|s| s.to_string());
    let cfg = Config::new(args).unwrap();
    assert_eq!(cfg.query, "query");
    assert_eq!(cfg.filename, "contents");
}

#[test]
#[should_panic(expected = "Expected: filename")]
fn new_config_fail() {
    let args = ["query", "contents"]
        .iter().map(|s| s.to_string());
    Config::new(args).unwrap();
    panic!("It should have panicked!");
}

// TODO how to test "run" function mocking out "search" function?
