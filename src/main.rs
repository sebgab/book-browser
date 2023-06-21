slint::include_modules!();
use std::rc::Rc;
use get_book_data::get_book_data;
use slint::{VecModel, Model, ModelRc, SharedString};
mod get_book_data;

#[tokio::main]
async fn main() {
    let ui = App::new().unwrap();

    // Get the pre-existing list of books
    let books: Vec<BookData> = ui.get_books().iter().collect();
    let books_model = Rc::new(VecModel::from(books));

    let ui_weak = ui.as_weak(); // Get a weak handle to the UI so we can do stuff within the move.

    // Lookup the book
    ui.on_lookup_isbn(move |isbn: SharedString| {
        // Get the book data
        let book_future = get_book_data(&isbn);
        let book = tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(book_future)); // Wait for the async function to return
        // TODO: Consider spawning separate thread

        if book.is_some() {
            let book_data = book.unwrap();

            // Convert the string array into a SharedString array
            let authors_shared_string: VecModel<SharedString> = VecModel::default();
            for author in book_data.authors {
                authors_shared_string.push(SharedString::from(author));
            }
            // Create a ModelRc from this data
            let authors_model: ModelRc<SharedString> = ModelRc::new(authors_shared_string);

            // Create the BookData from the collected data
            let converted_book_data = BookData {
                cover_image: slint::Image::load_from_path(std::path::Path::new("ui/book-cover.jpg")).unwrap(), // TODO: Get and set the image
                title: book_data.title.into(),
                description: book_data.text_snippet.into(), // TODO: Add description and short text like is in BookDataFull
                authors: authors_model,
                owner: "N/A".into()
            };

            books_model.push(converted_book_data); // Add it to the books array
            ui_weak.unwrap().set_books(books_model.clone().into()); // Overwrite the books variable in the UI with the books model.
            ui_weak.unwrap().window().request_redraw(); // Redraw after adding is necessary because the list doesn't refresh when the cursor is on the button.
        }
    });

    ui.run().unwrap();
}