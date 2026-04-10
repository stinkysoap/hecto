mod editor;
use editor::Editor;
use std::error::Error;
fn main() {
    let editor = Editor::default();
    editor.run();
}
