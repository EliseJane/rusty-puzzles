#[test]
fn greet() {
  #[derive(Debug)]
  struct Config { greeting: Option<String> };
  let config = &Config { greeting: Some("hello".to_string()) }; // but could be None!
  let greeting: &str;

  // How to convert from Option<String> into &str (defaulting to "hi" if None, and avoid allocations, if possible)?
  // Can only change the next line:
  greeting = config.greeting.as_ref().map(|s| s.as_str()).unwrap_or("hi");

  assert_eq!(greeting, "hello");
}
