use std::env;

use what3words::{
    AutosuggestOptions, Coordinates, GridSectionGeoJson, GridSectionJson, What3words,
};

#[::tokio::main]
async fn main() -> what3words::Result<()> {
    let api_key = env::var("W3W_API_KEY").expect(
        "Please ensure that W3W_API_KEY is added to your environment variables.\nRun `W3W_API_KEY=<YOUR_API_KEY> cargo run --example wrapper-demo` from bash/zsh or `$Env:W3W_API_KEY=<YOUR_API_KEY>; cargo run --example wrapper-demo` from PowerShell.",
    );
    let w3w = What3words::new(api_key).header("X-Foo", "Bar");
    // ------ CONVERT TO COORDINATES/3WA ------
    let address = w3w.convert_to_coordinates("filled.count.soap").await?;
    println!("Coordinates: {:?}", address.coordinates);
    let coordinates = Coordinates {
        lat: 51.520847,
        lng: -0.195521,
    };
    // ------ Error ------
    // let address = w3w.convert_to_coordinates("filled.count").await?;
    // println!("{}", address);
    // -------------------
    let address = w3w.convert_to_coordinates("filled.count.soap").await?;
    println!("{:?}", address);
    let address = w3w.convert_to_3wa(&coordinates).await?;
    println!("{:?}", address);
    // ------ ALL AVAILABLE LANGUAGES ------
    let languages = w3w.available_languages().await?;
    println!("{:?}", languages.languages);
    // ------ GRID SECTION ------
    let grid_section_json = w3w
        .grid_section::<GridSectionJson>("52.207988,0.116126,52.208867,0.117540")
        .await?;
    println!("{:?}", &grid_section_json.lines[0]); // Line { start: Coordinates { lat: 52.20801, lng: 0.116126 }, end: Coordinates { lat: 52.20801, lng: 0.11754 } }
    let grid_section_geojson = w3w
        .grid_section::<GridSectionGeoJson>("52.207988,0.116126,52.208867,0.117540")
        .await?;
    println!("{:?}", &grid_section_geojson.features);
    // ------ AUTOSUGGEST ------
    let autosuggest_option = AutosuggestOptions::default().focus("51.520847,-0.195521");
    let autosuggest = w3w
        .autosuggest("filled.count.so", Some(&autosuggest_option))
        .await?;
    println!("{:?} ", autosuggest.suggestions);
    // let autosuggest_with_coordinates = w3w
    //     .autosuggest_with_coordinates("filled.count.so", Some(&autosuggest_option))
    //     .await?;
    // println!("{:?}", autosuggest_with_coordinates);
    let selected = autosuggest.suggestions.first().expect("Not found");
    w3w.autosuggest_selection("f.f.f", selected, Some(&autosuggest_option))
        .await?;
    // ------ HELPER FUNCTIONS ------
    let is_possible_3wa: bool = w3w.is_possible_3wa("filled.count.soap");
    println!("{}", is_possible_3wa);
    let is_possible_3wa: bool = w3w.is_possible_3wa("not a 3wa");
    println!("{}", is_possible_3wa);
    let is_possible_3wa: bool = w3w.is_possible_3wa("not.a 3wa");
    println!("{}", is_possible_3wa);
    let all_possible_3wa: Vec<String> =
        w3w.find_possible_3wa("from index.home.raft to filled.count.soap");
    println!("All possible 3wa {:?}", all_possible_3wa);
    let find_possible_3wa: Vec<String> =
        w3w.find_possible_3wa("Please leave by my porch at filled.count.soap");
    println!("{:?}", find_possible_3wa); // ["filled.count.soap"]
    let find_possible_3wa: Vec<String> =
        w3w.find_possible_3wa("Please leave by my porch at filled.count.soap or deed.tulip.judge");
    println!("{:?}", find_possible_3wa); // ["filled.count.soap", "deed.tulip.judge"]
    let find_possible_3wa: Vec<String> = w3w.find_possible_3wa("Please leave by my porch");
    println!("{:?}", find_possible_3wa); // []
    let is_valid_3wa: bool = w3w.is_valid_3wa("filled.count.soap");
    println!("{}", is_valid_3wa); // true
    let is_valid_3wa: bool = w3w.is_valid_3wa("filled.count.");
    println!("{}", is_valid_3wa); // false
    let is_valid_3wa: bool = w3w.is_valid_3wa("rust.is.cool");
    println!("{}", is_valid_3wa); // false

    Ok(())
}
