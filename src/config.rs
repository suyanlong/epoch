#[derive(Debug, Deserialize)]
pub struct Config {
    pub mysql: Option<String>,
    pub node_id: Option<usize>,
    //    email: Vec<String>,
}
