mod image_loader;
mod image_saver;
mod image_editor;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FileChooserDialog, FileChooserAction, ResponseType, Image as GtkImage, Box as GtkBox, Orientation, ComboBoxText, Scale};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};
use image_loader::load_image;
use image_saver::save_image;
use image_editor::{apply_filter, apply_brightness, apply_contrast};

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
        .default_width(800)
        .default_height(600)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    let open_button = Button::with_label("Open Image");
    let apply_filter_button = Button::with_label("Apply Filter");
    let save_button = Button::with_label("Save Image");
    let img_widget = GtkImage::new();
    img_widget.set_size_request(600, 400);

    let image_state = Rc::new(RefCell::new(None));

    let filter_dropdown = ComboBoxText::new();
    filter_dropdown.append(Some("grayscale"), "Grayscale");
    filter_dropdown.append(Some("sepia"), "Sepia");
    filter_dropdown.append(Some("invert"), "Invert Colors");
    filter_dropdown.append(Some("contrast"), "Increase Contrast");
    filter_dropdown.set_active_id(Some("grayscale"));

    let brightness_slider = Scale::with_range(gtk::Orientation::Horizontal, -100.0, 100.0, 1.0);
    brightness_slider.set_value(0.0);
    brightness_slider.set_hexpand(true);

    let contrast_slider = Scale::with_range(gtk::Orientation::Horizontal, -100.0, 100.0, 1.0);
    contrast_slider.set_value(0.0);
    contrast_slider.set_hexpand(true);

    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let window_clone = window.clone();

    open_button.connect_clicked(move |_| {
        let dialog = FileChooserDialog::new(Some("Open Image"), Some(&window_clone), FileChooserAction::Open);
        dialog.add_buttons(&[("Open", ResponseType::Accept), ("Cancel", ResponseType::Cancel)]);

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

    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let filter_dropdown_clone = filter_dropdown.clone();

    apply_filter_button.connect_clicked(move |_| {
        let mut image_ref = image_state_clone.borrow_mut();
        if let Some(ref img) = *image_ref {
            if let Some(filter_type) = filter_dropdown_clone.active_id() {
                let edited_img = apply_filter(img, &filter_type.to_string());
                *image_ref = Some(edited_img.clone());

                let temp_path = format!("temp_{}.png", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                if save_image(&edited_img, &temp_path).is_ok() {
                    img_widget_clone.set_from_file(Some(&temp_path));
                }
            }
        }
    });

    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();

    brightness_slider.connect_value_changed(move |slider| {
        let mut image_ref = image_state_clone.borrow_mut();
        if let Some(ref img) = *image_ref {
            let brightness_value = slider.value() as i32;
            let adjusted_img = apply_brightness(img, brightness_value);
            *image_ref = Some(adjusted_img.clone());

            let temp_path = format!("temp_{}.png", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
            if save_image(&adjusted_img, &temp_path).is_ok() {
                img_widget_clone.set_from_file(Some(&temp_path));
            }
        }
    });

    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();

    contrast_slider.connect_value_changed(move |slider| {
        let mut image_ref = image_state_clone.borrow_mut();
        if let Some(ref img) = *image_ref {
            let contrast_value = slider.value() as i32;
            let adjusted_img = apply_contrast(img, contrast_value);
            *image_ref = Some(adjusted_img.clone());

            let temp_path = format!("temp_{}.png", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
            if save_image(&adjusted_img, &temp_path).is_ok() {
                img_widget_clone.set_from_file(Some(&temp_path));
            }
        }
    });

    let image_state_clone = image_state.clone();
    let window_clone = window.clone();
    save_button.connect_clicked(move |_| {
        let save_dialog = FileChooserDialog::new(Some("Save Image"), Some(&window_clone), FileChooserAction::Save);
        save_dialog.add_buttons(&[("Save", ResponseType::Accept), ("Cancel", ResponseType::Cancel)]);

        let image_state_clone = image_state_clone.clone();
        save_dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                    let path_str = file_path.to_string_lossy().to_string();
                    if let Some(ref img) = *image_state_clone.borrow() {
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

    vbox.append(&open_button);
    vbox.append(&img_widget);
    vbox.append(&filter_dropdown);
    vbox.append(&apply_filter_button);
    vbox.append(&brightness_slider);
    vbox.append(&contrast_slider);
    vbox.append(&save_button);
    window.set_child(Some(&vbox));
    window.show();
}