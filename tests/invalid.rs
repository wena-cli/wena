mod fixtures;

#[test]
fn it_runs() {
    let app = fixtures::app(Vec::new());

    app.run_with_arguments(
        vec!["my-application".to_string(), "hello".to_string()],
    );

    assert_eq!(app.output.contents, "Command not found.\n");
}
