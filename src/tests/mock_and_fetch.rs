pub struct Arguments<'a> {
    pub server: &'a httpmock::prelude::MockServer,
}

pub struct Options {
    pub is_caller_name_empty: bool,
    pub is_pick_empty: bool,
    pub is_response_failed: bool,
}

/// Mock the server and call it with Zero.fetch(). Returns the fetched secrets.
pub fn mock_and_fetch(
    arguments: Arguments,
    options: Option<Options>,
) -> Result<std::collections::HashMap<String, std::collections::HashMap<String, String>>, String> {
    const TOKEN: &str = "token";
    const CALLER_NAME: &str = "this-is-a-caller";

    let caller_name_query = if !options.is_none() && options.as_ref().unwrap().is_caller_name_empty {
        String::from("")
    } else {
        format!(", callerName: \\\"{}\\\"", CALLER_NAME)
    };

    let pick;
    let pick_query;
    let response_body;

    // Empty response, depending on the flag
    if !options.is_none() && options.as_ref().unwrap().is_pick_empty {
        pick = Some(vec![]);
        pick_query = "";
    } else {
        pick = Some(vec![String::from("aws"), String::from("stripe")]);
        pick_query = "\\\"aws\\\", \\\"stripe\\\"";
    }

    // Failed response, depending on the flag
    if !options.is_none() && options.as_ref().unwrap().is_response_failed {
        response_body = serde_json::json!({
            "data": null,

            "errors": [
                {
                    "message": "Could not establish connection with database",
                    "locations": [{"line": 2, "column": 2}],
                    "path": ["secrets"],
                    "extensions": {"internal_error": "Error occurred while creating a new object: error connecting to server: Connection refused (os error 61)"}
                }
            ]
        });
    } else {
        response_body = serde_json::json!({"data": {
            "secrets": [
                {"name": "aws", "fields": [{"name": "key", "value": "a"}, {"name": "secret", "value": "b"}]},
            ]
        }});
    }

    // Generate mock
    let mock = arguments.server.mock(|when, then| {
        when.method(httpmock::prelude::POST)
            .path("/v1/graphql")
            .body_contains(&format!(
                "secrets(zeroToken: \\\"{}\\\", pick: [{}]{})",
                TOKEN, pick_query, caller_name_query,
            ));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(response_body);
    });

    // Instantiate Zero and fetch the secrets
    let secrets = super::super::Zero::new(super::super::Arguments {
        pick,
        token: String::from(TOKEN),
        caller_name: if caller_name_query.is_empty() { None } else { Some(String::from(CALLER_NAME)) },
    })
    .unwrap()
    .set_api_url(arguments.server.url("/v1/graphql"))
    .fetch();

    mock.assert();
    return secrets;
}
