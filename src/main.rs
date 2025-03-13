mod image_loader;
mod image_saver;
mod image_editor;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FileChooserDialog, FileChooserAction, ResponseType, 
    Image as GtkImage, Box as GtkBox, Orientation, ComboBoxText, Scale, Label, Entry, Adjustment};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};
use image_loader::load_image;
use image_saver::save_image;
use image_editor::{apply_filter, apply_brightness, apply_contrast, rotate_image, crop_image};

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
        .default_width(1000)
        .default_height(800)
        .build();

    let main_box = GtkBox::new(Orientation::Horizontal, 10);
    let controls_box = GtkBox::new(Orientation::Vertical, 5);
    let image_box = GtkBox::new(Orientation::Vertical, 5);

    // UI Elements
    let open_button = Button::with_label("Open Image");
    let save_button = Button::with_label("Save Image");
    let undo_button = Button::with_label("Undo");
    let redo_button = Button::with_label("Redo");
    let apply_filter_button = Button::with_label("Apply Filter");
    let rotate_button = Button::with_label("Rotate 90°");
    let crop_button = Button::with_label("Crop");

    let img_widget = GtkImage::new();
    img_widget.set_size_request(700, 500);

    // Image state management
    let image_state = Rc::new(RefCell::new(None));
    let history = Rc::new(RefCell::new(Vec::new()));
    let redo_stack = Rc::new(RefCell::new(Vec::new()));

    // Filter controls
    let filter_dropdown = ComboBoxText::new();
    filter_dropdown.append(Some("grayscale"), "Grayscale");
    filter_dropdown.append(Some("sepia"), "Sepia");
    filter_dropdown.append(Some("invert"), "Invert Colors");
    filter_dropdown.append(Some("blur"), "Blur");
    filter_dropdown.append(Some("sharpen"), "Sharpen");
    filter_dropdown.set_active_id(Some("grayscale"));

    // Brightness and Contrast
    let brightness_label = Label::new(Some("Brightness"));
    let brightness_slider = Scale::with_range(Orientation::Horizontal, -100.0, 100.0, 1.0);
    brightness_slider.set_value(0.0);
    brightness_slider.set_hexpand(true);

    let contrast_label = Label::new(Some("Contrast"));
    let contrast_slider = Scale::with_range(Orientation::Horizontal, -100.0, 100.0, 1.0);
    contrast_slider.set_value(0.0);
    contrast_slider.set_hexpand(true);

    // Crop controls
    let crop_box = GtkBox::new(Orientation::Vertical, 5);
    let x_label = Label::new(Some("X:"));
    let x_entry = Entry::new();
    let y_label = Label::new(Some("Y:"));
    let y_entry = Entry::new();
    let width_label = Label::new(Some("Width:"));
    let width_entry = Entry::new();
    let height_label = Label::new(Some("Height:"));
    let height_entry = Entry::new();

    // Status label
    let status_label = Label::new(Some("Ready"));

    // Open Image
    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let history_clone = history.clone();
    let window_clone = window.clone();
    open_button.connect_clicked(move |_| {
        let dialog = FileChooserDialog::new(Some("Open Image"), Some(&window_clone), FileChooserAction::Open);
        dialog.add_buttons(&[("Open", ResponseType::Accept), ("Cancel", ResponseType::Cancel)]);

        let img_widget_clone = img_widget_clone.clone();
        let image_state_clone = image_state_clone.clone();
        let history_clone = history_clone.clone();

        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                    let path_str = file_path.to_string_lossy().to_string();
                    match load_image(&path_str) {
                        Ok(img) => {
                            img_widget_clone.set_from_file(Some(&path_str));
                            *image_state_clone.borrow_mut() = Some(img.clone());
                            history_clone.borrow_mut().push(img);
                        }
                        Err(err) => eprintln!("Failed to load image: {}", err),
                    }
                }
            }
            dialog.close();
        });
        dialog.show();
    });

    // Apply Filter
    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let history_clone = history.clone();
    let filter_dropdown_clone = filter_dropdown.clone();
    apply_filter_button.connect_clicked(move |_| {
        let mut image_ref = image_state_clone.borrow_mut();
        if let Some(ref img) = *image_ref {
            if let Some(filter_type) = filter_dropdown_clone.active_id() {
                let edited_img = apply_filter(img, &filter_type.to_string());
                history_clone.borrow_mut().push(edited_img.clone());
                *image_ref = Some(edited_img.clone());
                save_temp_and_display(&img_widget_clone, &edited_img);
            }
        }
    });

    // Brightness
    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let history_clone = history.clone();
    brightness_slider.connect_value_changed(move |slider| {
        let mut image_ref = image_state_clone.borrow_mut();
        if let Some(ref img) = *image_ref {
            let brightness_value = slider.value() as i32;
            let adjusted_img = apply_brightness(img, brightness_value);
            history_clone.borrow_mut().push(adjusted_img.clone());
            *image_ref = Some(adjusted_img.clone());
            save_temp_and_display(&img_widget_clone, &adjusted_img);
        }
    });

    // Contrast
    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let history_clone = history.clone();
    contrast_slider.connect_value_changed(move |slider| {
        let mut image_ref = image_state_clone.borrow_mut();
        if let Some(ref img) = *image_ref {
            let contrast_value = slider.value() as i32;
            let adjusted_img = apply_contrast(img, contrast_value);
            history_clone.borrow_mut().push(adjusted_img.clone());
            *image_ref = Some(adjusted_img.clone());
            save_temp_and_display(&img_widget_clone, &adjusted_img);
        }
    });

    // Rotate
    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let history_clone = history.clone();
    rotate_button.connect_clicked(move |_| {
        let mut image_ref = image_state_clone.borrow_mut();
        if let Some(ref img) = *image_ref {
            let rotated_img = rotate_image(img);
            history_clone.borrow_mut().push(rotated_img.clone());
            *image_ref = Some(rotated_img.clone());
            save_temp_and_display(&img_widget_clone, &rotated_img);
        }
    });

    // Crop
    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let history_clone = history.clone();
    let x_entry_clone = x_entry.clone();
    let y_entry_clone = y_entry.clone();
    let width_entry_clone = width_entry.clone();
    let height_entry_clone = height_entry.clone();
    crop_button.connect_clicked(move |_| {
        let mut image_ref = image_state_clone.borrow_mut();
        if let Some(ref img) = *image_ref {
            let x = x_entry_clone.text().parse::<u32>().unwrap_or(0);
            let y = y_entry_clone.text().parse::<u32>().unwrap_or(0);
            let width = width_entry_clone.text().parse::<u32>().unwrap_or(img.width());
            let height = height_entry_clone.text().parse::<u32>().unwrap_or(img.height());
            let cropped_img = crop_image(img, x, y, width, height);
            history_clone.borrow_mut().push(cropped_img.clone());
            *image_ref = Some(cropped_img.clone());
            save_temp_and_display(&img_widget_clone, &cropped_img);
        }
    });

    // Undo
    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let history_clone = history.clone();
    let redo_stack_clone = redo_stack.clone();
    undo_button.connect_clicked(move |_| {
        let mut history = history_clone.borrow_mut();
        let mut image_ref = image_state_clone.borrow_mut();
        if history.len() > 1 {
            if let Some(current) = image_ref.take() {
                redo_stack_clone.borrow_mut().push(current);
                history.pop();
                if let Some(prev_img) = history.last() {
                    *image_ref = Some(prev_img.clone());
                    save_temp_and_display(&img_widget_clone, prev_img);
                }
            }
        }
    });

    // Redo
    let img_widget_clone = img_widget.clone();
    let image_state_clone = image_state.clone();
    let history_clone = history.clone();
    let redo_stack_clone = redo_stack.clone();
    redo_button.connect_clicked(move |_| {
        let mut redo_stack = redo_stack_clone.borrow_mut();
        let mut image_ref = image_state_clone.borrow_mut();
        if let Some(next_img) = redo_stack.pop() {
            history_clone.borrow_mut().push(next_img.clone());
            *image_ref = Some(next_img.clone());
            save_temp_and_display(&img_widget_clone, &next_img);
        }
    });

    // Save Image
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

    // Layout setup
    controls_box.append(&open_button);
    controls_box.append(&save_button);
    controls_box.append(&undo_button);
    controls_box.append(&redo_button);
    controls_box.append(&filter_dropdown);
    controls_box.append(&apply_filter_button);
    controls_box.append(&brightness_label);
    controls_box.append(&brightness_slider);
    controls_box.append(&contrast_label);
    controls_box.append(&contrast_slider);
    controls_box.append(&rotate_button);
    
    crop_box.append(&x_label);
    crop_box.append(&x_entry);
    crop_box.append(&y_label);
    crop_box.append(&y_entry);
    crop_box.append(&width_label);
    crop_box.append(&width_entry);
    crop_box.append(&height_label);
    crop_box.append(&height_entry);
    crop_box.append(&crop_button);
    controls_box.append(&crop_box);
    
    controls_box.append(&status_label);
    
    image_box.append(&img_widget);
    
    main_box.append(&controls_box);
    main_box.append(&image_box);
    
    window.set_child(Some(&main_box));
    window.show();
}

fn save_temp_and_display(img_widget: &GtkImage, img: &image::DynamicImage) {
    let temp_path = format!("temp_{}.png", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    if save_image(img, &temp_path).is_ok() {
        img_widget.set_from_file(Some(&temp_path));
    }
}