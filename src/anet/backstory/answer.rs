use ::prelude::*;

gw2rs_id_string!(BackstoryAnswerId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BackstoryAnswer {
    pub id: BackstoryAnswerId,
    pub title: String,
    pub description: String,
    pub journal: String,
    pub question: BackstoryQuestionId,
    pub professions: Option<Vec<String>>, // TODO: Un-string this
    pub races: Option<Vec<String>>,       // TODO: Un-string this
}
