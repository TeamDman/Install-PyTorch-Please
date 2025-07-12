use holda::StringHolda;
use std::sync::Arc;

#[derive(StringHolda)]
pub struct IndexUrl {
    inner: String,
}
impl IndexUrl {
    pub fn without_path(&self) -> &str {
        // stop before the first slash after the scheme and host
        // https://pypi.org/simple => https://pypi.org
        // https://download.pytorch.org/whl/nightly/ => https://download.pytorch.org

        // Find "://" to skip the scheme
        if let Some(scheme_end) = self.find("://") {
            // Find the first '/' after the scheme
            let host_start = scheme_end + 3;
            if let Some(path_start) = self[host_start..].find('/') {
                let end = host_start + path_start;
                &self[..end]
            } else {
                // No path, return the whole string
                self
            }
        } else {
            // Not a valid URL, return as is
            self
        }
    }
}

pub fn get_indexes() -> Vec<Arc<IndexUrl>> {
    include_str!("./indexes.txt")
        .lines()
        .map(|line| Arc::new(IndexUrl::new(line.to_string())))
        .collect()
}

#[derive(Debug)]
pub struct Package {
    pub href: Arc<str>,
    pub label: Arc<str>,
    pub index: Arc<IndexUrl>,
}
impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
impl Package {
    fn as_url(&self) -> String {
        // torch index has href = "torch/"
        // pypi index has href = "/simple/loguru"
        // we need to not duplicate the subpath if it already exists
        if self.href.starts_with('/') {
            // pypi index
            return format!("{}{}", self.index.without_path(), self.href);
        } else {
            format!("{}{}", self.index.inner, self.href)
        }
    }
}

pub fn get_index_packages(index: Arc<IndexUrl>) -> eyre::Result<Vec<Package>> {
    // GET request to the index URL
    let response = reqwest::blocking::get(&index.inner)?;
    if response.status().is_success() {
        let content = response.text()?;
        let mut entries = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if let Some(href_start) = line.find("<a href=\"") {
                let href_end = line[href_start + 9..].find('"');
                if let Some(href_end) = href_end {
                    let href = &line[href_start + 9..href_start + 9 + href_end];
                    if let Some(label_start) = line[href_start + 9 + href_end..].find('>') {
                        let label_end =
                            line[href_start + 9 + href_end + label_start + 1..].find('<');
                        if let Some(label_end) = label_end {
                            let label = &line[href_start + 9 + href_end + label_start + 1
                                ..href_start + 9 + href_end + label_start + 1 + label_end];
                            entries.push(Package {
                                href: Arc::from(href),
                                label: Arc::from(label),
                                index: index.clone(),
                            });
                        }
                    }
                }
            }
        }
        return Ok(entries);
    } else {
        eyre::bail!("Failed to fetch index: {}", response.status());
    }
}

#[derive(Debug)]
pub struct PackageVersion {
    pub href: Arc<str>,
    pub label: Arc<str>,
    pub index_entry: Arc<Package>,
}
impl std::fmt::Display for PackageVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

pub fn get_package_versions(package: Arc<Package>) -> eyre::Result<Vec<PackageVersion>> {
    // GET request to the index entry URL
    let response = reqwest::blocking::get(package.as_url())?;
    if response.status().is_success() {
        let content = response.text()?;
        let mut entries = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if let Some(href_start) = line.find("<a href=\"") {
                let href_end = line[href_start + 9..].find('"');
                if let Some(href_end) = href_end {
                    let href = &line[href_start + 9..href_start + 9 + href_end];
                    if let Some(label_start) = line[href_start + 9 + href_end..].find('>') {
                        let label_end =
                            line[href_start + 9 + href_end + label_start + 1..].find('<');
                        if let Some(label_end) = label_end {
                            let label = &line[href_start + 9 + href_end + label_start + 1
                                ..href_start + 9 + href_end + label_start + 1 + label_end];
                            entries.push(PackageVersion {
                                href: Arc::from(href),
                                label: Arc::from(label),
                                index_entry: package.clone(),
                            });
                        }
                    }
                }
            }
        }
        return Ok(entries);
    } else {
        eyre::bail!("Failed to fetch index entry: {}", response.status());
    }
}
