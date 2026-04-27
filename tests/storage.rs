use crab_paperwork::storage::config::AppConfig;

#[test]
fn clamps_split_ratio_to_usable_bounds() {
    let too_small = AppConfig {
        split_ratio: 0.05,
        ..AppConfig::default()
    };
    let too_large = AppConfig {
        split_ratio: 0.95,
        ..AppConfig::default()
    };

    assert_eq!(too_small.normalized_split_ratio(), 0.2);
    assert_eq!(too_large.normalized_split_ratio(), 0.8);
}
