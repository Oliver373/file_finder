use regex::Regex;

pub enum SearchPattern {
    Plain(String),
    Regex(Regex),
}

impl SearchPattern {
    pub fn new(use_regex: bool, search_pattern: impl Into<String>) -> Result<SearchPattern, regex::Error>{
        let search_pattern = if use_regex {
            SearchPattern::from(Regex::new(&search_pattern.into())?)
        } else {
            SearchPattern::from(search_pattern.into())
        };
        Ok(search_pattern)
    }

    pub fn is_match(&self, file_name: &str) -> bool {
        match self {
            SearchPattern::Plain(pattern) => file_name.contains(pattern),
            SearchPattern::Regex(regex) => regex.is_match(file_name),
        }
    }
}

impl Clone for SearchPattern {
    fn clone(&self) -> Self {
        match self {
            SearchPattern::Plain(pattern) => SearchPattern::Plain(pattern.clone()),
            SearchPattern::Regex(regex) => SearchPattern::Regex(regex.clone()),
        }
    }
}

impl From<String> for SearchPattern {
    fn from(s: String) -> Self {
        SearchPattern::Plain(s)
    }
}

impl From<Regex> for SearchPattern {
    fn from(r: Regex) -> Self {
        SearchPattern::Regex(r)
    }
}