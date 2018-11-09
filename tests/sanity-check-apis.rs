extern crate futures;
extern crate gw2rs;
extern crate tokio_core;
use gw2rs::prelude::*;
use tokio_core::reactor::Core;

#[test]
fn daily_achievements() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.daily_achievements();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn daily_achievements_tomorrow() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.daily_achievements_tomorrow();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn achievements_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec![2686, 2709, 760];
    let gw2_future = gw2_client.achievements_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[1];
    assert_eq!(result_item.id(), AchievementId(2709));
    assert_eq!(result_item.name(), "League Dominator—Recruit");
}

#[test]
fn achievement_categories() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.achievement_categories();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn achievement_categories_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec![1u64, 3u64, 4u64];
    let gw2_future = gw2_client.achievement_categories_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[1];
    assert_eq!(result_item.name(), "PvP Conqueror");
    assert_eq!(result_item.order(), 5);
}

#[test]
fn achievement_groups() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.achievement_groups();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn achievement_groups_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec![
        "B42E2379-9599-46CA-9D4A-40A27E192BBE",
        "A4ED8379-5B6B-4ECC-B6E1-70C350C902D2",
        "1CAFA333-0C2B-4782-BC4C-7DA30E9CE289",
    ];
    let gw2_future = gw2_client.achievement_groups_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[0];
    assert_eq!(
        result_item.id(),
        &AchievementGroupId::from("B42E2379-9599-46CA-9D4A-40A27E192BBE")
    );
    assert_eq!(result_item.name(), "Path of Fire");
}

#[test]
fn build() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.build();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn backstory_answers() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.backstory_answers();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn backstory_answers_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec!["181-180", "36-153", "7-54"];
    let gw2_future = gw2_client.backstory_answers_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[1];
    assert_eq!(result_item.id(), &BackstoryAnswerId::from("36-153"));
    assert_eq!(
        result_item.journal(),
        "Dwayna, the goddess of healing, blessed me when I was young."
    );
    assert_eq!(
        result_item.races(),
        Some(vec!["Human".to_string()].as_slice())
    );
}

#[test]
fn backstory_questions() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.backstory_questions();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn backstory_questions_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::DE, reactor.handle());
    let ids = vec![10, 21, 32, 186];
    let gw2_future = gw2_client.backstory_questions_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[2];
    assert_eq!(result_item.id(), BackstoryQuestionId(32));
    assert_eq!(result_item.title(), "Mein Traum");
    assert_eq!(
        result_item.answers(),
        vec![
            BackstoryAnswerId::from("32-141"),
            BackstoryAnswerId::from("32-142"),
            BackstoryAnswerId::from("32-143"),
        ].as_slice()
    );
    assert_eq!(result_item.order(), 2);
    assert_eq!(
        result_item.races(),
        Some(vec!["Sylvari".to_string()].as_slice())
    );
}

#[test]
fn cats() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.cats();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn cats_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec![7, 12, 28];
    let gw2_future = gw2_client.cats_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[2];
    assert_eq!(result_item.id(), CatId(28));
    assert_eq!(result_item.hint(), "cold");
}

#[test]
fn colors() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.colors();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn colors_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::DE, reactor.handle());
    let ids = vec![7, 12];
    let gw2_future = gw2_client.colors_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[1];
    assert_eq!(result_item.id(), ColorId(12));
    assert_eq!(result_item.name(), "Lehmziegel");
    assert_eq!(result_item.base_rgb(), &[128, 26, 26]);
    assert_eq!(result_item.item(), ItemId(20_388));
}

#[test]
fn gems_to_gold() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.gems_to_gold(1000);
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn gold_to_gems() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.gold_to_gems(10_000);
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn trading_post_listings() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec![71_334];
    let gw2_future = gw2_client.trading_post_listings(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn trading_post_prices() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec![19_684, 46_741, 46_739];
    let gw2_future = gw2_client.trading_post_prices(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn trading_post_deliveries() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.trading_post_deliveries();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn trading_post_current_buys() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.trading_post_current_buys();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn trading_post_current_sells() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.trading_post_current_sells();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn trading_post_past_buys() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.trading_post_past_buys();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn trading_post_past_sells() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.trading_post_past_sells();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn currencies() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.currencies();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn currencies_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::ES, reactor.handle());
    let ids = vec![12, 24, 39, 41];
    let gw2_future = gw2_client.currencies_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[2];
    assert_eq!(result_item.id(), CurrencyId(39));
    assert_eq!(result_item.name(), "Cristal de Gaets");
    /// a0 -> non-breaking space (U+00A0)
    assert_eq!(
        result_item.description(),
        "Se obtiene de jefes y eventos en incursiones de Path\u{a0}of\u{a0}Fire."
    );
    assert_eq!(result_item.order(), 317);
}

#[test]
fn gliders() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.gliders();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn gliders_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec![1, 5, 19];
    let gw2_future = gw2_client.gliders_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[2];
    assert_eq!(result_item.id(), GliderId(19));
    assert_eq!(result_item.order(), 17);
    assert_eq!(
        result_item.unlock_items(),
        Some(vec![ItemId(78_006)].as_slice())
    );
    assert_eq!(result_item.name(), "Ironclad Glider");
    assert_eq!(
        result_item.default_dyes(),
        vec![ColorId(5), ColorId(12)].as_slice()
    );
}

#[test]
fn legends() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.legends();
    let api_results = reactor.run(gw2_future).unwrap();
    assert_eq!(api_results[3], LegendId::from("Legend4"));
}

#[test]
fn legends_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec!["Legend3", "Legend5"];
    let gw2_future = gw2_client.legends_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[1];
    assert_eq!(result_item.id(), &LegendId::from("Legend5"));
    assert_eq!(result_item.heal(), SkillId(45_686));
    assert_eq!(result_item.utilities()[1], SkillId(40_485));
}

#[test]
fn specializations() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.specializations();
    let api_results = reactor.run(gw2_future).unwrap();
    assert_eq!(api_results[5], SpecializationId(6));
}

#[test]
fn specializations_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::DE, reactor.handle());
    let ids = vec![6, 12, 18];
    let gw2_future = gw2_client.specializations_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[1];
    assert_eq!(result_item.id(), SpecializationId(12));
    assert_eq!(result_item.name(), "Erlösung");
    assert_eq!(result_item.profession(), "Revenant");
    assert_eq!(result_item.elite(), false);
    assert_eq!(
        result_item.minor_traits(),
        vec![TraitId(1816), TraitId(1821), TraitId(1814)].as_slice()
    );
}

#[test]
fn stories() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::DE, reactor.handle());
    let gw2_future = gw2_client.stories();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn stories_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::DE, reactor.handle());
    let ids = vec![17, 25, 46, 72];
    let gw2_future = gw2_client.stories_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[2];
    assert_eq!(result_item.id(), StoryId(46));
    assert_eq!(
        result_item.season(),
        &StorySeasonId::from("09766A86-D88D-4DF2-9385-259E9A8CA583")
    );
    assert_eq!(result_item.name(), "1. Aus den Schatten");
    assert_eq!(
        result_item.description(),
        "Die Ereignisse des Krieges gegen Mordremoth hatten weitreichende Auswirkungen."
    );
    assert_eq!(result_item.timeline(), "1329 n. E.");
    assert_eq!(result_item.chapters(), vec![].as_slice());
    assert_eq!(
        result_item.flags(),
        Some(vec![String::from("RequiresUnlock")].as_slice())
    );
}

#[test]
fn story_seasons() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::DE, reactor.handle());
    let gw2_future = gw2_client.story_seasons();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn story_seasons_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::DE, reactor.handle());
    let ids = vec![
        "C22AFD21-667A-4AA8-8210-AC74EAEE58BB",
        "215AAA0F-CDAC-4F93-86DA-C155A99B5784",
        "09766A86-D88D-4DF2-9385-259E9A8CA583",
    ];
    let gw2_future = gw2_client.story_seasons_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[2];
    assert_eq!(
        result_item.id(),
        &StorySeasonId::from("09766A86-D88D-4DF2-9385-259E9A8CA583")
    );
    assert_eq!(result_item.name(), "Staffel 3 der Lebendigen Welt");
    assert_eq!(result_item.order(), 5);
    assert_eq!(
        result_item.stories(),
        vec![
            StoryId(65),
            StoryId(66),
            StoryId(64),
            StoryId(46),
            StoryId(56),
            StoryId(63),
        ].as_slice()
    );
}

#[test]
fn worlds() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.worlds();
    let api_results = reactor.run(gw2_future).unwrap();
    assert_eq!(api_results[3], WorldId(1004));
}

#[test]
fn worlds_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::DE, reactor.handle());
    let ids = vec![1004, 1005];
    let gw2_future = gw2_client.worlds_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[0];
    assert_eq!(result_item.id(), WorldId(1004));
    assert_eq!(result_item.name(), "Steinkreis von Denravi");
}

#[test]
fn wvw_abilities() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::DE, reactor.handle());
    let gw2_future = gw2_client.wvw_abilities();
    let api_results = reactor.run(gw2_future).unwrap();
}

#[test]
fn wvw_abilities_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::DE, reactor.handle());
    let ids = vec![5, 12, 18];
    let gw2_future = gw2_client.wvw_abilities_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[2];
    assert_eq!(result_item.id(), WVWAbilityId(18));
    assert_eq!(result_item.name(), "Trébuchetbeherrschung");
    assert_eq!(result_item.ranks()[3].cost(), 15);
}

#[test]
fn objectives() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.objectives();
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[2];
    assert_eq!(result_item, &ObjectiveId::from("1102-99"));
}

#[test]
fn objectives_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec!["38-124", "96-33", "38-15"];
    let gw2_future = gw2_client.objectives_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let results_item = api_results.get(1).expect("Failed to unwrap objective");
    assert_eq!(results_item.name(), "Ascension Bay");
}

#[test]
fn wvw_matches() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.wvw_matches();
    let api_results = reactor.run(gw2_future).unwrap();
    let results_item = &api_results[2];
}

#[test]
fn wvw_ranks() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.wvw_ranks();
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[100];
    assert_eq!(result_item, &WVWRankId(101));
}

#[test]
fn wvw_ranks_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec![42, 45];
    let gw2_future = gw2_client.wvw_ranks_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[1];
    assert_eq!(result_item.id(), WVWRankId(45));
    assert_eq!(result_item.title(), "Silver Legend");
    assert_eq!(result_item.min_rank(), 1320);
}

#[test]
fn wvw_upgrades() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.wvw_upgrades();
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[40];
    assert_eq!(result_item, &UpgradeId(50));
}

#[test]
fn wvw_upgrades_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec![32];

    let gw2_future = gw2_client.wvw_upgrades_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[0];
    assert_eq!(result_item.id(), UpgradeId(32));
    assert_eq!(
        result_item.tiers()[1].upgrades()[0].name(),
        "Reinforced Walls"
    );
}

#[test]
fn titles() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let gw2_future = gw2_client.titles();
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[13];
    assert_eq!(result_item, &TitleId(15));
}

#[test]
fn titles_by_ids() {
    let mut reactor = Core::new().unwrap();
    let token = std::env::var("GW2RS_TEST_API_KEY")
        .expect("The environment variable GW2RS_TEST_API_KEY not set");
    let gw2_client = GW2::new(token, Locale::EN, reactor.handle());
    let ids = vec![251, 261, 264];
    let gw2_future = gw2_client.titles_by_ids(&ids);
    let api_results = reactor.run(gw2_future).unwrap();
    let result_item = &api_results[2];
    assert_eq!(result_item.id(), TitleId(264));
    assert_eq!(result_item.name(), "Committed");
    assert_eq!(
        result_item.achievements(),
        vec![AchievementId(3287)].as_slice()
    );
}


// #[test]
// fn continents() {
//     unimplemented!();
// }

// #[test]
// fn continents_by_ids() {
//     unimplemented!();
// }