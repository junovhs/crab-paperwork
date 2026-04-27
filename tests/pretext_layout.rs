use crab_paperwork::pretext::{
    layout, measure_line_stats, prepare, FontMetrics, PrepareOptions, WhiteSpace,
};

#[test]
fn prepare_then_layout_is_reusable_for_multiple_widths() {
    let prepared = prepare(
        "AGI spring paperwork",
        FontMetrics::monospace(16.0),
        PrepareOptions::default(),
    );

    let narrow = layout(&prepared, 60.0, 20.0);
    let wide = layout(&prepared, 400.0, 20.0);

    assert!(narrow.line_count > wide.line_count);
    assert!(narrow.height > wide.height);
}

#[test]
fn pre_wrap_counts_hard_breaks_and_tabs() {
    let prepared = prepare(
        "one\n\two",
        FontMetrics::monospace(16.0),
        PrepareOptions {
            white_space: WhiteSpace::PreWrap,
            ..PrepareOptions::default()
        },
    );

    let stats = measure_line_stats(&prepared, 500.0);

    assert_eq!(stats.line_count, 2);
    assert!(stats.max_line_width > 16.0);
}
