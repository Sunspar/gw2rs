extern crate gw2rs;
use gw2rs::prelude::GoldValue;

#[test]
fn positive_a_plus_positive_b() {
    let a = GoldValue::from(15_000);
    let b = GoldValue::from(14_380);
    assert_eq!(a + b, GoldValue::from(29_380));
}

#[test]
fn positive_a_plus_negative_b_with_a_coins_gt_b_coins() {
    let a = GoldValue::from(15_000);
    let b = GoldValue::from(-13_985);
    assert_eq!(a + b, GoldValue::from(1_015));
}

#[test]
fn positive_a_plus_negative_b_with_a_coins_lt_b_coins() {
    let a = GoldValue::from(12_123);
    let b = GoldValue::from(-16_153);
    assert_eq!(a + b, GoldValue::from(-4_030));
}

#[test]
fn positive_a_plus_negative_b_with_a_coins_eq_b_coins() {
    let a = GoldValue::from(185_723);
    let b = GoldValue::from(-185_723);
    assert_eq!(a + b, GoldValue::from(0));
}

#[test]
fn negative_a_plus_positive_b_with_a_coins_gt_b_coins() {
    let a = GoldValue::from(-382_850);
    let b = GoldValue::from(185_294);
    assert_eq!(a + b, GoldValue::from(-197_556));
}

#[test]
fn negative_a_plus_positive_b_with_a_coins_lt_b_coins() {
    let a = GoldValue::from(-195_823);
    let b = GoldValue::from(19_973);
    assert_eq!(a + b, GoldValue::from(-175_850));
}

#[test]
fn negative_a_plus_positive_b_with_a_coins_eq_b_coins() {
    let a = GoldValue::from(-19_434);
    let b = GoldValue::from(19_434);
    assert_eq!(a + b, GoldValue::from(0));
}

#[test]
fn negative_a_plus_negative_b() {
    let a = GoldValue::from(-142_196);
    let b = GoldValue::from(-385_188);
    assert_eq!(a + b, GoldValue::from(-527_384));
}

#[test]
fn positive_a_sub_positive_b() {
    let a = GoldValue::from(195_190);
    let b = GoldValue::from(95_152);
    assert_eq!(a - b, GoldValue::from(100_038));
}

#[test]
fn positive_a_sub_negative_b_with_a_coins_gt_b_coins() {
    let a = GoldValue::from(947_232);
    let b = GoldValue::from(-496_908);
    assert_eq!(a - b, GoldValue::from(1_444_140));
}

#[test]
fn positive_a_sub_negative_b_with_a_coins_lt_b_coins() {
    let a = GoldValue::from(38_194);
    let b = GoldValue::from(-237_094);
    assert_eq!(a - b, GoldValue::from(275_288));
}

#[test]
fn positive_a_sub_negative_b_with_a_coins_eq_b_coins() {
    let a = GoldValue::from(839_290);
    let b = GoldValue::from(-839290);
    assert_eq!(a - b, GoldValue::from(1_678_580));
}

#[test]
fn negative_a_sub_positive_b_with_a_coins_gt_b_coins() {
    let a = GoldValue::from(-947_232);
    let b = GoldValue::from(824_112);
    assert_eq!(a - b, GoldValue::from(-1_771_344));
}

#[test]
fn negative_a_sub_positive_b_with_a_coins_lt_b_coins() {
    let a = GoldValue::from(-128_423);
    let b = GoldValue::from(280_767);
    assert_eq!(a - b, GoldValue::from(-409_190));
}

#[test]
fn negative_a_sub_positive_b_with_a_coins_eq_b_coins() {
    let a = GoldValue::from(-834_995);
    let b = GoldValue::from(834_995);
    assert_eq!(a - b, GoldValue::from(-1_669_990));
}

#[test]
fn negative_a_sub_negative_b() {
    let a = GoldValue::from(-192_382_311);
    let b = GoldValue::from(-47_280);
    assert_eq!(a - b, GoldValue::from(-192_335_031));
}


#[test]
fn positive_coins_with_tax_applied() {
    let a = GoldValue::from(180_12_33);
    let b = GoldValue::from(153_10_48);
    let c = a.tax();
    assert_eq!(b, c);
}