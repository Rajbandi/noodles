//! GFF record attributes and entry.

pub mod entry;

pub use self::entry::Entry;

use std::{error, fmt, ops::Deref, str::FromStr};

const DELIMITER: char = ';';

/// GFF record attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Attributes(Vec<Entry>);

impl Deref for Attributes {
    type Target = [Entry];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, entry) in self.iter().enumerate() {
            if i > 0 {
                write!(f, "{DELIMITER}")?;
            }

            write!(f, "{entry}")?;
        }

        Ok(())
    }
}

/// An error returned when raw attributes fail to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The input attributes has an invalid entry.
    InvalidEntry(entry::ParseError),
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::InvalidEntry(e) => Some(e),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEntry(_) => f.write_str("invalid entry"),
        }
    }
}

impl From<Vec<Entry>> for Attributes {
    fn from(entries: Vec<Entry>) -> Self {
        Self(entries)
    }
}

impl FromStr for Attributes {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::default());
        }

        s.split(DELIMITER)
            .map(|t| t.parse())
            .collect::<Result<Vec<_>, _>>()
            .map(Self::from)
            .map_err(ParseError::InvalidEntry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        let attributes = Attributes::default();
        assert!(attributes.to_string().is_empty());

        let attributes = Attributes::from(vec![Entry::new("gene_id", "ndls0")]);

        assert_eq!(attributes.to_string(), "gene_id=ndls0");

        let attributes = Attributes::from(vec![
            Entry::new("gene_id", "ndls0"),
            Entry::new("gene_name", "gene0"),
        ]);

        assert_eq!(attributes.to_string(), "gene_id=ndls0;gene_name=gene0")
    }

    #[test]
    fn test_from_str() -> Result<(), ParseError> {
        let s = "gene_id=ndls0;gene_name=gene0";
        let actual = s.parse::<Attributes>()?;
        let expected = Attributes::from(vec![
            Entry::new("gene_id", "ndls0"),
            Entry::new("gene_name", "gene0"),
        ]);
        assert_eq!(actual, expected);

        let s = "gene_id=ndls0";
        let actual = s.parse::<Attributes>()?;
        let expected = Attributes::from(vec![Entry::new(
            String::from("gene_id"),
            String::from("ndls0"),
        )]);
        assert_eq!(actual, expected);

        let actual = "".parse::<Attributes>()?;
        let expected = Attributes::default();
        assert_eq!(actual, expected);

        Ok(())
    }
}
