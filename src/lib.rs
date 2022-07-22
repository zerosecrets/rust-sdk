use std::collections::HashMap;
mod json;
mod tests;

/// Zero API client. Instantiate with a token, than call the `.fetch()` method to download secrets.
pub struct Zero {
    api_url: String,
    pick: Vec<String>,
    token: String,
}

/// Constructor arguments. Defines required and optional params.
pub struct Arguments {
    pub token: String,
    pub pick: Option<Vec<String>>,
}

/// The main client for accessing Zero GraphQL API.
///
/// ### Example:
/// ```rust
/// use zero_sdk::{Zero, Arguments};
///
/// let client = Zero::new(Arguments {
///     pick: Some(vec![String::from("my-secret")]),
///     token: String::from("my-zero-token"),
/// })
/// .unwrap();
/// ```
impl Zero {
    /// Set the URL which will be called in fetch(). The method was added mostly for convience of testing.
    pub fn set_api_url(mut self, new_api_url: String) -> Self {
        self.api_url = new_api_url;
        return self;
    }

    // TODO Implement proper error structures with a message and a code
    // TODO Accepts an array of secrets to fetch
    /// Fetch the secrets assigned to the token.
    pub fn fetch(self) -> Result<HashMap<String, HashMap<String, String>>, String> {
        let response = if let Ok(value) = ureq::post(&self.api_url).send_json(serde_json::json!({
            "query":
                format!(
                    "query {{
                        secrets(zeroToken: \"{}\", pick: [{}]) {{
                            name
                            fields {{
                                name value
                            }}
                        }}
                    }}",
                    &self.token,
                    &self
                        .pick
                        .iter()
                        .map(|secret| format!("\"{}\"", &secret))
                        .collect::<Vec<String>>()
                        .join(", "),
                )
        })) {
            value
        } else {
            return Err(String::from("Failed to fetch secrets due to network issue"));
        };

        let response_json = if let Ok(value) = response.into_json::<json::ResponseJson>() {
            value
        } else {
            return Err(String::from("Server returned invalid response"));
        };

        if response_json.errors.is_some() {
            return Err(String::from(&response_json.errors.unwrap()[0].message));
        }

        if response_json.data.is_none() {
            return Err(String::from(
                "Server returned invalid response (no secrets)",
            ));
        }

        // Tranform response to the following structure:
        // {nameOfTheSecret: {fieldOne: "fieldOneValue", fieldTwo: "fieldTwoValue"}}
        Ok(response_json
            .data
            .unwrap()
            .secrets
            .unwrap()
            .iter()
            .map(|secret| {
                (
                    secret.name.to_owned(),
                    HashMap::from_iter(
                        secret
                            .fields
                            .iter()
                            .map(|field| (field.name.to_owned(), field.value.to_owned())),
                    ),
                )
            })
            .collect())
    }

    /// Instantiate new Zero struct. Requires token string to be non empty, other params are optional.
    pub fn new(arguments: Arguments) -> Result<Self, &'static str> {
        if arguments.token == "" {
            return Err("Zero-token is empty");
        }

        Ok(Self {
            api_url: String::from("https://core.tryzero.com/v1/graphql"),
            pick: arguments.pick.unwrap_or(vec![]),
            token: arguments.token,
        })
    }
}
