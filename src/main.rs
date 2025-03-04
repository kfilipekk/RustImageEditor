mod image_loader;
mod image_saver;
mod image_editor;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FileChooserDialog, FileChooserAction, ResponseType, Image as GtkImage, Box as GtkBox, Orientation, CssProvider};
use std::cell::RefCell;
use std::rc::Rc;
use image_loader::load_image;
use image_saver::save_image;
use image_editor::apply_filter;

fn main() {
    let app = Application::builder()
        .application_id("com.example.imageeditor")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rust Image Editor")
        .default_width(800)  // Increased width for a larger window
        .default_height(600) // Kept height the same
        .build();

    // Apply custom CSS styling
    let provider = CssProvider::new();
    // Removed the 'b' prefix and used a string literal instead
    provider.load_from_data("
        window {
            background-color: #f0f0f0;
        }
        button {
            background-color: #4CAF50;
            color: white;
            border-radius: 5px;
            padding: 10px;
        }
        button:hover {
            background-color: #45a049;
        }
        image {
            border: 2px solid #ddd;
            border-radius: 5px;
        }
    ");

    // Apply the CSS provider directly to the window's style context
    gtk::StyleContext::add_provider(&window.style_context(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    let open_button = Button::with_label("Open Image");
    let apply_filter_button = Button::with_label("Apply Grayscale Filter");
    let save_button = Button::with_label("Save Image");

    let img_widget = GtkImage::new();
    img_widget.set_size_request(600, 400); // Increased size for the image preview

    let image_state = Rc::new(RefCell::new(None));

    // Open Image Button
    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let window_clone = window.clone();
    
    open_button.connect_clicked(move |_| {
        let dialog = FileChooserDialog::new(
            Some("Open Image"),
            Some(&window_clone),
            FileChooserAction::Open,
            &[("Open", ResponseType::Accept), ("Cancel", ResponseType::Cancel)],
        );

        let img_widget_clone = img_widget_clone.clone();
        let image_state_clone = image_state_clone.clone();

        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                    let path_str = file_path.to_string_lossy().to_string();

                    match load_image(&path_str) {
                        Ok(img) => {
                            img_widget_clone.set_from_file(Some(&path_str));
                            *image_state_clone.borrow_mut() = Some(img);
                        }
                        Err(err) => eprintln!("Failed to load image: {}", err),
                    }
                }
            }
            dialog.close();
        });

        dialog.show();
    });

    // Apply Grayscale Filter Button
    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    
    apply_filter_button.connect_clicked(move |_| {
        let mut image_ref = image_state_clone.borrow_mut();
        if let Some(img) = image_ref.take() {
            let edited_img = apply_filter(img);
            *image_ref = Some(edited_img.clone());

            let temp_path = "temp_output.png";
            if save_image(&edited_img, temp_path).is_ok() {
                img_widget_clone.set_from_file(Some(temp_path));
            }
        }
    });

    // Save Image Button
    let image_state_clone = image_state.clone();
    let window_clone = window.clone();

    save_button.connect_clicked(move |_| {
        let save_dialog = FileChooserDialog::new(
            Some("Save Image"),
            Some(&window_clone),
            FileChooserAction::Save,
            &[("Save", ResponseType::Accept), ("Cancel", ResponseType::Cancel)],
        );

        let image_state_clone = image_state_clone.clone();

        save_dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                    let path_str = file_path.to_string_lossy().to_string();

                    let image_ref = image_state_clone.borrow();
                    if let Some(img) = image_ref.as_ref() {
                        if let Err(err) = save_image(img, &path_str) {
                            eprintln!("Failed to save image: {}", err);
                        }
                    }
                }
            }
            dialog.close();
        });

        save_dialog.show();
    });

    // Layout
    vbox.append(&open_button);
    vbox.append(&img_widget);
    vbox.append(&apply_filter_button);
    vbox.append(&save_button);
    window.set_child(Some(&vbox));

    window.show();
}
