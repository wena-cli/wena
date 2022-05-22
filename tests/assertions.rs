use wena::application::Application;
use wena::output::buffer::Buffer;
use wena::Inline;

pub fn assert_output(app: Application<Inline, Buffer>, expected: &str) {
    assert_eq!(true, app.output.contents.contains(expected));
}
