#![warn(clippy::all, clippy::pedantic)]

mod editor;

fn main() {
    let mut editor = editor::Editor::default();
    editor.run();
}
