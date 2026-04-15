#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Shift(pub usize);

impl std::fmt::Display for Shift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>amnt$}", "", amnt = self.0)
    }
}

pub trait StrWidth {
    #[must_use]
    fn width(&self) -> usize;
}

impl<T: ?Sized + StrWidth> StrWidth for &T {
    #[inline]
    fn width(&self) -> usize {
        (*self).width()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharWidth {
    /// Always adds the value to the width
    Exact(usize),
    /// Minimum width that gets replaced with the next character if there is one
    Augment(usize),
    /// Rounds the current width up to the nearest multiple of the value
    Align(usize),
}

impl Default for CharWidth {
    fn default() -> Self {
        Self::Exact(0)
    }
}

impl std::iter::Sum<CharWidth> for usize {
    fn sum<I: Iterator<Item = CharWidth>>(iter: I) -> Self {
        let (total, trailing_augmentation) = iter.fold((0, 0), |(total, _), width| match width {
            CharWidth::Exact(n) => (total + n, 0),
            CharWidth::Augment(n) => (total, n),
            CharWidth::Align(n) => ((total + 1).next_multiple_of(n), 0),
        });
        total + trailing_augmentation
    }
}

pub fn char_width(ch: char) -> CharWidth {
    if ch == '\t' {
        CharWidth::Align(4)
    } else if matches!(ch, ' '..='~' | '\u{2500}'..='\u{25ef}') {
        CharWidth::Exact(1)
    } else if matches!(ch, '\0'..='\x7f' | '\u{00ad}' | '\u{200b}'..='\u{200d}' | '\u{2060}' | '\u{feff}')
    {
        CharWidth::Exact(0)
    } else {
        unimplemented!()
    }
}

impl StrWidth for str {
    #[inline]
    fn width(&self) -> usize {
        self.chars().map(char_width).sum()
    }
}
impl StrWidth for usize {
    fn width(&self) -> usize {
        let mut x = *self;
        1 + std::iter::from_fn(move || {
            x /= 10;
            (x != 0).then_some(x)
        })
        .count()
    }
}
impl StrWidth for u8 {
    #[inline]
    fn width(&self) -> usize {
        (*self as usize).width()
    }
}
impl StrWidth for u16 {
    #[inline]
    fn width(&self) -> usize {
        (*self as usize).width()
    }
}
impl StrWidth for u32 {
    #[inline]
    fn width(&self) -> usize {
        (*self as usize).width()
    }
}
impl StrWidth for u64 {
    #[inline]
    fn width(&self) -> usize {
        (*self as usize).width()
    }
}
impl StrWidth for u128 {
    #[inline]
    fn width(&self) -> usize {
        (*self as usize).width()
    }
}

#[must_use]
pub fn widest<I>(iter: I) -> Option<usize>
where
    I: IntoIterator<Item: StrWidth>,
{
    iter.into_iter().map(|x| x.width()).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        const OFFSET: Shift = Shift(5);
        println!("{OFFSET}apple");
    }

    #[test]
    fn test1() {
        let items: [usize; _] = [100, 4, 45, 684, 0];
        let max_width = widest(items).unwrap();
        for item in items {
            println!("{item:>0$}", max_width);
        }
    }

    #[test]
    fn test2() {
        println!("{}", "a\tb".width());
        let items = ["apple", "orange", "a\tb"];
        let max_width = widest(items).unwrap();
        for item in items {
            println!("{item:>0$}", max_width);
        }
    }
}
