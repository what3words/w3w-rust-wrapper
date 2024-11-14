![what3words](https://what3words.com/assets/images/w3w_square_red.png 'what3words')

# w3w-rust-wrapper

The official rust library for [what3words v3 API](https://docs.what3words.com/api/v3/).

API methods are grouped into a single service object which can be centrally managed by a `What3words`instance.

# Overview

The what3words rust library gives you programmatic access to:

- convert a 3 word address to coordinates
- convert coordinates to a 3 word address
- autosuggest functionality which takes a slightly incorrect 3 word address, and suggests a list of valid 3 word addresses
- obtain a section of the 3m x 3m what3words grid for a bounding box.
- determine the currently support 3 word address languages.

## Authentication

To use this library you’ll need an API key, please visit [https://what3words.com/select-plan](https://what3words.com/select-plan) and sign up for an account.

# Installation

To install what3words, simply:

```bash
$ cargo add what3words-api
```

## Features

The functions are asynchronous by default, but this crate supports synchronous functions as well, simply enable `sync` feature when adding the crate to your project.

```bash
cargo add what3words-api --features=sync
```

> [!NOTE]
> Ensure that you have an async runtime installed such as `tokio` except when `sync` feature is enabled.

# Usage

## Initialisation

Once you have the API Key, you can initialise the wrapper like this:

```rust
let wrapper = what3words_api::What3words::new("YOUR_API_KEY_HERE");
```

### Optional

You can also pass a different hostname if you have your own self-hosted what3words API.

```rust
let wrapper = what3words_api::What3words::new("YOUR_API_KEY_HERE").hostname("https://your.what3words.api/v3");
```

You can also set configure your own headers:

```rust
let wrapper = what3words_api::What3words::new("YOUR_API_KEY_HERE").header("X-Foo", "Bar");
```

## Convert To Coordinates

This function takes an instance of `what3words_api::ConvertToCoordinates` which accepts a string of 3 words `'filled.count.soap'`.

Example:

```rust
let convert_to_coordinates = what3words_api::ConvertToCoordinates::new("filled.count.soap").locale("zh_tr");
```

> [!NOTE]
> The returned payload from the `convert-to-coordinates` method is described in the [what3words REST API documentation](https://docs.what3words.com/api/v3/#convert-to-coordinates).

Example:

> [!NOTE]
> It is required to specify the type annotation for this function which will allow you to choose between `json` and `geojson` format. Using `Address` will use `json` (default) and `AddressGeoJson` will use `geojson`.

```rust
use what3words_api::{Address, AddressGeoJson, ConvertToCoordinates, What3words};

let w3w = What3words::new("YOUR_API_KEY_HERE");

let convert_to_coordinates = ConvertToCoordinates::new("產權.絕緣.墨鏡").locale("zh_tr");
let address_json: Address = w3w.convert_to_coordinates::<Address>(&convert_to_coordinates);
println!("{:?}", address_json.coordinates); // Coordinates { lat: 51.520847, lng: -0.195521 }

let convert_to_coordinates = ConvertToCoordinates::new("filled.count.soap");
let address_geojson: AddressGeoJson = w3w.convert_to_coordinates::<AddressGeoJson>(&convert_to_coordinates);
println!("{:?}", address_geojson.features); // [Feature { bbox: Some[-0.195543, 51.520833], ..., }]
```

## Convert To 3 Word Address

This function takes an instance of `what3words_api::ConvertTo3wa` which accepts a latitude and longitude values (i.e.: `51.520847, -0.195521`):

Example:

```rust
let convert_to_3wa = what3words_api::ConvertTo3wa::new(51.520847, -0.195521).language("oo").locale("oo_cy");
```

> [!NOTE]
> The returned payload from the `convert-to-3wa` method is described in the [what3words REST API documentation](https://docs.what3words.com/api/v3/#convert-to-3wa).

Example:

> [!NOTE]
> It is required to specify the type annotation for this function which will allow you to choose between `json` and `geojson` format. Using `Address` will use `json` (default) and `AddressGeoJson` will use `geojson`.

```rust
use what3words_api::{Address, AddressGeoJson, ConvertTo3wa, What3words};

let w3w = What3words::new("YOUR_API_KEY_HERE");


let convert_to_3wa = ConvertTo3wa::new(51.520847, -0.195521).language("mn").locale("mn_la");
let address_json: Address = w3w.convert_to_3wa::<Address>(&convert_to_3wa);
println!("{:?}", address_json.words); // "seruuhen.zemseg.dagaldah"

let convert_to_3wa = ConvertTo3wa::new(51.520847, -0.195521);
let address_geojson: Address = w3w.convert_to_3wa::<Address>(&convert_to_3wa);
println!("{:?}", address_geojson.features); // [Feature { bbox: Some[-0.195543, 51.520833], ..., }]
```

## AutoSuggest

Returns a list of 3 word addresses based on user input and other parameters.

This method provides corrections for the following types of input error:

- typing errors
- spelling errors
- misremembered words (e.g. singular vs. plural)
- words in the wrong order

The `autosuggest` method determines possible corrections to the supplied 3 word address string based on the probability of the input errors listed above and returns a ranked list of suggestions. This method can also take into consideration the geographic proximity of possible corrections to a given location to further improve the suggestions returned.

### Input 3 word address

You will only receive results back if the partial 3 word address string you submit contains the first two words and at least the first character of the third word; otherwise an error message will be returned.

### Clipping and Focus

We provide various `clip` policies to allow you to specify a geographic area that is used to exclude results that are not likely to be relevant to your users. We recommend that you use the `clip` parameter to give a more targeted, shorter set of results to your user. If you know your user’s current location, we also strongly recommend that you use the `focus` to return results which are likely to be more relevant.

In summary, the `clip` policy is used to optionally restrict the list of candidate AutoSuggest results, after which, if focus has been supplied, this will be used to rank the results in order of relevancy to the focus.

https://docs.what3words.com/api/v3/#autosuggest

> [!NOTE]
> The returned payload from the `autosuggest` method is described in the [what3words REST API documentation](https://docs.what3words.com/api/v3/#autosuggest).

This function takes an instance of `what3words_api::Autosuggest` which accepts a string of partial 3 words `'filled.count.so'`.

Example:

```rust
let autosuggest = what3words_api::Autosuggest::new("filled.count.so");
```

The instance of `what3words_api::Autosuggest` also allows you to set optional parameter(s) (i.e: clipping, focus, etc.):

Examples:

#### Focus

```rust
let autosuggest = what3words_api::Autosuggest::new("filled.count.so").focus(&Coordinates::new(51.520847, -0.195521));
```

#### Clipping

```rust
let autosuggest = what3words_api::Autosuggest::new("filled.count.so")
    .clip_to_country(&["GB","US"])
    .clip_to_bounding_box(&BoundingBox::new(
        51.521251, -0.203586, 51.521251, -0.203586,
    ))
    .clip_to_circle(&Circle::new(51.521251, -0.203586, 1000))
    .clip_to_polygon(&Polygon::new(&[
        Coordinates::new(51.521251, -0.203586),
        Coordinates::new(51.521251, -0.203586),
        Coordinates::new(51.521251, -0.203581),
    ]));
```

Example:

```rust
use what3words_api::{Autosuggest, AutosuggestResult, What3words};

let w3w = What3words::new("YOUR_API_KEY_HERE");

let autosuggest_option = Autosuggest::new("filled.count.so").focus(&Coordinates::new(51.520847, -0.195521));
let autosuggest: AutosuggestResult = w3w.autosuggest(&autosuggest_option);
println!("{:?}", autosuggest.suggestions); // [Suggestion { words: "filled.count.soap", ..., ... }, ..., ...]
```

## Grid Section

Returns a section of the 3m x 3m what3words grid for a bounding box.

Example:

> [!NOTE]
> It is required to specify the type annotation for this function which will allow you to choose between `json` and `geojson` format. Using `GridSection` will use `json` (default) and `GridSectionGeoJson` will use `geojson`.

```rust
use what3words_api::{BoundingBox, GridSection, GridSectionGeoJson, What3words};

let w3w: What3words = What3words::new("YOUR_API_KEY_HERE");

let grid_section_json: GridSection = w3w
        .grid_section::<GridSection>(&BoundingBox::new(52.207988,0.116126,52.208867,0.117540));
println!("{:?}", &grid_section_json.lines[0]); // Line { start: Coordinates { lat: 52.20801, lng: 0.116126 }, end: Coordinates { lat: 52.20801, lng: 0.11754 } }
let grid_section_geojson: GridSectionGeoJson = w3w
    .grid_section::<GridSectionGeoJson>(&BoundingBox::new(52.207988,0.116126,52.208867,0.117540));
println!("{:?}", &grid_section_geojson.features); // [Features { geometry: ..., }, ..., ..., kind: "Feature"]
```

## Available Languages

Retrieves a list of the currently loaded and available 3 word address languages.

The returned payload from the `available-languages` method is described in the [what3words REST API documentation](https://docs.what3words.com/api/v3/#available-languages).

Example:

```rust
use what3words_api::{AvailableLanguages, What3words};

let w3w: What3words = What3words::new("YOUR_API_KEY_HERE");

let available_languages: AvailableLanguages = w3w.available_languages();
println!("{:?}", available_languages.languages); // [Language { code: "en", ..., ... }, ..., ... ]
```

## Helper functions

Below are some helper functions that you can use to identify if a given text is possibly a what3words address.

### did_you_mean

This method takes a string as a parameter and determines if the string passed in is almost in the form of a three word address.

> [!NOTE]
> This function does not validate if it is a real 3WA.

Example:

```rust
use what3words_api::What3words;

let w3w: What3words = What3words::new("YOUR_API_KEY_HERE");

let did_you_mean: bool = w3w.did_you_mean("filled count soap");
println!("{}", did_you_mean); // true
let did_you_mean: bool = w3w.did_you_mean("filled-count-soap");
println!("{}", did_you_mean); // true
let did_you_mean: bool = w3w.did_you_mean("filledcountsoap");
println!("{}", did_you_mean); // false

```

### is_possible_3wa

This method takes a string as a parameter and returns whether the string is in the format of a 3WA (eg “filled.count.soap”). Return type is boolean.

> [!NOTE]
> This function does not validate if it is a real 3WA.

Example:

```rust
use what3words_api::What3words;

let w3w: What3words = What3words::new("YOUR_API_KEY_HERE");

let is_possible_3wa: bool = w3w.is_possible_3wa("filled.count.soap");
println!("{}", is_possible_3wa); // true
let is_possible_3wa: bool = w3w.is_possible_3wa("not a 3wa");
println!("{}", is_possible_3wa); // false
let is_possible_3wa: bool = w3w.is_possible_3wa("not.a 3wa");
println!("{}", is_possible_3wa); // false
```

### find_possible_3wa

This method takes a string as a parameter and searches the string for any possible instances of a 3WA - e.g. "leave in my porch at word.word.word." Likely to be the main method that is called on the delivery notes. Returns an array of matched items. Returns an empty array if no matches are found. NOTE: Does not check if it is an actual existing 3WA.

> [!NOTE]
> This function does not validate if it is a real 3WA.

Example:

```rust
use what3words_api::What3words;

let w3w: What3words = What3words::new("YOUR_API_KEY_HERE");

let find_possible_3wa: Vec<String> = w3w.find_possible_3wa("Please leave by my porch at filled.count.soap");
println!("{:?}", find_possible_3wa); // ["filled.count.soap"]
let find_possible_3wa: Vec<String> = w3w.find_possible_3wa("Please leave by my porch at filled.count.soap or deed.tulip.judge");
println!("{:?}", find_possible_3wa); // ["filled.count.soap", "deed.tulip.judge"]
let find_possible_3wa: Vec<String> = w3w.find_possible_3wa("Please leave by my porch");
println!("{:?}", find_possible_3wa); // []
```

### is_valid_3wa

This method takes a string as a parameter and first passes it through the W3W regex filter (akin to calling `is_possible_3wa()` on the string) and then calls the W3W api to verify it is a real 3WA.

Example:

```rust
use what3words_api::What3words;

let w3w: What3words = What3words::new("YOUR_API_KEY_HERE");

let is_valid_3wa: bool = w3w.is_valid_3wa("filled.count.soap");
println!("{}", is_valid_3wa); // true
let is_valid_3wa: bool = w3w.is_valid_3wa("filled.count.");
println!("{}", is_valid_3wa); // false
let is_valid_3wa: bool = w3w.is_valid_3wa("rust.is.cool");
println!("{}", is_valid_3wa); // false
```

## Examples

Examples can be found in `/examples` directory, simply run the following to try it out:

```bash
# Blocking
$ W3W_API_KEY=<YOUR_API_KEY> cargo run --example sync --features="sync"
```

```bash
# Non-blocking
$ W3W_API_KEY=<YOUR_API_KEY> cargo run --example async --features="async"
```

## Tests

To run the tests, simple run either of the following:

```bash
# Blocking
cargo test --features="sync" --lib
```

```bash
# Non-blocking
cargo test --features="async" --lib
```

## Issues

Found a bug or want to request a new feature? Please let us know by submitting an issue.

## Contributing

Anyone and everyone is welcome to contribute.

1. Fork it (https://github.com/what3words/w3w-rust-wrapper and click "Fork")
2. Clone your fork locally: (`git clone https://github.com/YOUR-USERNAME/w3w-rust-wrapper.git`)
3. Create your feature branch (`git checkout -b my-new-feature`)
4. Commit your changes (`git commit -am 'Add some feature'`)
5. Push to the branch (`git push origin my-new-feature`)
6. Create a Pull Request from your branch to the main repository's main (or appropriate) branch.

## Revision History

- `0.1.1` 14/11/24 - Initial release

## Licensing

The MIT License (MIT)

A copy of the license is available in the repository's [license](LICENSE) file.
