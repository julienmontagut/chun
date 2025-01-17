use reqwest::StatusCode;

pub struct Registry {
    pub name: &'static str,
    pub url_template: &'static str,
    existance_check: fn(&PackageSearchResponse) -> Result<bool, Box<dyn std::error::Error>>,
}

impl Registry {
    fn new(name: &'static str, url_template: &'static str) -> Self {
        Self {
            name,
            url_template,
            existance_check: |response| Ok(response.is_success()),
        }
    }

    fn with_existance_check(
        self,
        existance_check: fn(&PackageSearchResponse) -> Result<bool, Box<dyn std::error::Error>>,
    ) -> Self {
        Self {
            name: self.name,
            url_template: self.url_template,
            existance_check,
        }
    }
}

struct PackageSearchResponse {
    status: StatusCode,
    _content: String,
}

impl PackageSearchResponse {
    fn is_success(&self) -> bool {
        self.status.is_success()
    }
}

pub fn registries() -> Vec<Registry> {
    vec![
        Registry::new("crates.io", "https://crates.io/api/v1/crates/{}"),
        Registry::new("brew.sh", "https://formulae.brew.sh/formula/{}"),
        Registry::new("debian", "https://tracker.debian.org/pkg/{}"),
        // Registry::new("nixos", "https://search.nixos.org/packages?channel=unstable&from=0&size=1&sort=relevance&type=packages&query={}"),
        // Registry::new("archlinux", "https://archlinux.org/packages/search/json/?q={}")
        // .with_existance_check(|response| {
        //     Ok(response.content.contains("\"name\":"))
        // }),
    ]
}

pub async fn check_package(
    registry: &Registry,
    package: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = registry.url_template.replace("{}", package);
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (compatible)")
        .send()
        .await?;

    let response = PackageSearchResponse {
        status: response.status(),
        _content: response.text().await?,
    };

    (registry.existance_check)(&response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn when_package_exists_then_return_true() {
        let existing_package = "eza";
        let registries = registries();
        for registry in registries {
            let result = check_package(&registry, existing_package).await;
            assert!(
                result.is_ok(),
                "Failed to check package on registry: {}",
                registry.name
            );
            assert!(
                result.unwrap(),
                "Package {} should be found on registry: {}",
                existing_package,
                registry.name
            );
        }
    }

    #[tokio::test]
    async fn when_package_does_not_exist_then_return_false() {
        let non_existing_package = "nonexistent-package";
        let registries = registries();
        for registry in registries {
            let result = check_package(&registry, non_existing_package).await;
            assert!(
                result.is_ok(),
                "Failed to check package on registry: {}",
                registry.name
            );
            assert!(
                !result.unwrap(),
                "Package {} should not be found on registry: {}",
                non_existing_package,
                registry.name
            );
        }
    }
}
