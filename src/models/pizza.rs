
use serde::{Serialize, Deserialize};

use validator::Validate;
#[derive(Serialize, Deserialize, Validate)]
pub struct BuyPizzaRequest {
    #[validate(length(min =1 ,message="pizza name required "))]
    pub pizza_name: String,

}

