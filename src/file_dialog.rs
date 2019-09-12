use gtk::*;
use gtk::{FileChooserDialog};
use std::path::PathBuf;

/// A wrapped FileChooserDialog that automatically destroys itself upon being dropped.
pub struct OpenDialog(FileChooserDialog);

/// A wrapped FileChooserDialog that automatically destroys itself upon being dropped.
pub struct SaveDialog(FileChooserDialog);

impl OpenDialog {
    pub fn new() -> OpenDialog {
        // Create a new file chooser dialog for opening a file.
        let open_dialog = FileChooserDialog::new(
            Some("Open"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
        );

        // Add the cancel and open buttons to that dialog.
        open_dialog.add_button("Cancel", ResponseType::Cancel.into());
        open_dialog.add_button("Open", ResponseType::Ok.into());

        OpenDialog(open_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl SaveDialog {
    pub fn new() -> SaveDialog {
        // Initializes a new save as dialog, which is embedded within a new popup window.
        let save_dialog = FileChooserDialog::new(
            Some("Save As"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
        );

        // Add the cancel and save buttons to that dialog.
        save_dialog.add_button("Cancel", ResponseType::Cancel.into());
        save_dialog.add_button("Save", ResponseType::Ok.into());

        SaveDialog(save_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl Drop for OpenDialog {
    fn drop(&mut self) { self.0.destroy(); }
}

impl Drop for SaveDialog {
    fn drop(&mut self) { self.0.destroy(); }
}