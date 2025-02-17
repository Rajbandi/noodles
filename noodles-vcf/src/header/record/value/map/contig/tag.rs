use std::str::FromStr;

use crate::header::record::value::map;

pub(super) type StandardTag = Standard;

/// A VCF header contig map tag.
pub type Tag = map::tag::Tag<StandardTag>;

// For some reason, using the `Tag` type alias produces a `nontrivial_structural_match` warning
// when pattern matching, so it's avoided here.
pub(crate) const ID: Tag = map::tag::Tag::<StandardTag>::Standard(StandardTag::Id);
pub(super) const LENGTH: Tag = map::tag::Tag::<StandardTag>::Standard(StandardTag::Length);
pub(super) const MD5: Tag = map::tag::Tag::<StandardTag>::Standard(StandardTag::Md5);
pub(super) const URL: Tag = map::tag::Tag::<StandardTag>::Standard(StandardTag::Url);
pub(super) const IDX: Tag = map::tag::Tag::<StandardTag>::Standard(StandardTag::Idx);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Standard {
    Id,
    Length,
    Md5,
    Url,
    Idx,
}

impl map::tag::Standard for Standard {}

impl AsRef<str> for Standard {
    fn as_ref(&self) -> &str {
        match self {
            Self::Id => "ID",
            Self::Length => "length",
            Self::Md5 => "md5",
            Self::Url => "URL",
            Self::Idx => "IDX",
        }
    }
}

impl FromStr for Standard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ID" => Ok(Self::Id),
            "length" => Ok(Self::Length),
            "md5" => Ok(Self::Md5),
            "URL" => Ok(Self::Url),
            "IDX" => Ok(Self::Idx),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_ref_str_for_standard() {
        assert_eq!(Standard::Id.as_ref(), "ID");
        assert_eq!(Standard::Length.as_ref(), "length");
        assert_eq!(Standard::Md5.as_ref(), "md5");
        assert_eq!(Standard::Url.as_ref(), "URL");
        assert_eq!(Standard::Idx.as_ref(), "IDX");
    }
}
