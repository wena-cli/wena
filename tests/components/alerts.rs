use wena::{BufferOutput, *};

#[test]
fn test_error() {
    let mut output = BufferOutput::default();

    output.writeln(Alert::error("test"));

    assert_eq!(output.contents, "\n   ERROR  test\n\n");
}

#[test]
fn test_info() {
    let mut output = BufferOutput::default();

    output.writeln(Alert::info("test"));

    assert_eq!(output.contents, "\n   INFO  test\n\n");
}

#[test]
fn test_warn() {
    let mut output = BufferOutput::default();

    output.writeln(Alert::warning("test"));

    assert_eq!(output.contents, "\n   WARN  test\n\n");
}
