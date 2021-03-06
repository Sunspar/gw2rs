use futures::Future;

use ::prelude::*;

impl GW2 {
    /// Retrieves today's daily achievements.
    #[must_use]
    pub fn daily_achievements(&self) -> impl Future<Item = DailyAchievements, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::AchievementsDaily;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<DailyAchievements>(&res))
    }

    /// Retrieves tomorrows daily achievements.
    #[must_use]
    pub fn daily_achievements_tomorrow(&self) -> impl Future<Item = DailyAchievements, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::AchievementsDailyTomorrow;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<DailyAchievements>(&res))
    }

    /// Fetches achievements using a slice of u64 identifiers.
    #[must_use]
    pub fn achievements_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Achievement>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::Achievements;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Achievement>>(&res))
    }

    /// Retrieves the identifiers for valid achievement categories.
    #[must_use]
    pub fn achievement_categories(&self) -> impl Future<Item = Vec<u64>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::AchievementsCategories;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<u64>>(&res))
    }

    /// Retrieves achievement categories with the given identifiers.
    #[must_use]
    pub fn achievement_categories_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<AchievementCategory>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::AchievementsCategories;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<AchievementCategory>>(&res))
    }
    
    /// Retrieves valid achievement group identifiers.
    #[must_use]
    pub fn achievement_groups(&self) -> impl Future<Item = Vec<AchievementGroupId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::AchievementsGroups;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<AchievementGroupId>>(&res))
    }
    
    /// Retrieves achievement groups with the given identifiers.
    #[must_use]
    pub fn achievement_groups_by_ids<'a>(&self, ids: &'a [&str]) -> impl Future<Item = Vec<AchievementGroup>, Error = APIError> + 'a {
        let client = self.get_http_client();   
        let locale = self.locale(); 
        let endpoint = Endpoint::AchievementsGroups;
        ::internal::http::request_with_string_ids(client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<AchievementGroup>>(&res))
    }
}