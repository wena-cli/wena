use wena::input::Inline;
use wena::output::Buffer;
use wena::Application;

pub fn assert_output(app: Application<Inline, Buffer>, expected: &str) {
    assert!(app.output.contents.contains(expected));
}
