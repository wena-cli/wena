use wena::Terminal;

#[test]
fn test_width() {
    let terminal = Terminal::default();

    assert!(terminal.width() > 0);
}
