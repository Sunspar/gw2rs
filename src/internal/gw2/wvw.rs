use futures::Future;

use ::prelude::*;

impl GW2 {
    /// Fetches World versus World Ability identifiers.
    #[must_use]
    pub fn wvw_abilities(&self) -> impl Future<Item = Vec<WVWAbilityId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::WVWAbilities;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<WVWAbilityId>>(&res))
    }

    /// Fetches World versus World abilities with the given identifiers.
    #[must_use]
    pub fn wvw_abilities_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<WVWAbility>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::WVWAbilities;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<WVWAbility>>(&res))
    }

    /// Returns a list of World vs World Objective identifiers.
    #[must_use]
    pub fn objectives(&self) -> impl Future<Item = Vec<ObjectiveId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::WVWObjectives;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<ObjectiveId>>(&res))
    }

    // /// Returns a list of World vs World objectives with the argument identifiers.
    #[must_use]
    pub fn objectives_by_ids<'a>(&self, ids: &'a [&str]) -> impl Future<Item = Vec<Objective>, Error = APIError> + 'a {
        let client = self.get_http_client();   
        let locale = self.locale(); 
        let endpoint = Endpoint::WVWObjectives;
        ::internal::http::request_with_string_ids(client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Objective>>(&res))
    }

    // /// Fetches World versus World match list for both North America and Europe.
    #[must_use]
   pub fn wvw_matches(&self) -> impl Future<Item = Vec<WVWMatchId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::WVWMatches;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<WVWMatchId>>(&res))
    }

    /// Fetches world versus world match data for the given matchups.
    #[must_use]
    pub fn wvw_matches_by_ids<'a>(&self, ids: &'a [&str]) -> impl Future<Item = Vec<WVWMatch>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let endpoint = Endpoint::WVWMatches;
        ::internal::http::request_with_string_ids(client, endpoint, Some(ids), None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<WVWMatch>>(&res))
    }

    ///  Returns a list of World vs World Rank identifiers.
    #[must_use]
    pub fn wvw_ranks(&self) -> impl Future<Item = Vec<WVWRankId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::WVWRanks;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<WVWRankId>>(&res))
    }

	/// Returns a list of World vs World Ranks given a set of identifiers.
    #[must_use]
    pub fn wvw_ranks_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<WVWRank>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::WVWRanks;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<WVWRank>>(&res))
    }

    ///  Returns a list of World vs World objective upgrade identifiers.
    #[must_use]
    pub fn wvw_upgrades(&self) -> impl Future<Item = Vec<UpgradeId>, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::WVWUpgrades;
        ::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<UpgradeId>>(&res))
    }

    /// Returns a list of World vs World objective upgrades given a set of identifiers.
    #[must_use]
    pub fn wvw_upgrades_by_ids<'a>(&self, ids: &'a [u64]) -> impl Future<Item = Vec<Upgrade>, Error = APIError> + 'a {
        let client = self.get_http_client();
        let locale = self.locale();
        let endpoint = Endpoint::WVWUpgrades;
        ::internal::http::request_with_numeric_ids( client, endpoint, Some(ids), None, Some(locale), None)
            .and_then(|res| ::internal::http::convert_to_struct::<Vec<Upgrade>>(&res))
    }
}