use futures::Future;
use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;
use std::ops::Deref;

use crate::prelude::*;

/// The Guild Wars 2 API Wrapper.
///
/// Structs of this type provide rust functions mapping to ArenaNet GuildWars 2 API (Version 2).
/// The struct retains the API key to use for authenticated requests, along with the desired Locale.
///
/// Almost every method used to interact with the API through this crate is a self method on GW2.
/// Mutator functions have been provided for scenarios where changing the API key or locale is
/// desired, but since the wrapper keeps very little state, it is usually easier to just spawn new
/// instances for one-off calls.
///
/// All exposed API functionality is, therefore, built on the requirement that only an immutable
/// reference is necessary once initialized.
#[derive(Debug)]
pub struct GW2 {
    /// The Guild Wars 2 API key to use for authenticated requests.
    api_key: String,
    /// The locale to use for localized requests.
    locale: Locale,
    /// The Tokio Core Handle for the API.
    tokio_handle: Handle,
    /// The internal HTTP client for API requests
    http_client: Client<HttpsConnector<HttpConnector>>,
}

impl GW2 {
    /// Initializes the GW2 API interface with a given API token.
    pub fn new<I>(token: I, locale: Locale, handle: Handle) -> GW2
    where
        I: Into<String>,
    {
        trace!("Initializing a new GW2 struct.");

        let http_client = Client::configure()
            .connector(
                HttpsConnector::new(8, &handle).expect("Failed to initialize SSL connector."),
            )
            .build(&handle);

        GW2 {
            api_key: token.into(),
            locale: locale,
            tokio_handle: handle,
            http_client: http_client,
        }
    }

    /// Returns a reference to the current Locale.
    pub fn locale(&self) -> Locale {
        self.locale
    }

    /// Updates the current locale.
    pub fn set_locale(&mut self, l: Locale) {
        self.locale = l;
    }

    /// Returns a reference to the current API key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Updates the current API Key.
    ///
    /// # Note
    /// This method is provided for convenience. In usual workflows however it will probably make
    /// more sense to just instantiate a new GW2 struct, as there isnt really any state to
    /// lose. However, this is handy for situations where you're keeping a global struct, or
    /// need to be conservative regarding allocations being made.
    pub fn set_api_key(&mut self, key: String) {
        self.api_key = key;
    }

    /// Grants the caller an immutable reference to the internal HTTP Client.
    /// Useful for queueing up http requests as work to the internal Tokio Core.
    pub fn get_http_client(&self) -> &Client<HttpsConnector<HttpConnector>> {
        &self.http_client
    }

    /// Grants the caller an immutable reference to the internal Tokio handle.
    pub fn get_tokio_handle(&self) -> &Handle {
        &self.tokio_handle
    }

    /// Retrieves the current build information.
    gw2rs_api_wrapper![ NIL, build, Build, Endpoint::Build ];

    /// Retrieves backstory answer identifiers.
    gw2rs_api_wrapper![ NIL, backstory_answers, Vec<BackstoryAnswerId>, Endpoint::BackstoryAnswers ];

    /// Retrieves backstory answers using the given list of identifiers.
    gw2rs_api_wrapper![ STR_LOC, backstory_answers_by_ids, Vec<BackstoryAnswer>, Endpoint::BackstoryAnswers ];

    /// Retrieves backstory question identifiers.
    gw2rs_api_wrapper![ NIL, backstory_questions, Vec<BackstoryQuestionId>, Endpoint::BackstoryQuestions ];

    /// Retrieves backstory questions using the given list of identifiers.
    gw2rs_api_wrapper![ INT_LOC, backstory_questions_by_ids, Vec<BackstoryQuestion>, Endpoint::BackstoryQuestions ];

    /// Retrieves cat identifiers.
    gw2rs_api_wrapper![ NIL, cats, Vec<CatId>, Endpoint::Cats ];

    /// Retrieves cats using a list of identifiers.
    gw2rs_api_wrapper![ INT, cats_by_ids, Vec<Cat>, Endpoint::Cats ];

    /// Retrieves today's daily achievements.
    gw2rs_api_wrapper![ NIL, daily_achievements, DailyAchievements, Endpoint::AchievementsDaily ];

    /// Retrieves tomorrows daily achievements.
    gw2rs_api_wrapper![ NIL, daily_achievements_tomorrow, DailyAchievements, Endpoint::AchievementsDailyTomorrow ];

    /// Fetches achievements using a slice of u64 identifiers.
    gw2rs_api_wrapper![ INT_LOC, achievements_by_ids, Vec<Achievement>, Endpoint::Achievements ];

    /// Retrieves the identifiers for valid achievement categories.
    gw2rs_api_wrapper![ NIL, achievement_categories, Vec<u64>, Endpoint::AchievementsCategories ];

    /// Retrieves achievement categories with the given identifiers.
    gw2rs_api_wrapper![ INT_LOC, achievement_categories_by_ids, Vec<AchievementCategory>, Endpoint::AchievementsCategories ];

    /// Retrieves valid achievement group identifiers.
    gw2rs_api_wrapper![ NIL, achievement_groups, Vec<String>, Endpoint::AchievementsGroups ];

    /// Retrieves achievement groups with the given identifiers.
    gw2rs_api_wrapper![ STR_LOC, achievement_groups_by_ids, Vec<AchievementGroup>, Endpoint::AchievementsGroups ];

    /// Retrieves valid color identifiers.
    gw2rs_api_wrapper![ NIL, colors, Vec<ColorId>, Endpoint::Colors ];

    /// Retrieves a list of Colors using the given list of identifiers.
    gw2rs_api_wrapper![ INT_LOC, colors_by_ids, Vec<Color>, Endpoint::Colors ];

    /// Fetches the current going rate converting the given number of gems into gold.
    gw2rs_api_wrapper![ QTY, gems_to_gold, CurrencyConversion, Endpoint::CommerceExchangeGems ];

    /// Fetches the current conversion rate converting gold into gems.
    gw2rs_api_wrapper![ QTY, gold_to_gems, CurrencyConversion, Endpoint::CommerceExchangeCoins ];

    /// Fetches trading post sales data for the given item identifiers.
    gw2rs_api_wrapper![ INT, trading_post_listings, Vec<Listing>, Endpoint::CommerceListings ];

    /// Fetches trading post data data on the prices of the given items.
    gw2rs_api_wrapper![ INT, trading_post_prices, Vec<Prices>, Endpoint::CommercePrices ];

    /// Fetches the trading post deliveries for an account.
    gw2rs_api_wrapper![ AUTH, trading_post_deliveries, TradingPostDeliveries, Endpoint::CommerceDelivery ];

    /// Fetches the current buy orders for an account.
    gw2rs_api_wrapper![ AUTH, trading_post_current_buys, Vec<TradingPostTransaction>, Endpoint::CommerceTransactionsCurrentBuys ];

    /// Fetches the current sell orders for an account.
    gw2rs_api_wrapper![ AUTH, trading_post_current_sells, Vec<TradingPostTransaction>, Endpoint::CommerceTransactionsCurrentSells ];

    /// Fetches historical buy orders for an account.
    gw2rs_api_wrapper![ AUTH, trading_post_past_buys, Vec<TradingPostTransaction>, Endpoint::CommerceTransactionsHistoryBuys ];

    /// Fetches historical sell orders for an account.
    gw2rs_api_wrapper![ AUTH, trading_post_past_sells, Vec<TradingPostTransaction>, Endpoint::CommerceTransactionsHistorySells ];

    /// Eetrieves a list of Currency identifiers.
    gw2rs_api_wrapper![ NIL, currencies, Vec<CurrencyId>, Endpoint::Currencies ];

    /// Retrieves a list of Currency objects using the given list of identifiers.
    gw2rs_api_wrapper![ INT_LOC, currencies_by_ids, Vec<Currency>, Endpoint::Currencies ];

    /// Retrieves a list of Glider identifiers.
    gw2rs_api_wrapper![ NIL, gliders, Vec<GliderId>, Endpoint::Gliders ];

    /// Retrieves a list of GW2 Gliders.
    gw2rs_api_wrapper![ INT_LOC, gliders_by_ids, Vec<Glider>, Endpoint::Gliders ];

    /// Fetches a list of GW2 Revenant Legend identifiers.
    gw2rs_api_wrapper![ NIL, legends, Vec<LegendId>, Endpoint::Legends ];

    /// Fetches a list of GW2 Revenant Legends.
    gw2rs_api_wrapper![ STR_LOC, legends_by_ids, Vec<Legend>, Endpoint::Legends ];

    /// Fetches character title identifiers.
    gw2rs_api_wrapper![ NIL, titles, Vec<TitleId>, Endpoint::Titles ];

    /// Fetches specificied character titles.
    gw2rs_api_wrapper![ INT_LOC, titles_by_ids, Vec<Title>, Endpoint::Titles ];

    /// Fetches information on the provided access token.
    gw2rs_api_wrapper![ AUTH_LOC, token_info, TokenInfo, Endpoint::TokenInfo ];

    /// Fetches the list of Specialization identifiers.
    gw2rs_api_wrapper![ NIL, specializations, Vec<SpecializationId>, Endpoint::Specializations ];

    /// Fetches a list of class specializations.
    gw2rs_api_wrapper![ INT_LOC, specializations_by_ids, Vec<Specialization>, Endpoint::Specializations ];

    /// Fetches the list of Story identifiers.
    gw2rs_api_wrapper![ NIL, stories, Vec<StoryId>, Endpoint::Stories ];

    /// Fetches a list of stories with identifiers matching the input set.
    gw2rs_api_wrapper![ INT_LOC, stories_by_ids, Vec<Story>, Endpoint::Stories ];

    /// Fetches the list of story season identifiers.
    gw2rs_api_wrapper![ NIL, story_seasons, Vec<StorySeasonId>, Endpoint::StorySeasons ];

    /// Fetches a list of StorySeason's with identifiers matching the input set.
    gw2rs_api_wrapper![ STR_LOC, story_seasons_by_ids, Vec<StorySeason>, Endpoint::StorySeasons ];

    /// Fetches a list of world (server) identifiers.
    gw2rs_api_wrapper![ NIL, worlds, Vec<WorldId>, Endpoint::Worlds ];

    /// Fetches world data for the desired world identifiers.
    gw2rs_api_wrapper![ INT_LOC, worlds_by_ids, Vec<World>, Endpoint::Worlds ];

    /// Fetches World versus World Ability identifiers.
    gw2rs_api_wrapper![ NIL, wvw_abilities, Vec<WVWAbilityId>, Endpoint::WVWAbilities ];

    /// Fetches World versus World abilities with the given identifiers.
    gw2rs_api_wrapper![ INT_LOC, wvw_abilities_by_ids, Vec<WVWAbility>, Endpoint::WVWAbilities ];

    /// Returns a list of World vs World Objective identifiers.
    gw2rs_api_wrapper![ NIL, objectives, Vec<ObjectiveId>, Endpoint::WVWObjectives ];

    // /// Returns a list of World vs World objectives with the argument identifiers.
    gw2rs_api_wrapper![ STR_LOC, objectives_by_ids, Vec<Objective>, Endpoint::WVWObjectives ];

    // /// Fetches World versus World match list for both North America and Europe.
    gw2rs_api_wrapper![ NIL, wvw_matches, Vec<WVWMatchId>, Endpoint::WVWMatches ];

    /// Fetches world versus world match data for the given matchups.
    gw2rs_api_wrapper![ STR, wvw_matches_by_ids, Vec<WVWMatch>, Endpoint::WVWMatches ];

    ///  Returns a list of World vs World Rank identifiers.
    gw2rs_api_wrapper![ NIL, wvw_ranks, Vec<WVWRankId>, Endpoint::WVWRanks ];
	
	/// Returns a list of World vs World Ranks given a set of identifiers.
    gw2rs_api_wrapper![ INT_LOC, wvw_ranks_by_ids, Vec<WVWRank>, Endpoint::WVWRanks ];

    ///  Returns a list of World vs World objective upgrade identifiers.
    gw2rs_api_wrapper![ NIL, wvw_upgrades, Vec<UpgradeId>, Endpoint::WVWUpgrades ];

    /// Returns a list of World vs World objective upgrades given a set of identifiers.
    gw2rs_api_wrapper![ INT_LOC, wvw_upgrades_by_ids, Vec<Upgrade>, Endpoint::WVWUpgrades ];
}