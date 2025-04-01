mod editor;
mod terminal;
mod types;
use editor::Editor;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
