use reqwest;

async fn get_book_data(isbn: &str) -> Result<String, reqwest::Error> {
    // TODO: Consider changing to a client, as this is recommended, but this works for now.
    let url_string: String = format!("https://www.googleapis.com/books/v1/volumes?q=isbn:{}", isbn);
    let body: String = reqwest::get(url_string).await?.text().await?;

    return Ok(body);
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::get_book_data;

    #[tokio::test]
    async fn test_get_book_data() {
        let raw_result = get_book_data("9781491927250").await.unwrap();
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

}