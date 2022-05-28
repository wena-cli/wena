use wena::output::Buffer;
use wena::*;

#[test]
fn test_error() {
    let mut output = Buffer::default();

    output.writeln(Alert::error("test"));

    assert_eq!(output.contents, "\n   ERROR  test\n\n");
}

#[test]
fn test_info() {
    let mut output = Buffer::default();

    output.writeln(Alert::info("test"));

    assert_eq!(output.contents, "\n   INFO  test\n\n");
}

#[test]
fn test_warn() {
    let mut output = Buffer::default();

    output.writeln(Alert::warn("test"));

    assert_eq!(output.contents, "\n   WARN  test\n\n");
}
