pub mod greetings
{
    pub mod english;
    pub mod french
    {
        pub fn hello() -> String {"bonjour".to_string()}
        pub fn goodbye() -> String {"au voir".to_string()}
    }
}

#[test] // tell rust this is a part of unit test in rust
fn english_greeting_correct()
{
  assert_eq!("hello", greetings::english::hello());
}