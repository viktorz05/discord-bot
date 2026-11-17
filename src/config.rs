pub struct Config {
    pub priv_key: String,
    pub sheet_id: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            priv_key: String::from("credentials.json"),
            sheet_id: String::from("sheet_id")
        }
    }
}