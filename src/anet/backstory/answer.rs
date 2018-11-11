use ::prelude::*;

gw2rs_id_string!(BackstoryAnswerId);

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BackstoryAnswer {
    id: BackstoryAnswerId,
    title: String,
    description: String,
    journal: String,
    question: BackstoryQuestionId,
    professions: Option<Vec<String>>, // TODO: Un-string this
    races: Option<Vec<String>>,       // TODO: Un-string this
}

impl BackstoryAnswer {
    /// Returns the internal identifier for this backstory question answer.
    pub fn id(&self) -> &BackstoryAnswerId {
        &self.id
    }

    /// Returns the answer's localized title as presented in-game.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the answer's localized description as presented in game.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the answer's localized journal entry text as presented in game.
    pub fn journal(&self) -> &str {
        &self.journal
    }

    /// Returns the answer's related question identifier.
    pub fn question(&self) -> &BackstoryQuestionId {
        &self.question
    }

    /// Returns the set of professions this answer is an available option for.
    pub fn professions(&self) -> Option<&[String]> {
        match self.professions {
            Some(ref v) => Some(v.as_slice()),
            None => None,
        }
    }

    /// Returns the set of races this answer is an available option for.
    pub fn races(&self) -> Option<&[String]> {
        match self.races {
            Some(ref v) => Some(v.as_slice()),
            None => None,
        }
    }
}
