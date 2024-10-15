![what3words](https://what3words.com/assets/images/w3w_square_red.png 'what3words')

# w3w-rust-wrapper

A rust library to use the [what3words v3 API](https://docs.what3words.com/api/v3/).

API methods are grouped into a single service object which can be centrally managed by a `What3wordsV3`instance.

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
$ cargo add what3words
```

# Usage

## Initialisation

Once you have the API Key, you can initialise the wrapper like this:

```rust
use what3words::What3words;

let wrapper = What3words::new("YOUR_API_KEY_HERE");
```

> [!NOTE]
> The service provides asynchronous functions therefore you would need to install `tokio` or any other asynchronous programming runtime.

### Optional

You can also pass a different hostname if you have your own self-hosted what3words API.

```rust
use what3words::What3words;

let wrapper = What3words::new("YOUR_API_KEY_HERE").hostname("https://your.what3words.api/v3");
```

You can also set configure your own headers:

```rust
use what3words::What3words;

let wrapper = What3words::new("YOUR_API_KEY_HERE").header("X-Foo", "Bar");
```

## Convert To Coordinates

This function takes the words parameter as a string of 3 words `'filled.count.soap'`

The returned payload from the `convert-to-coordinates` method is described in the [what3words REST API documentation](https://docs.what3words.com/api/v3/#convert-to-coordinates).

Example:

```rust
use what3words::What3words;

let address = What3words::new("YOUR_API_KEY_HERE").convert_to_coordinates("filled.count.soap").await?;
println!("{:?}", address.coordinates); // Coordinates { lat: 51.520847, lng: -0.195521 }

```

## Convert To 3 Word Address

This function takes the latitude and longitude:

- 2 parameters: `lat=0.1234`, `lng=1.5678`

The returned payload from the `convert-to-3wa` method is described in the [what3words REST API documentation](https://docs.what3words.com/api/v3/#convert-to-3wa).

Example:

```rust
use what3words::{Coordinates, What3words};

let coordinates = Coordinates { lat: 51.520847, lng: -0.195521  };
let address = What3words::new("YOUR_API_KEY_HERE").convert_to_3wa(&coordinates).await?;
println!("{:?}", address.words); // "filled.count.soap"

```
