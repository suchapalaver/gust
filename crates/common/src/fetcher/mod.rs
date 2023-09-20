use scraper::{Html, Selector};
use thiserror::Error;
use url::Url;

use crate::recipes::{Ingredients, Recipe};

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("CSS selector failed to select anything")]
    CSS,
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

pub struct Fetcher {
    site: Site,
    url: Url,
}

#[allow(dead_code, clippy::upper_case_acronyms)]
enum Site {
    BBC,
    NYT,
}

impl From<Url> for Fetcher {
    fn from(url: Url) -> Self {
        match url.host_str() {
            Some("www.bbc.co.uk") => Self::new(Site::BBC, url),
            _ => unimplemented!(
                "'gust' currently only supports requests for recipes from the BBC Food website."
            ),
        }
    }
}

impl Fetcher {
    fn new(site: Site, url: Url) -> Self {
        Self { site, url }
    }

    pub async fn fetch_recipe(&self) -> Result<(Recipe, Ingredients), FetchError> {
        let document = self.fetch_html().await?;
        Ok((
            self.fetch_recipe_name(&document)?.trim().into(),
            self.fetch_recipe_ingredients(&document)?
                .into_iter()
                .map(|i| i.trim().into())
                .collect(),
        ))
    }

    async fn fetch_html(&self) -> Result<Html, reqwest::Error> {
        let response = reqwest::get(self.url.as_str()).await?;
        let body = response.text().await?;
        Ok(Html::parse_document(&body))
    }

    fn fetch_recipe_name(&self, document: &Html) -> Result<String, FetchError> {
        let recipe_name_selector = match self.site {
            Site::BBC => Selector::parse(".gel-trafalgar").unwrap(),
            Site::NYT => unimplemented!(),
        };

        match document.select(&recipe_name_selector).next() {
            Some(recipe_name_element) => Ok(recipe_name_element
                .text()
                .collect::<String>()
                .to_lowercase()),
            None => Err(FetchError::CSS),
        }
    }

    fn fetch_recipe_ingredients(&self, document: &Html) -> Result<Vec<String>, FetchError> {
        let ingredients_selector = match self.site {
            Site::BBC => Selector::parse(".recipe-ingredients__list").unwrap(),
            Site::NYT => unimplemented!(),
        };

        if let Some(ingredients_container) = document.select(&ingredients_selector).next() {
            let mut ingredients = Vec::new();
            // Iterate through child elements to extract individual ingredients
            for ingredient_element in ingredients_container.select(&Selector::parse("li").unwrap())
            {
                ingredients.push(ingredient_element.text().collect::<String>().to_lowercase());
            }
            Ok(ingredients)
        } else {
            Err(FetchError::CSS)
        }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::fetcher::Fetcher;

    fn url() -> Url {
        Url::parse("https://www.bbc.co.uk/food/recipes/scrambledeggandtoast_75736").unwrap()
    }

    #[tokio::test]
    async fn test_fetch_recipe_ingredients() {
        let recipe_url = url();
        let fetcher: Fetcher = recipe_url.into();
        let doc = fetcher.fetch_html().await.unwrap();
        let ingredients = fetcher.fetch_recipe_ingredients(&doc).unwrap();
        insta::assert_debug_snapshot!(ingredients, @r#"
        [
            "1 tbsp butter, plus extra for spreading",
            "2 large free-range eggs",
            "1 tbsp milk",
            "1 slice wholemeal bread, toasted",
            "2 slices smoked salmon",
            "salt and freshly ground black pepper",
        ]
        "#);
    }

    #[tokio::test]
    async fn test_fetch_recipe_name() {
        let recipe_url = url();
        let fetcher: Fetcher = recipe_url.into();
        let doc = fetcher.fetch_html().await.unwrap();
        let recipe = fetcher.fetch_recipe_name(&doc).unwrap();
        insta::assert_display_snapshot!(recipe, @"scrambled egg and toast with smoked salmon");
    }
}
