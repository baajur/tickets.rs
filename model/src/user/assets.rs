use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
    pub large_image: Option<String>, // *usually* a snowflake
    pub large_text: Option<String>,
    pub small_image: Option<String>, // *usually* a snowflake
    pub small_text: Option<String>,
}
