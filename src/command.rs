pub struct Command<'a> {
    build: &'a str,
    run: Option<&'a str>
}