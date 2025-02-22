#[derive(Debug)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub license: String,
    pub requires_cpp: String,
    pub keywords: Vec<String>,
    pub maintainers: Vec<Maintainer>,
    pub authors: Vec<Author>,
}

#[derive(Debug)]
pub struct Maintainer {
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl ProjectMetadata {
    pub fn new(
        name: &str,
        description: &str,
        version: &str,
        license: &str,
        requires_cpp: &str,
        keywords: Vec<&str>,
        maintainers: Vec<Maintainer>,
        authors: Vec<Author>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            version: version.to_string(),
            license: license.to_string(),
            requires_cpp: requires_cpp.to_string(),
            keywords: keywords.into_iter().map(|s| s.to_string()).collect(),
            maintainers,
            authors,
        }
    }
}