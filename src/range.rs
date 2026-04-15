#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub const fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub const fn position(pos: usize) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }

    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn line_around(&self, s: &str) -> Range {
        Range {
            start: line_start(s, self.start),
            end: line_end(s, self.end),
        }
    }
}

pub fn line_start(s: &str, pos: usize) -> usize {
    assert!(pos <= s.len());
    s[..pos]
        .rfind('\n')
        .map(|i| i + '\n'.len_utf8())
        .unwrap_or(0)
}

pub fn line_end(s: &str, pos: usize) -> usize {
    assert!(pos <= s.len());
    s[pos..].find('\n').map(|i| i + pos).unwrap_or(s.len())
}

pub fn line_index(s: &str, pos: usize) -> usize {
    assert!(pos <= s.len());
    s[..pos].matches('\n').count()
}

pub fn column(line_start: usize, pos: usize) -> usize {
    assert!(pos >= line_start);
    pos - line_start
}
