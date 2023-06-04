slint::include_modules!();
use std::rc::Rc;
use slint::{VecModel, Model, ModelRc};
mod get_book_data;
fn main() {
    let ui = MainPage::new().unwrap();

    // Get the pre-existing list of books
    let books: Vec<BookData> = ui.get_books().iter().collect();
    let books_model = Rc::new(VecModel::from(books));

    let ui_weak = ui.as_weak(); // Get a weak handle to the UI so we can do stuff within the move.

    // Handle the add button click
    ui.on_add_book_clicked(move || {
        // For now I just create a sample book
        // TODO: Make add book page and create this data properly
        let sample_book: BookData = BookData {
            authors: ModelRc::default(),
            cover_image: slint::Image::load_from_path(std::path::Path::new("ui/book-cover.jpg")).unwrap(),
            description: "This is a description for the sample book".into(),
            owner: "Seb".into(),
            title: "Sample book".into()
        };

        books_model.push(sample_book);
        ui_weak.unwrap().set_books(books_model.clone().into()); // Overwrite the books variable in the UI with the books model.
        ui_weak.unwrap().window().request_redraw(); // Redraw after adding is necessary because the list doesn't refresh when the cursor is on the button.
    });

    ui.run().unwrap();
}