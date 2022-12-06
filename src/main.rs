use truc::{MeiliDeserError, SearchQuery};

fn main() {
    let input = r#"{
        "q": "toto",
        "offset": 0,
        "limit": 0,
        "page": null,
        "hitsPerPage": null,
        "attributesToRetrieve": null,
        "attributesToCrop": null,
        "cropLength": 0,
        "attributesToHighlight": null,
        "filter": false,
        "sort": null,
        "showMatchesPosition": false,
        "facets": null,
        "highlightPreTag": "<em>",
        "highlightPostTag": "</em>",
        "cropMarker": "...",
        "matchingStrategy": "last"
    }"#;

    // warmup
    for _ in 0..1000 {
        let _s: SearchQuery = serde_json::from_str(&input).unwrap();
    }

    let iter = 10000000;
    let time = std::time::Instant::now();
    for _ in 0..iter {
        let _s: SearchQuery = serde_json::from_str(&input).unwrap();
    }
    println!(
        "On average, deserializing one search request with SERDE_JSON took: {:?}.",
        time.elapsed() / iter
    );

    // warmup
    for _ in 0..1000 {
        let json: serde_json::Value = serde_json::from_str(&input).unwrap();
        let _data =
            jayson::deserialize::<SearchQuery, serde_json::Value, MeiliDeserError>(json).unwrap();
    }

    let iter = 10000000;
    let time = std::time::Instant::now();
    for _ in 0..iter {
        let json: serde_json::Value = serde_json::from_str(&input).unwrap();
        let _data =
            jayson::deserialize::<SearchQuery, serde_json::Value, MeiliDeserError>(json).unwrap();
    }
    println!(
        "On average, deserializing one search request with JAYSON took: {:?}.",
        time.elapsed() / iter
    );
}
