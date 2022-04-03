use wena::output::*;

#[test]
fn it_runs() {
    let app = wena::new("my-application", "0.0.1");
    let mut output = buffer::new();

    app.run_with_arguments(
        &mut output,
        vec!["my-application".to_string(), "hello".to_string()],
    );

    assert_eq!(output.contents, "Command not found.\n");
}
