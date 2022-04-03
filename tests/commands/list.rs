use wena::output::*;

#[test]
fn it_runs() {
    let app = wena::new("my-application", "0.0.1");
    let mut output = buffer::new();

    app.run_with_arguments(&mut output, vec![]);

    assert_eq!(output.contents, "Listing commands.\n");
}
