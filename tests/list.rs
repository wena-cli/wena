mod fixtures;

#[test]
fn it_runs() {
    let app = fixtures::app(vec![], vec![]);

    assert_eq!(app.output.contents, "\n  my-application: 0.0.1\n\n");
}
