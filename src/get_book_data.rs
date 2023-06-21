use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookDataFull {
  pub title: String,
  pub subtitle: String,
  pub authors: Vec<String>,
  pub publisher: String,
  pub published_date: String,
  pub description: String,
  pub isbn: String,
  pub thumbnail: String,
  pub text_snippet: String
}

// TODO: Add cover downloading

#[derive(Debug, Deserialize)]
struct BookJsonStart {
    items: Vec<RawBookDataFull>
}

#[derive(Debug, Deserialize)]
struct RawBookDataFull {
    #[serde(rename = "volumeInfo")]
    volume_info: VolumeInfo,
    #[serde(rename = "searchInfo")]
    search_info: SearchInfo,
}

#[derive(Debug, Deserialize)]
struct VolumeInfo{
    title: String,
    subtitle: String,
    authors: Vec<String>,
    publisher: String,
    #[serde(rename = "publishedDate")]
    published_date: String,
    description: String,
    #[serde(rename = "industryIdentifiers")]
    isbn: Vec<IsbnIdentifier>,
    #[serde(rename = "imageLinks")]
    thumbnail: ImageLinks,
}

#[derive(Debug, Deserialize)]
struct IsbnIdentifier {
    #[serde(rename = "identifier")]
    isbn: String,
}

#[derive(Debug, Deserialize)]
struct ImageLinks {
    thumbnail: String,
}

#[derive(Debug, Deserialize)]
struct SearchInfo {
    #[serde(rename = "textSnippet")]
    text_snippet: String,
}

async fn fetch_book_data(isbn: &str) -> Result<String, reqwest::Error> {
    // TODO: Consider changing to a client, as this is recommended, but this works for now.
    let url_string: String = format!("https://www.googleapis.com/books/v1/volumes?q=isbn:{}", isbn);
    let body: String = reqwest::get(url_string).await?.text().await?;

    return Ok(body);
}

fn parse_book_data_json(input_json: &str) -> Result<BookDataFull, std::io::Error> {
    let parsed_json: BookJsonStart = serde_json::from_str(input_json)?;
    let raw_book_data = parsed_json.items.get(0).unwrap(); // TODO: Remove unwrap and handle this properly

    // Extract the ISBN
    let isbn: String;
    if raw_book_data.volume_info.isbn.get(0).is_some() {
        isbn = raw_book_data.volume_info.isbn.get(0).unwrap().isbn.clone();
    } else {
        isbn = "".to_string();
    }
    let book_data_extracted: BookDataFull = BookDataFull {
        title: raw_book_data.volume_info.title.clone(),
        subtitle: raw_book_data.volume_info.subtitle.clone(),
        authors: raw_book_data.volume_info.authors.clone(),
        publisher: raw_book_data.volume_info.publisher.clone(),
        published_date: raw_book_data.volume_info.published_date.clone(),
        description: raw_book_data.volume_info.description.clone(),
        isbn: isbn.clone(),
        thumbnail: raw_book_data.volume_info.thumbnail.thumbnail.clone(),
        text_snippet: raw_book_data.search_info.text_snippet.clone()
    };

    return Ok(book_data_extracted);
}

pub async fn get_book_data(isbn: &str) -> Option<BookDataFull> {
    let fetched_json: Result<String, reqwest::Error> = fetch_book_data(isbn).await;
    let book_json: String;
    match fetched_json {
        Ok(result) => { book_json = result; },
        Err(e) => { eprintln!("Failed to fetch JSON from ISBN: `{isbn}` with error:\n\t{}", e); return None; }
    }

    let parsed_book_data: Result<BookDataFull, std::io::Error> = parse_book_data_json(&book_json);
    let book_data: BookDataFull;
    match parsed_book_data {
        Ok(result) => { book_data = result; },
        Err(e) => { eprintln!("Failed to parse JSON from ISBN: `{isbn}` with error:\n\t{}", e); return None; }
    }

    return Some(book_data);
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::{fetch_book_data, BookDataFull, parse_book_data_json, get_book_data};

    #[tokio::test]
    async fn fetch_book_data_test() {
        let raw_result = fetch_book_data("9781491927250").await.unwrap();
        //let expected_response: String = r#"{"kind":"books#volumes","totalItems":1,"items":[{"kind":"books#volume","id":"h8c_DwAAQBAJ","etag":"6IyektsAgX8","selfLink":"https://www.googleapis.com/books/v1/volumes/h8c_DwAAQBAJ","volumeInfo":{"title":"ProgrammingRust","subtitle":"Fast,SafeSystemsDevelopment","authors":["JimBlandy","JasonOrendorff"],"publisher":"\"O'ReillyMedia,Inc.\"","publishedDate":"2017-11-21","description":"Rustisanewsystemsprogramminglanguagethatcombinestheperformanceandlow-levelcontrolofCandC++withmemorysafetyandthreadsafety.Rust’smodern,flexibletypesensureyourprogramisfreeofnullpointerdereferences,doublefrees,danglingpointers,andsimilarbugs,allatcompiletime,withoutruntimeoverhead.Inmulti-threadedcode,Rustcatchesdataracesatcompiletime,makingconcurrencymucheasiertouse.Writtenbytwoexperiencedsystemsprogrammers,thisbookexplainshowRustmanagestobridgethegapbetweenperformanceandsafety,andhowyoucantakeadvantageofit.Topicsinclude:HowRustrepresentsvaluesinmemory(withdiagrams)Completeexplanationsofownership,moves,borrows,andlifetimesCargo,rustdoc,unittests,andhowtopublishyourcodeoncrates.io,Rust’spublicpackagerepositoryHigh-levelfeatureslikegenericcode,closures,collections,anditeratorsthatmakeRustproductiveandflexibleConcurrencyinRust:threads,mutexes,channels,andatomics,allmuchsafertousethaninCorC++Unsafecode,andhowtopreservetheintegrityofordinarycodethatusesitExtendedexamplesillustratinghowpiecesofthelanguagefittogether","industryIdentifiers":[{"type":"ISBN_13","identifier":"9781491927250"},{"type":"ISBN_10","identifier":"1491927259"}],"readingModes":{"text":false,"image":true},"pageCount":621,"printType":"BOOK","categories":["Computers"],"averageRating":5,"ratingsCount":1,"maturityRating":"NOT_MATURE","allowAnonLogging":false,"contentVersion":"0.2.0.0.preview.1","panelizationSummary":{"containsEpubBubbles":false,"containsImageBubbles":false},"imageLinks":{"smallThumbnail":"http://books.google.com/books/content?id=h8c_DwAAQBAJ&printsec=frontcover&img=1&zoom=5&edge=curl&source=gbs_api","thumbnail":"http://books.google.com/books/content?id=h8c_DwAAQBAJ&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api"},"language":"en","previewLink":"http://books.google.no/books?id=h8c_DwAAQBAJ&printsec=frontcover&dq=isbn:9781491927250&hl=&cd=1&source=gbs_api","infoLink":"http://books.google.no/books?id=h8c_DwAAQBAJ&dq=isbn:9781491927250&hl=&source=gbs_api","canonicalVolumeLink":"https://books.google.com/books/about/Programming_Rust.html?hl=&id=h8c_DwAAQBAJ"},"saleInfo":{"country":"NO","saleability":"NOT_FOR_SALE","isEbook":false},"accessInfo":{"country":"NO","viewability":"PARTIAL","embeddable":true,"publicDomain":false,"textToSpeechPermission":"ALLOWED","epub":{"isAvailable":false},"pdf":{"isAvailable":true},"webReaderLink":"http://play.google.com/books/reader?id=h8c_DwAAQBAJ&hl=&source=gbs_api","accessViewStatus":"SAMPLE","quoteSharingAllowed":false},"searchInfo":{"textSnippet":"Writtenbytwoexperiencedsystemsprogrammers,thisbookexplainshowRustmanagestobridgethegapbetweenperformanceandsafety,andhowyoucantakeadvantageofit."}}]}"#.to_string();
        let expected_response: String = r#"{"kind":"books#volumes","totalItems":1,"items":[{"kind":"books#volume","id":"h8c_DwAAQBAJ","selfLink":"https://www.googleapis.com/books/v1/volumes/h8c_DwAAQBAJ","volumeInfo":{"title":"ProgrammingRust","subtitle":"Fast,SafeSystemsDevelopment","authors":["JimBlandy","JasonOrendorff"],"publisher":"\"O'ReillyMedia,Inc.\"","publishedDate":"2017-11-21","description":"Rustisanewsystemsprogramminglanguagethatcombinestheperformanceandlow-levelcontrolofCandC++withmemorysafetyandthreadsafety.Rust’smodern,flexibletypesensureyourprogramisfreeofnullpointerdereferences,doublefrees,danglingpointers,andsimilarbugs,allatcompiletime,withoutruntimeoverhead.Inmulti-threadedcode,Rustcatchesdataracesatcompiletime,makingconcurrencymucheasiertouse.Writtenbytwoexperiencedsystemsprogrammers,thisbookexplainshowRustmanagestobridgethegapbetweenperformanceandsafety,andhowyoucantakeadvantageofit.Topicsinclude:HowRustrepresentsvaluesinmemory(withdiagrams)Completeexplanationsofownership,moves,borrows,andlifetimesCargo,rustdoc,unittests,andhowtopublishyourcodeoncrates.io,Rust’spublicpackagerepositoryHigh-levelfeatureslikegenericcode,closures,collections,anditeratorsthatmakeRustproductiveandflexibleConcurrencyinRust:threads,mutexes,channels,andatomics,allmuchsafertousethaninCorC++Unsafecode,andhowtopreservetheintegrityofordinarycodethatusesitExtendedexamplesillustratinghowpiecesofthelanguagefittogether","industryIdentifiers":[{"type":"ISBN_13","identifier":"9781491927250"},{"type":"ISBN_10","identifier":"1491927259"}],"readingModes":{"text":false,"image":true},"pageCount":621,"printType":"BOOK","categories":["Computers"],"averageRating":5,"ratingsCount":1,"maturityRating":"NOT_MATURE","allowAnonLogging":false,"contentVersion":"0.2.0.0.preview.1","panelizationSummary":{"containsEpubBubbles":false,"containsImageBubbles":false},"imageLinks":{"smallThumbnail":"http://books.google.com/books/content?id=h8c_DwAAQBAJ&printsec=frontcover&img=1&zoom=5&edge=curl&source=gbs_api","thumbnail":"http://books.google.com/books/content?id=h8c_DwAAQBAJ&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api"},"language":"en","previewLink":"http://books.google.no/books?id=h8c_DwAAQBAJ&printsec=frontcover&dq=isbn:9781491927250&hl=&cd=1&source=gbs_api","infoLink":"http://books.google.no/books?id=h8c_DwAAQBAJ&dq=isbn:9781491927250&hl=&source=gbs_api","canonicalVolumeLink":"https://books.google.com/books/about/Programming_Rust.html?hl=&id=h8c_DwAAQBAJ"},"saleInfo":{"country":"NO","saleability":"NOT_FOR_SALE","isEbook":false},"accessInfo":{"country":"NO","viewability":"PARTIAL","embeddable":true,"publicDomain":false,"textToSpeechPermission":"ALLOWED","epub":{"isAvailable":false},"pdf":{"isAvailable":true},"webReaderLink":"http://play.google.com/books/reader?id=h8c_DwAAQBAJ&hl=&source=gbs_api","accessViewStatus":"SAMPLE","quoteSharingAllowed":false},"searchInfo":{"textSnippet":"Writtenbytwoexperiencedsystemsprogrammers,thisbookexplainshowRustmanagestobridgethegapbetweenperformanceandsafety,andhowyoucantakeadvantageofit."}}]}"#.to_string();

        // Remove all whitespace from the result, as we just want to see if the data is correct, and ignore the formatting
        let result_without_whitespace: String = raw_result.chars().filter(|c| !c.is_whitespace()).collect();

        // We need to remove the etag field, because it is randomly generated for each request
        let regex_pattern = r#""etag":"[^"]+","#;
        let regex = Regex::new(regex_pattern).unwrap();
        let result = regex.replace_all(&result_without_whitespace, "");

        assert_eq!(result, expected_response);
    }

    #[test]
    fn parse_book_data_json_test() {
        let json_to_parse: String = r#"{
            "kind": "books#volumes",
            "totalItems": 1,
            "items": [
              {
                "kind": "books#volume",
                "id": "h8c_DwAAQBAJ",
                "etag": "wIfwunhDOt0",
                "selfLink": "https://www.googleapis.com/books/v1/volumes/h8c_DwAAQBAJ",
                "volumeInfo": {
                  "title": "Programming Rust",
                  "subtitle": "Fast, Safe Systems Development",
                  "authors": [
                    "Jim Blandy",
                    "Jason Orendorff"
                  ],
                  "publisher": "\"O'Reilly Media, Inc.\"",
                  "publishedDate": "2017-11-21",
                  "description": "Rust is a new systems programming language that combines the performance and low-level control of C and C++ with memory safety and thread safety. Rust’s modern, flexible types ensure your program is free of null pointer dereferences, double frees, dangling pointers, and similar bugs, all at compile time, without runtime overhead. In multi-threaded code, Rust catches data races at compile time, making concurrency much easier to use. Written by two experienced systems programmers, this book explains how Rust manages to bridge the gap between performance and safety, and how you can take advantage of it. Topics include: How Rust represents values in memory (with diagrams) Complete explanations of ownership, moves, borrows, and lifetimes Cargo, rustdoc, unit tests, and how to publish your code on crates.io, Rust’s public package repository High-level features like generic code, closures, collections, and iterators that make Rust productive and flexible Concurrency in Rust: threads, mutexes, channels, and atomics, all much safer to use than in C or C++ Unsafe code, and how to preserve the integrity of ordinary code that uses it Extended examples illustrating how pieces of the language fit together",
                  "industryIdentifiers": [
                    {
                      "type": "ISBN_13",
                      "identifier": "9781491927250"
                    },
                    {
                      "type": "ISBN_10",
                      "identifier": "1491927259"
                    }
                  ],
                  "readingModes": {
                    "text": false,
                    "image": true
                  },
                  "pageCount": 621,
                  "printType": "BOOK",
                  "categories": [
                    "Computers"
                  ],
                  "averageRating": 5,
                  "ratingsCount": 1,
                  "maturityRating": "NOT_MATURE",
                  "allowAnonLogging": false,
                  "contentVersion": "0.2.0.0.preview.1",
                  "panelizationSummary": {
                    "containsEpubBubbles": false,
                    "containsImageBubbles": false
                  },
                  "imageLinks": {
                    "smallThumbnail": "http://books.google.com/books/content?id=h8c_DwAAQBAJ&printsec=frontcover&img=1&zoom=5&edge=curl&source=gbs_api",
                    "thumbnail": "http://books.google.com/books/content?id=h8c_DwAAQBAJ&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api"
                  },
                  "language": "en",
                  "previewLink": "http://books.google.no/books?id=h8c_DwAAQBAJ&printsec=frontcover&dq=isbn:9781491927250&hl=&cd=1&source=gbs_api",
                  "infoLink": "http://books.google.no/books?id=h8c_DwAAQBAJ&dq=isbn:9781491927250&hl=&source=gbs_api",
                  "canonicalVolumeLink": "https://books.google.com/books/about/Programming_Rust.html?hl=&id=h8c_DwAAQBAJ"
                },
                "saleInfo": {
                  "country": "NO",
                  "saleability": "NOT_FOR_SALE",
                  "isEbook": false
                },
                "accessInfo": {
                  "country": "NO",
                  "viewability": "PARTIAL",
                  "embeddable": true,
                  "publicDomain": false,
                  "textToSpeechPermission": "ALLOWED",
                  "epub": {
                    "isAvailable": false
                  },
                  "pdf": {
                    "isAvailable": true
                  },
                  "webReaderLink": "http://play.google.com/books/reader?id=h8c_DwAAQBAJ&hl=&source=gbs_api",
                  "accessViewStatus": "SAMPLE",
                  "quoteSharingAllowed": false
                },
                "searchInfo": {
                  "textSnippet": "Written by two experienced systems programmers, this book explains how Rust manages to bridge the gap between performance and safety, and how you can take advantage of it."
                }
              }
            ]
          }
          "#.to_string();
        let expected_result: BookDataFull = BookDataFull {
            title: "Programming Rust".to_string(),
            subtitle: "Fast, Safe Systems Development".to_string(),
            authors: vec!["Jim Blandy".to_string(), "Jason Orendorff".to_string()],
            publisher: r#""O'Reilly Media, Inc.""#.to_string(),
            published_date: "2017-11-21".to_string(),
            description: r#"Rust is a new systems programming language that combines the performance and low-level control of C and C++ with memory safety and thread safety. Rust’s modern, flexible types ensure your program is free of null pointer dereferences, double frees, dangling pointers, and similar bugs, all at compile time, without runtime overhead. In multi-threaded code, Rust catches data races at compile time, making concurrency much easier to use. Written by two experienced systems programmers, this book explains how Rust manages to bridge the gap between performance and safety, and how you can take advantage of it. Topics include: How Rust represents values in memory (with diagrams) Complete explanations of ownership, moves, borrows, and lifetimes Cargo, rustdoc, unit tests, and how to publish your code on crates.io, Rust’s public package repository High-level features like generic code, closures, collections, and iterators that make Rust productive and flexible Concurrency in Rust: threads, mutexes, channels, and atomics, all much safer to use than in C or C++ Unsafe code, and how to preserve the integrity of ordinary code that uses it Extended examples illustrating how pieces of the language fit together"#.to_string(),
            isbn: "9781491927250".to_string(),
            thumbnail: "http://books.google.com/books/content?id=h8c_DwAAQBAJ&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api".to_string(),
            text_snippet: "Written by two experienced systems programmers, this book explains how Rust manages to bridge the gap between performance and safety, and how you can take advantage of it.".to_string()
        };

        let result = parse_book_data_json(&json_to_parse).unwrap();
        assert_eq!(result, expected_result);
    }

    #[tokio::test]
    async fn get_book_data_test() {
        let result = get_book_data("9781491927250").await.unwrap();

        let expected_result: BookDataFull = BookDataFull {
            title: "Programming Rust".to_string(),
            subtitle: "Fast, Safe Systems Development".to_string(),
            authors: vec!["Jim Blandy".to_string(), "Jason Orendorff".to_string()],
            publisher: r#""O'Reilly Media, Inc.""#.to_string(),
            published_date: "2017-11-21".to_string(),
            description: r#"Rust is a new systems programming language that combines the performance and low-level control of C and C++ with memory safety and thread safety. Rust’s modern, flexible types ensure your program is free of null pointer dereferences, double frees, dangling pointers, and similar bugs, all at compile time, without runtime overhead. In multi-threaded code, Rust catches data races at compile time, making concurrency much easier to use. Written by two experienced systems programmers, this book explains how Rust manages to bridge the gap between performance and safety, and how you can take advantage of it. Topics include: How Rust represents values in memory (with diagrams) Complete explanations of ownership, moves, borrows, and lifetimes Cargo, rustdoc, unit tests, and how to publish your code on crates.io, Rust’s public package repository High-level features like generic code, closures, collections, and iterators that make Rust productive and flexible Concurrency in Rust: threads, mutexes, channels, and atomics, all much safer to use than in C or C++ Unsafe code, and how to preserve the integrity of ordinary code that uses it Extended examples illustrating how pieces of the language fit together"#.to_string(),
            isbn: "9781491927250".to_string(),
            thumbnail: "http://books.google.com/books/content?id=h8c_DwAAQBAJ&printsec=frontcover&img=1&zoom=1&edge=curl&source=gbs_api".to_string(),
            text_snippet: "Written by two experienced systems programmers, this book explains how Rust manages to bridge the gap between performance and safety, and how you can take advantage of it.".to_string()
        };
        assert_eq!(result, expected_result);
    }
}