use crate::prelude::*;

gw2rs_id_u64!(BackstoryQuestionId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BackstoryQuestion {
    id: BackstoryQuestionId,
    title: String,
    description: String,
    answers: Vec<BackstoryAnswerId>,
    order: u64,
    races: Option<Vec<String>>,       // TODO: Un-string this
    professions: Option<Vec<String>>, // TODO: Un-string this
}

impl BackstoryQuestion {
    /// Returns the internal identifier for the given question.
    pub fn id(&self) -> BackstoryQuestionId {
        self.id
    }

    /// Returns the question's localized title as presented in-game.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the question's localized description as presented in-game.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the questions set of available answer identifiers.
    pub fn answers(&self) -> &[BackstoryAnswerId] {
        self.answers.as_slice()
    }

    /// Returns the natural ordering of this question among others asked during
    /// the character creation process.
    pub fn order(&self) -> u64 {
        self.order
    }

    /// Returns the set of races this question is asked for, if restricted.
    pub fn races(&self) -> Option<&[String]> {
        match self.races {
            Some(ref v) => Some(v.as_slice()),
            None => None,
        }
    }

    /// Returns the set of professions this question is asked for, if restricted.
    pub fn professions(&self) -> Option<&[String]> {
        match self.professions {
            Some(ref v) => Some(v.as_slice()),
            None => None,
        }
    }
}
