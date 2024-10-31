use std::env;

use what3words::{
    Address, AddressGeoJson, Autosuggest, AutosuggestSelection, BoundingBox, ConvertTo3wa,
    ConvertToCoordinates, Coordinates, GridSection, GridSectionGeoJson, What3words,
};

#[::tokio::main]
async fn main() -> what3words::Result<()> {
    let api_key = env::var("W3W_API_KEY").expect(
        "Please ensure that W3W_API_KEY is added to your environment variables.\nRun `W3W_API_KEY=<YOUR_API_KEY> cargo run --example wrapper-demo` from bash/zsh or `$Env:W3W_API_KEY=<YOUR_API_KEY>; cargo run --example wrapper-demo` from PowerShell.",
    );
    let w3w = What3words::new(&api_key).header("X-Foo", "Bar");
    let words = "filled.count.soap";
    // ------ CONVERT TO COORDINATES/3WA ------
    // ------ Error ------
    let address = w3w
        .convert_to_coordinates_async::<Address>(ConvertToCoordinates::new("filled.count"))
        .await;
    match address {
        Ok(address) => println!("Address {:?}", address),
        Err(error) => println!("Error {:?}", error),
    }
    // -------------------
    let convert_to_coordinates = ConvertToCoordinates::new(words);
    let address: Address = w3w
        .convert_to_coordinates_async(convert_to_coordinates)
        .await?;
    println!("Convert to Coordinates Json Format (async)");
    println!("{:?}", address);
    let convert_to_coordinates = ConvertToCoordinates::new(words);
    let address: AddressGeoJson = w3w
        .convert_to_coordinates_async(convert_to_coordinates)
        .await?;
    println!("Convert to Coordinates GeoJson Format (async)");
    println!("{:?}", address);
    let convert_to_3wa = ConvertTo3wa::new(51.520847, -0.195521);
    let address: Address = w3w.convert_to_3wa_async(convert_to_3wa).await?;
    println!("Convert to 3WA Json Format (async)");
    println!("{:?}", address);
    let convert_to_3wa = ConvertTo3wa::new(51.520847, -0.195521);
    let address: AddressGeoJson = w3w.convert_to_3wa_async(convert_to_3wa).await?;
    println!("Convert to 3WA GeoJson Format (async)");
    println!("{:?}", address);
    // ------ ALL AVAILABLE LANGUAGES ------
    let languages = w3w.available_languages_async().await?;
    println!("Available Languages (async)");
    println!("{:?}", languages.languages);
    // ------ GRID SECTION ------
    let grid_section_json: GridSection = w3w
        .grid_section_async(BoundingBox::new(52.207988, 0.116126, 52.208867, 0.117540))
        .await?;
    println!("Grid Section Json Format (async)");
    println!("{:?}", grid_section_json);

    let grid_section_geojson: GridSectionGeoJson = w3w
        .grid_section_async(BoundingBox::new(52.207988, 0.116126, 52.208867, 0.117540))
        .await?;
    println!("Grid Section GeoJson Format (async)");
    println!("{:?}", grid_section_geojson);

    // ------ AUTOSUGGEST ------
    let autosuggest_option =
        Autosuggest::new("filled.count.so").focus(Coordinates::new(51.520847, -0.195521));
    let autosuggest = w3w.autosuggest_async(&autosuggest_option).await?;
    println!("Autosuggest (async)");
    println!("{:?} ", autosuggest);

    // ------ AUTOSUGGEST WITH COORDINATES ------
    let autosuggest_with_coordinates = w3w
        .autosuggest_with_coordinates_async(&autosuggest_option)
        .await;
    println!("Autosuggest with Coordinates (async)");
    match autosuggest_with_coordinates {
        Ok(autosuggest_with_coordinates) => println!("{:?}", autosuggest_with_coordinates),
        Err(err) => println!("{:?}", err),
    }
    // ------ AUTOSUGGEST SELECTION ------
    let selected = autosuggest.suggestions.first().expect("Not found");
    match w3w
        .autosuggest_selection_async(
            AutosuggestSelection::new("f.f.f", selected).options(&autosuggest_option),
        )
        .await
    {
        Ok(_) => println!("Suggested selection sent (async)"),
        Err(err) => println!("{:?}", err),
    };
    // ------ HELPER FUNCTIONS ------
    let is_valid_3wa: bool = w3w.is_valid_3wa_async("filled.count.soap").await;
    println!("is_valid_3wa_async [1]: {}", is_valid_3wa);
    let is_valid_3wa: bool = w3w.is_valid_3wa_async("filled.count.").await;
    println!("is_valid_3wa_async [2]: {}", is_valid_3wa);
    let is_valid_3wa: bool = w3w.is_valid_3wa_async("rust.is.cool").await;
    println!("is_valid_3wa_async [3]: {}", is_valid_3wa);

    Ok(())
}
