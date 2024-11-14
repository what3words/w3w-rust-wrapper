use std::env;

use what3words_api::{
    Address, AddressGeoJson, Autosuggest, AutosuggestSelection, BoundingBox, ConvertTo3wa,
    ConvertToCoordinates, Coordinates, Error, GridSection, GridSectionGeoJson, What3words,
};

fn main() -> Result<(), Error> {
    let api_key = env::var("W3W_API_KEY").expect(
        "Please ensure that W3W_API_KEY is added to your environment variables.\nRun `W3W_API_KEY=<YOUR_API_KEY> cargo run --example wrapper-demo` from bash/zsh or `$Env:W3W_API_KEY=<YOUR_API_KEY>; cargo run --example wrapper-demo` from PowerShell.",
    );
    let w3w = What3words::new(&api_key).header("X-Foo", "Bar");
    let words = "filled.count.soap";
    // ------ CONVERT TO COORDINATES/3WA ------
    // ------ Error ------
    let address = w3w.convert_to_coordinates::<Address>(&ConvertToCoordinates::new("filled.count"));
    match address {
        Ok(address) => println!("Address {:?}", address),
        Err(error) => println!("Error {:?}", error),
    }
    // -------------------
    let convert_to_coordinates = ConvertToCoordinates::new(words);
    let address: Address = w3w.convert_to_coordinates(&convert_to_coordinates)?;
    println!("Convert to Coordinates Json Format");
    println!("{:?}", address);
    let convert_to_coordinates = ConvertToCoordinates::new(words);
    let address: AddressGeoJson = w3w.convert_to_coordinates(&convert_to_coordinates)?;
    println!("Convert to Coordinates GeoJson Format");
    println!("{:?}", address);
    let convert_to_3wa = ConvertTo3wa::new(51.520847, -0.195521);
    let address: Address = w3w.convert_to_3wa(&convert_to_3wa)?;
    println!("Convert to 3WA Json Format");
    println!("{:?}", address);
    let convert_to_3wa = ConvertTo3wa::new(51.520847, -0.195521);
    let address: AddressGeoJson = w3w.convert_to_3wa(&convert_to_3wa)?;
    println!("Convert to 3WA GeoJson Format");
    println!("{:?}", address);

    // ------ ALL AVAILABLE LANGUAGES ------
    let languages = w3w.available_languages()?;
    println!("Available Languages");
    println!("{:?}", languages.languages);
    // ------ GRID SECTION ------
    let grid_section_json: GridSection =
        w3w.grid_section(&BoundingBox::new(52.207988, 0.116126, 52.208867, 0.117540))?;
    println!("Grid Section Json Format");
    println!("{:?}", grid_section_json);
    let grid_section_geojson: GridSectionGeoJson =
        w3w.grid_section(&BoundingBox::new(52.207988, 0.116126, 52.208867, 0.117540))?;
    println!("Grid Section GeoJson Format");
    println!("{:?}", grid_section_geojson);
    // ------ AUTOSUGGEST ------
    let autosuggest_option =
        Autosuggest::new("filled.count.so").focus(&Coordinates::new(51.520847, -0.195521));
    let autosuggest = w3w.autosuggest(&autosuggest_option)?;
    println!("Autosuggest");
    println!("{:?} ", autosuggest);
    // ------ AUTOSUGGEST WITH COORDINATES ------
    let autosuggest_with_coordinates = w3w.autosuggest_with_coordinates(&autosuggest_option);
    println!("Autosuggest with Coordinates");
    match autosuggest_with_coordinates {
        Ok(autosuggest_with_coordinates) => println!("{:?}", autosuggest_with_coordinates),
        Err(err) => println!("{:?}", err),
    }
    // ------ AUTOSUGGEST SELECTION ------
    let selected = autosuggest.suggestions.first().expect("Not found");
    match w3w.autosuggest_selection(
        &AutosuggestSelection::new("f.f.f", selected).options(&autosuggest_option),
    ) {
        Ok(_) => println!("Suggested selection sent"),
        Err(err) => println!("{:?}", err),
    };
    // ------ HELPER FUNCTIONS ------
    let is_valid_3wa: bool = w3w.is_valid_3wa("filled.count.soap");
    println!("is_valid_3wa [1]: {}", is_valid_3wa);
    let is_valid_3wa: bool = w3w.is_valid_3wa("filled.count");
    println!("is_valid_3wa [2]: {}", is_valid_3wa);
    let is_valid_3wa: bool = w3w.is_valid_3wa("rust.is.cool");
    println!("is_valid_3wa [3]: {}", is_valid_3wa);
    let did_you_mean: bool = w3w.did_you_mean("filled count soap");
    println!("did_you_mean [1]: {}", did_you_mean);
    let did_you_mean: bool = w3w.did_you_mean("filled-count-soap");
    println!("did_you_mean [2]: {}", did_you_mean);
    let did_you_mean: bool = w3w.did_you_mean("filledcountsoap");
    println!("did_you_mean [3]: {}", did_you_mean);
    let is_possible_3wa: bool = w3w.is_possible_3wa("filled.count.soap");
    println!("is_possible_3wa [1]: {}", is_possible_3wa);
    let is_possible_3wa: bool = w3w.is_possible_3wa("not a 3wa");
    println!("is_possible_3wa [2]: {}", is_possible_3wa);
    let is_possible_3wa: bool = w3w.is_possible_3wa("not.a 3wa");
    println!("is_possible_3wa [3]: {}", is_possible_3wa);
    let all_possible_3wa: Vec<String> =
        w3w.find_possible_3wa("from index.home.raft to filled.count.soap");
    println!("find_possible_3wa [1]: {:?}", all_possible_3wa);
    let find_possible_3wa: Vec<String> =
        w3w.find_possible_3wa("Please leave by my porch at filled.count.soap");
    println!("find_possible_3wa [2]: {:?}", find_possible_3wa);
    let find_possible_3wa: Vec<String> =
        w3w.find_possible_3wa("Please leave by my porch at filled.count.soap or deed.tulip.judge");
    println!("find_possible_3wa [3]: {:?}", find_possible_3wa);
    let find_possible_3wa: Vec<String> = w3w.find_possible_3wa("Please leave by my porch");
    println!("find_possible_3wa [4]: {:?}", find_possible_3wa);
    let find_possible_3wa = w3w.find_possible_3wa("This is a test with filled count soap in it.");
    println!("find_possible_3wa [5]: {:?}", find_possible_3wa);
    Ok(())
}
