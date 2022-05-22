mod assertions;
mod fixtures;

use assertions::*;

#[test]
fn it_runs() {
    let app = fixtures::app(vec![], vec![]);

    assert_output(app, "my-application : 0.0.1");
}
