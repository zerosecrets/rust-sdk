#[derive(Debug, serde::Deserialize)]
pub struct FieldFromResponseJson {
    pub name: String,
    pub value: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct SecretFromResponseJson {
    pub fields: Vec<FieldFromResponseJson>,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DataJson {
    pub secrets: Option<Vec<SecretFromResponseJson>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ErrorFromResponseJson {
    pub message: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ResponseJson {
    pub errors: Option<Vec<ErrorFromResponseJson>>,
    pub data: Option<DataJson>,
}
