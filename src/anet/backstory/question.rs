use ::prelude::*;

gw2rs_id_u64!(BackstoryQuestionId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BackstoryQuestion {
    pub id: BackstoryQuestionId,
    pub title: String,
    pub description: String,
    pub answers: Vec<BackstoryAnswerId>,
    pub order: u64,
    pub races: Option<Vec<String>>,       // TODO: Un-string this
    pub professions: Option<Vec<String>>, // TODO: Un-string this
}
