mod editor;
use editor::Editor;
use std::error::Error;
fn main() {
    Editor::default().run();
}
