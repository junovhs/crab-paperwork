use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WhiteSpace {
    Normal,
    PreWrap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordBreak {
    Normal,
    KeepAll,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrepareOptions {
    pub white_space: WhiteSpace,
    pub word_break: WordBreak,
    pub letter_spacing: f32,
}

impl Default for PrepareOptions {
    fn default() -> Self {
        Self {
            white_space: WhiteSpace::Normal,
            word_break: WordBreak::Normal,
            letter_spacing: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontMetrics {
    pub latin_advance: f32,
    pub wide_advance: f32,
    pub space_advance: f32,
    pub tab_size: usize,
}

impl FontMetrics {
    pub fn monospace(font_size_px: f32) -> Self {
        let latin_advance = font_size_px * 0.6;

        Self {
            latin_advance,
            wide_advance: font_size_px,
            space_advance: latin_advance,
            tab_size: 8,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PreparedText {
    segments: Vec<Segment>,
}

#[derive(Clone, Debug)]
struct Segment {
    text: String,
    width: f32,
    break_after: bool,
    hard_break_after: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineStats {
    pub line_count: usize,
    pub max_line_width: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutResult {
    pub height: f32,
    pub line_count: usize,
}

pub fn prepare(text: &str, metrics: FontMetrics, options: PrepareOptions) -> PreparedText {
    let normalized = match options.white_space {
        WhiteSpace::Normal => normalize_normal_whitespace(text),
        WhiteSpace::PreWrap => text.replace("\r\n", "\n").replace('\r', "\n"),
    };

    let mut segments = Vec::new();

    for (line_index, line) in normalized.split('\n').enumerate() {
        if line_index > 0 && let Some(previous) = segments.last_mut() {
            previous.hard_break_after = true;
            previous.break_after = true;
        }

        prepare_line(line, metrics, options, &mut segments);
    }

    PreparedText { segments }
}

pub fn layout(prepared: &PreparedText, max_width: f32, line_height: f32) -> LayoutResult {
    let line_count = measure_line_stats(prepared, max_width).line_count;

    LayoutResult {
        height: line_count as f32 * line_height,
        line_count,
    }
}

pub fn measure_line_stats(prepared: &PreparedText, max_width: f32) -> LineStats {
    if prepared.segments.is_empty() {
        return LineStats {
            line_count: 0,
            max_line_width: 0.0,
        };
    }

    let max_width = max_width.max(1.0);
    let mut line_count = 1;
    let mut current_width = 0.0_f32;
    let mut max_line_width = 0.0_f32;

    for segment in &prepared.segments {
        let overflows = current_width > 0.0 && current_width + segment.width > max_width;

        if overflows {
            max_line_width = max_line_width.max(current_width);
            line_count += 1;
            current_width = 0.0;
        }

        current_width += segment.width;

        if segment.hard_break_after {
            max_line_width = max_line_width.max(current_width);
            line_count += 1;
            current_width = 0.0;
        }
    }

    if current_width == 0.0 {
        line_count = line_count.saturating_sub(1);
    } else {
        max_line_width = max_line_width.max(current_width);
    }

    LineStats {
        line_count,
        max_line_width,
    }
}

fn prepare_line(
    line: &str,
    metrics: FontMetrics,
    options: PrepareOptions,
    segments: &mut Vec<Segment>,
) {
    match options.white_space {
        WhiteSpace::Normal => {
            for word in line.split(' ') {
                if word.is_empty() {
                    continue;
                }

                push_segment(word, metrics, options, true, false, segments);
                push_space(metrics, options, true, segments);
            }

            if matches!(segments.last(), Some(segment) if segment.text == " ") {
                segments.pop();
            }
        }
        WhiteSpace::PreWrap => {
            for grapheme in line.graphemes(true) {
                let break_after = match options.word_break {
                    WordBreak::Normal => grapheme.chars().all(char::is_whitespace),
                    WordBreak::KeepAll => grapheme == " " || grapheme == "\t",
                };

                push_segment(grapheme, metrics, options, break_after, false, segments);
            }
        }
    }
}

fn push_space(
    metrics: FontMetrics,
    options: PrepareOptions,
    break_after: bool,
    segments: &mut Vec<Segment>,
) {
    segments.push(Segment {
        text: " ".to_string(),
        width: metrics.space_advance + options.letter_spacing,
        break_after,
        hard_break_after: false,
    });
}

fn push_segment(
    text: &str,
    metrics: FontMetrics,
    options: PrepareOptions,
    break_after: bool,
    hard_break_after: bool,
    segments: &mut Vec<Segment>,
) {
    segments.push(Segment {
        text: text.to_string(),
        width: measure_text(text, metrics, options.letter_spacing),
        break_after,
        hard_break_after,
    });
}

fn measure_text(text: &str, metrics: FontMetrics, letter_spacing: f32) -> f32 {
    let mut width = 0.0_f32;
    let mut graphemes = 0_usize;

    for grapheme in text.graphemes(true) {
        graphemes += 1;

        width += if grapheme == "\t" {
            metrics.space_advance * metrics.tab_size as f32
        } else if grapheme.chars().any(is_wide) {
            metrics.wide_advance
        } else if grapheme.chars().all(char::is_whitespace) {
            metrics.space_advance
        } else {
            metrics.latin_advance
        };
    }

    width + letter_spacing * graphemes.saturating_sub(1) as f32
}

fn normalize_normal_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_wide(character: char) -> bool {
    matches!(
        character,
        '\u{1100}'..='\u{11ff}'
            | '\u{2e80}'..='\u{a4cf}'
            | '\u{ac00}'..='\u{d7af}'
            | '\u{f900}'..='\u{faff}'
            | '\u{fe10}'..='\u{fe1f}'
            | '\u{fe30}'..='\u{fe6f}'
            | '\u{ff00}'..='\u{ffef}'
            | '\u{1f300}'..='\u{1faff}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_has_no_layout_height() {
        let prepared = prepare(
            "",
            FontMetrics::monospace(16.0),
            PrepareOptions::default(),
        );

        assert_eq!(layout(&prepared, 320.0, 20.0).line_count, 0);
        assert_eq!(layout(&prepared, 320.0, 20.0).height, 0.0);
    }

    #[test]
    fn pre_wrap_preserves_hard_breaks() {
        let prepared = prepare(
            "one\ntwo\nthree",
            FontMetrics::monospace(16.0),
            PrepareOptions {
                white_space: WhiteSpace::PreWrap,
                ..PrepareOptions::default()
            },
        );

        assert_eq!(layout(&prepared, 400.0, 20.0).line_count, 3);
    }

    #[test]
    fn letter_spacing_changes_measured_width() {
        let metrics = FontMetrics::monospace(10.0);
        let normal = prepare("abcd", metrics, PrepareOptions::default());
        let spaced = prepare(
            "abcd",
            metrics,
            PrepareOptions {
                letter_spacing: 2.0,
                ..PrepareOptions::default()
            },
        );

        assert!(measure_line_stats(&spaced, 500.0).max_line_width > measure_line_stats(&normal, 500.0).max_line_width);
    }
}
