use std::iter::Iterator;
use super::Region;

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
    region: Region
}

impl<'t> Captures<'t> {
    pub fn new(text: &'t str, region: Region) -> Captures<'t> {
        Captures {
            text: text,
            region: region
        }
    }

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
    // the regular expression.
    pub fn iter(&'t self) -> SubCaptures<'t> {
        SubCaptures { idx: 0, caps: self }
    }

    /// Creates an iterator of all the capture group positions in order of
    /// appearance in the regular expression. Positions are byte indices in
    /// terms of the original string matched.
    pub fn iter_pos(&'t self) -> SubCapturesPos<'t> {
        SubCapturesPos { idx: 0, caps: self }
    }
}

/// An iterator over capture groups for a particular match of a regular
/// expression.
///
///`'t` is the lifetime of the matched text.
pub struct SubCaptures<'t> {
    idx: usize,
    caps: &'t Captures<'t>
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
    caps: &'t Captures<'t>
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
