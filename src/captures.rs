use std::iter::Iterator;
use super::{Region, Regex, SEARCH_OPTION_NONE};

impl Regex {
    /// Returns the capture groups corresponding to the leftmost-first match
    /// in text. Capture group `0` always corresponds to the entire match.
    /// If no match is found, then `None` is returned.
    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        let mut region = Region::new();
        self.search_with_options(text, 0, text.len(),
                                 SEARCH_OPTION_NONE, Some(&mut region))
            .map(|_| Captures {
                text: text,
                region: region,
            })
    }
}

/// Captures represents a group of captured strings for a single match.
///
/// The 0th capture always corresponds to the entire match. Each subsequent
/// index corresponds to the next capture group in the regex. Positions
/// returned from a capture group are always byte indices.
///
/// `'t` is the lifetime of the matched text.
#[derive(Debug)]
pub struct Captures<'t> {
    text: &'t str,
    region: Region,
}

impl<'t> Captures<'t> {
    /// Returns the start and end positions of the Nth capture group. Returns
    /// `None` if i is not a valid capture group or if the capture group did
    /// not match anything. The positions returned are always byte indices with
    /// respect to the original string matched.
    pub fn pos(&self, pos: usize) -> Option<(usize, usize)> {
        self.region.pos(pos)
    }

    /// Returns the matched string for the capture group `i`. If `i` isn't
    /// a valid capture group or didn't match anything, then `None` is returned.
    pub fn at(&self, pos: usize) -> Option<&'t str> {
        self.pos(pos).map(|(beg, end)| &self.text[beg..end])
    }

    /// Returns the number of captured groups.
    pub fn len(&self) -> usize {
        self.region.len()
    }

    /// Returns true if and only if there are no captured groups.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Creates an iterator of all the capture groups in order of appearance in
    /// the regular expression.
    pub fn iter(&'t self) -> SubCaptures<'t> {
        SubCaptures {
            idx: 0,
            caps: self,
        }
    }

    /// Creates an iterator of all the capture group positions in order of
    /// appearance in the regular expression. Positions are byte indices in
    /// terms of the original string matched.
    pub fn iter_pos(&'t self) -> SubCapturesPos<'t> {
        SubCapturesPos {
            idx: 0,
            caps: self,
        }
    }
}

/// An iterator over capture groups for a particular match of a regular
/// expression.
///
///`'t` is the lifetime of the matched text.
pub struct SubCaptures<'t> {
    idx: usize,
    caps: &'t Captures<'t>,
}

impl<'t> Iterator for SubCaptures<'t> {
    type Item = Option<&'t str>;

    fn next(&mut self) -> Option<Option<&'t str>> {
        if self.idx < self.caps.len() {
            self.idx += 1;
            Some(self.caps.at(self.idx - 1))
        } else {
            None
        }
    }
}

/// An iterator over capture group positions for a particular match of
/// a regular expression.
///
/// Positions are byte indices in terms of the original
/// string matched. `'t` is the lifetime of the matched text.
pub struct SubCapturesPos<'t> {
    idx: usize,
    caps: &'t Captures<'t>,
}

impl<'t> Iterator for SubCapturesPos<'t> {
    type Item = Option<(usize, usize)>;

    fn next(&mut self) -> Option<Option<(usize, usize)>> {
        if self.idx < self.caps.len() {
            self.idx += 1;
            Some(self.caps.pos(self.idx - 1))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_regex_captures() {
        let regex = Regex::new("e(l+)|(r+)").unwrap();
        let captures = regex.captures("hello").unwrap();
        assert_eq!(captures.len(), 3);
        assert_eq!(captures.is_empty(), false);
        let pos1 = captures.pos(0).unwrap();
        let pos2 = captures.pos(1).unwrap();
        let pos3 = captures.pos(2);
        assert_eq!(pos1, (1, 4));
        assert_eq!(pos2, (2, 4));
        assert_eq!(pos3, None);
        let str1 = captures.at(0).unwrap();
        let str2 = captures.at(1).unwrap();
        let str3 = captures.at(2);
        assert_eq!(str1, "ell");
        assert_eq!(str2, "ll");
        assert_eq!(str3, None);

    }

    #[test]
    fn test_regex_subcaptures() {
        let regex = Regex::new("e(l+)").unwrap();
        let captures = regex.captures("hello").unwrap();
        let caps = captures.iter().collect::<Vec<_>>();
        assert_eq!(caps[0], Some("ell"));
        assert_eq!(caps[1], Some("ll"));
        assert_eq!(caps.len(), 2);

    }

    #[test]
    fn test_regex_subcapturespos() {
        let regex = Regex::new("e(l+)").unwrap();
        let captures = regex.captures("hello").unwrap();
        let caps = captures.iter_pos().collect::<Vec<_>>();
        assert_eq!(caps[0], Some((1, 4)));
        assert_eq!(caps[1], Some((2, 4)));
        assert_eq!(caps.len(), 2);

    }
}
