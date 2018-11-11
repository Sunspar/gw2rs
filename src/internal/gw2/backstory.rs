use futures::Future;

use crate::prelude::*;

impl GW2 {
    /// Retrieves backstory answer identifiers.
    pub fn backstory_answers(&self) -> impl Future<Item = Vec<BackstoryAnswerId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::BackstoryAnswers;
        crate::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<BackstoryAnswerId>>(&res))
    }

    /// Retrieves backstory answers using the given list of identifiers.
    pub fn backstory_answers_by_ids<'a>(&self, ids: &'a [&str]) -> impl Future<Item = Vec<BackstoryAnswer>, Error = APIError> + 'a {
        let client = self.get_http_client();   
        let locale = self.locale(); 
        let endpoint = Endpoint::BackstoryAnswers;
        crate::internal::http::request_with_string_ids(client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<BackstoryAnswer>>(&res))
    }
    
    /// Retrieves backstory question identifiers.
    pub fn backstory_questions(&self) -> impl Future<Item = Vec<BackstoryQuestionId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::BackstoryQuestions;
        crate::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<BackstoryQuestionId>>(&res))
    }

    /// Retrieves backstory questions using the given list of identifiers.
    pub fn backstory_questions_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<BackstoryQuestion>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::BackstoryQuestions;
        crate::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Vec<BackstoryQuestion>>(&res))
    }
}