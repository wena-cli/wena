mod fixtures;

#[test]
fn it_runs() {
    let (app, output) = fixtures::app(Vec::new());

    app.run_with_arguments(vec![]);

    assert_eq!(app.output.contents, "\n  my-application: 0.0.1\n\n");
}
