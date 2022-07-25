mod mock_and_fetch;
#[cfg(test)]
#[test]
pub fn it_requires_token_to_be_non_empty() {
    if let Err(_) = super::Zero::new(super::Arguments {
        pick: Some(vec![]),
        token: String::from("token"),
        caller_name: None,
    }) {
        panic!("Instantion with a valid token string failed")
    }

    if let Ok(_) = super::Zero::new(super::Arguments {
        pick: Some(vec![]),
        token: String::from(""),
        caller_name: None,
    }) {
        panic!("No error thrown during instantiation with empty token string")
    }
}

#[test]
pub fn it_sends_empty_pick_if_it_wasnt_provided() {
    let server = httpmock::prelude::MockServer::start();

    let secrets = mock_and_fetch::mock_and_fetch(
        mock_and_fetch::Arguments { server },
        Some(mock_and_fetch::Options {
            is_pick_empty: true,
            is_response_failed: false,
        }),
    );

    assert_eq!(
        secrets.unwrap().get("aws").unwrap().get("secret").unwrap(),
        "b"
    );
}

#[test]
pub fn it_sends_provided_pick() {
    let server = httpmock::prelude::MockServer::start();

    let secrets = mock_and_fetch::mock_and_fetch(mock_and_fetch::Arguments { server }, None);

    assert!(secrets.is_ok());
}

#[test]
pub fn it_returns_err_in_case_of_graphql_api_error() {
    let server = httpmock::prelude::MockServer::start();

    let secrets = mock_and_fetch::mock_and_fetch(
        mock_and_fetch::Arguments { server },
        Some(mock_and_fetch::Options {
            is_pick_empty: true,
            is_response_failed: true,
        }),
    );

    assert!(secrets.is_err());
}
