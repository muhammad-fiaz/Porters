#[derive(Debug)]
pub struct Contact {
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: Option<String>,
    pub project_type: String,
    pub version: String,
    pub license: Option<String>,
    pub requires_cpp: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub maintainers: Option<Vec<Contact>>,
    pub authors: Option<Vec<Contact>>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub readme: Option<String>,
}

impl ProjectMetadata {
    pub fn new(
        name: String,
        project_type: String,
        version: String,
        description: Option<String>,
        license: Option<String>,
        requires_cpp: Option<String>,
        keywords: Option<Vec<String>>,
        maintainers: Option<Vec<Contact>>,
        authors: Option<Vec<Contact>>,
        homepage: Option<String>,
        documentation: Option<String>,
        repository: Option<String>,
        readme: Option<String>,
    ) -> Self {
        Self {
            name,
            project_type,
            version,
            description,
            license,
            requires_cpp,
            keywords,
            maintainers,
            authors,
            homepage,
            documentation,
            repository,
            readme,
        }
    }
}