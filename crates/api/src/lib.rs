use std::fmt::{self, Display};

use common::{
    commands::{Add, ApiCommand, Delete, Read, Update},
    fetcher::{FetchError, Fetcher},
    item::{Item, Name, Section},
    items::Items,
    list::List,
    recipes::{Ingredients, Recipe},
};
use futures::FutureExt;
use persistence::store::{Storage, Store, StoreError, StoreType};

use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use tracing::{error, info, instrument, trace, warn};
use url::Url;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("API shut down before reply")]
    ApiShutdownRx,

    #[error("API shut down before send: {0}")]
    ApiShutdownTx(#[from] SendError<ApiSendWithReply>),

    #[error("fetch error: {0}")]
    FetchError(#[from] FetchError),

    #[error("store error: {0}")]
    StoreError(#[from] StoreError),
}

#[derive(Clone)]
pub struct Api {
    store: Store,
}

impl From<Store> for Api {
    fn from(store: Store) -> Self {
        Self { store }
    }
}

impl Api {
    pub async fn new(store: StoreType) -> Result<Self, ApiError> {
        info!("Initializing API with store type: {:?}", store);
        let store = Store::new(store).await?;
        Ok(Self { store })
    }

    pub async fn dispatch(&self) -> Result<ApiDispatch, ApiError> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<ApiSendWithReply>(10);

        let dispatch = ApiDispatch { tx };

        let api = self.clone();

        tokio::task::spawn(async move {
            let mut api = api;

            loop {
                tokio::select! {
                    cmd = rx.recv().fuse() => {
                        if let Some((command, reply)) = cmd {
                            let result = api
                                .execute(command)
                                .await;

                            reply
                                .send(result)
                                .await
                                .map_err(|e| {
                                    warn!(?e, "Send reply to API consumer failed");
                                })
                                .ok();
                        }
                    }
                    else => break
                }
            }
        });

        Ok(dispatch)
    }

    #[instrument(level = "debug", skip(self), ret(Debug))]
    async fn execute(&mut self, command: ApiCommand) -> Result<ApiResponse, ApiError> {
        match command {
            ApiCommand::Add(cmd) => self.add(cmd).await,
            ApiCommand::Delete(cmd) => self.delete(cmd).await,
            ApiCommand::FetchRecipe(url) => self.fetch_recipe(url).await,
            ApiCommand::MigrateJsonDbToSqlite => self.migrate_json_store_to_sqlite().await,
            ApiCommand::Read(cmd) => self.read(cmd).await,
            ApiCommand::Update(cmd) => self.update(cmd).await,
        }
    }

    async fn add(&mut self, cmd: Add) -> Result<ApiResponse, ApiError> {
        match cmd {
            Add::ChecklistItem(name) => {
                self.store.add_checklist_item(&name).await?;
                Ok(ApiResponse::AddedItem(name))
            }
            Add::Item { name, .. } => {
                self.store.add_item(&name).await?;
                Ok(ApiResponse::AddedItem(name))
            }
            Add::ListItem(name) => {
                self.store.add_list_item(&name).await?;
                Ok(ApiResponse::AddedListItem(name))
            }
            Add::ListRecipe(name) => {
                self.store.add_list_recipe(&name).await?;
                Ok(ApiResponse::AddedListRecipe(name))
            }
            Add::Recipe {
                recipe,
                ingredients,
            } => {
                self.store.add_recipe(&recipe, &ingredients).await?;
                Ok(ApiResponse::AddedRecipe(recipe))
            }
        }
    }

    async fn read(&mut self, cmd: Read) -> Result<ApiResponse, ApiError> {
        match cmd {
            Read::All => {
                let results = self.store.items().await?;
                Ok(ApiResponse::Items(results))
            }
            Read::Checklist => {
                let items = self.store.checklist().await?;
                Ok(ApiResponse::Checklist(items))
            }
            Read::Item(_name) => todo!(),
            Read::List => {
                let list = self.store.list().await?;
                Ok(ApiResponse::List(list))
            }
            Read::ListRecipes => todo!(),
            Read::Recipe(recipe) => match self.store.recipe_ingredients(&recipe).await {
                Ok(Some(ingredients)) => Ok(ApiResponse::RecipeIngredients(ingredients)),
                Ok(None) => Ok(ApiResponse::NothingReturned(ApiCommand::Read(
                    Read::Recipe(recipe),
                ))),
                Err(e) => Err(e.into()),
            },
            Read::Recipes => Ok(ApiResponse::Recipes(self.store.recipes().await?)),
            Read::Sections => {
                let results = self.store.sections().await?;
                Ok(ApiResponse::Sections(results))
            }
        }
    }

    async fn update(&mut self, cmd: Update) -> Result<ApiResponse, ApiError> {
        match cmd {
            Update::Item(_name) => todo!(),
            Update::RefreshList => {
                self.store.refresh_list().await?;
                Ok(ApiResponse::RefreshList)
            }
            Update::Recipe(_name) => todo!(),
        }
    }

    async fn delete(&mut self, cmd: Delete) -> Result<ApiResponse, ApiError> {
        match cmd {
            Delete::ChecklistItem(name) => {
                self.store.delete_checklist_item(&name).await?;
                Ok(ApiResponse::DeletedChecklistItem(name))
            }
            Delete::ClearChecklist => todo!(),
            Delete::ClearList => todo!(),
            Delete::Item(_name) => todo!(),
            Delete::ListItem(_name) => todo!(),
            Delete::Recipe(recipe) => {
                let recipe = self.store.delete_recipe(&recipe).await?;
                Ok(ApiResponse::DeletedRecipe(recipe))
            }
        }
    }

    async fn fetch_recipe(&mut self, url: Url) -> Result<ApiResponse, ApiError> {
        let fetcher = Fetcher::from(url);
        let (recipe, ingredients) = fetcher.fetch_recipe().await?;

        self.store.add_recipe(&recipe, &ingredients).await?;
        Ok(ApiResponse::FetchedRecipe((recipe, ingredients)))
    }

    async fn migrate_json_store_to_sqlite(&mut self) -> Result<ApiResponse, ApiError> {
        self.store.migrate_json_store_to_sqlite().await?;
        Ok(ApiResponse::JsonToSqlite)
    }
}

type ApiSendWithReply = (
    ApiCommand,
    tokio::sync::mpsc::Sender<Result<ApiResponse, ApiError>>,
);

#[derive(Debug, Clone)]
/// A clonable API handle
pub struct ApiDispatch {
    tx: tokio::sync::mpsc::Sender<ApiSendWithReply>,
}

impl ApiDispatch {
    #[instrument]
    pub async fn dispatch(&self, command: ApiCommand) -> Result<ApiResponse, ApiError> {
        let (reply_tx, mut reply_rx) = tokio::sync::mpsc::channel(1);
        trace!(?command, "Dispatch command to API");

        self.tx.clone().send((command, reply_tx)).await?;

        let reply = reply_rx.recv().await;

        if let Some(Err(ref error)) = reply {
            error!(?error, "API dispatch");
        }

        reply.ok_or(ApiError::ApiShutdownRx)?
    }
}

#[derive(Debug)]
pub enum ApiResponse {
    AddedItem(Name),
    AddedListItem(Name),
    AddedListRecipe(Recipe),
    AddedRecipe(Recipe),
    Checklist(Vec<Item>),
    DeletedRecipe(Recipe),
    DeletedChecklistItem(Name),
    FetchedRecipe((Recipe, Ingredients)),
    Items(Items),
    JsonToSqlite,
    List(List),
    NothingReturned(ApiCommand),
    Recipes(Vec<Recipe>),
    RecipeIngredients(Ingredients),
    RefreshList,
    Sections(Vec<Section>),
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddedItem(name) => writeln!(f, "\nitem added: {name}"),
            Self::AddedListItem(name) => writeln!(f, "\nitem added to list: {name}"),
            Self::AddedListRecipe(recipe) => {
                writeln!(f, "\nrecipe added:\n{recipe}")?;
                Ok(())
            }
            Self::AddedRecipe(name) => writeln!(f, "\nrecipe added: {name}"),
            Self::Checklist(items) => {
                writeln!(f, "\nchecklist:")?;
                for item in items {
                    writeln!(f, "{item}")?;
                }
                Ok(())
            }
            Self::DeletedChecklistItem(name) => writeln!(f, "\ndeleted from checklist: \n{name}"),
            Self::DeletedRecipe(recipe) => writeln!(f, "\ndeleted recipe: \n{recipe}"),
            Self::FetchedRecipe((recipe, ingredients)) => {
                writeln!(f, "\n{recipe}:")?;
                for ingredient in ingredients.iter() {
                    writeln!(f, "{ingredient}")?;
                }
                Ok(())
            }
            Self::Items(items) => {
                writeln!(f)?;
                for item in &items.collection {
                    writeln!(f, "{item}")?;
                }
                Ok(())
            }
            Self::JsonToSqlite => writeln!(f, "\nJSON to SQLite data store migration successful"),
            Self::List(list) => {
                writeln!(f)?;
                for item in &list.items {
                    writeln!(f, "{item}")?;
                }
                Ok(())
            }
            Self::NothingReturned(cmd) => writeln!(f, "\nnothing returned for command: {cmd:?}."),
            Self::Recipes(recipes) => {
                writeln!(f)?;
                for recipe in recipes {
                    writeln!(f, "{recipe}")?;
                }
                Ok(())
            }
            Self::RecipeIngredients(ingredients) => {
                writeln!(f)?;
                for ingredient in ingredients.iter() {
                    writeln!(f, "{ingredient}")?;
                }
                Ok(())
            }

            Self::RefreshList => writeln!(f, "\nList is now empty"),
            Self::Sections(sections) => {
                writeln!(f)?;
                for section in sections {
                    writeln!(f, "{section}")?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn serve_api() {
        let api = Api::from(Store::new_inmem(StoreType::Sqlite).await.unwrap());
        let api_dispatch = api.dispatch().await.unwrap();

        let response = api_dispatch
            .dispatch(ApiCommand::Add(Add::Recipe { recipe: Recipe::new("fluffy american pancakes").unwrap(), ingredients: Ingredients::from_input_string("135g/4¾oz plain flour, 1 tsp baking powder, ½ tsp salt, 2 tbsp caster sugar, 130ml/4½fl oz milk, 1 large egg, lightly beaten, 2 tbsp melted butter (allowed to cool slightly), plus extra for cooking") }))
            .await
            .unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @r###"

        recipe added: fluffy american pancakes
        "###);

        let response = api_dispatch
            .dispatch(ApiCommand::Read(Read::Recipes))
            .await
            .unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @"fluffy american pancakes");

        let response = api_dispatch
            .dispatch(ApiCommand::Read(Read::Recipe(
                Recipe::new("fluffy american pancakes").unwrap(),
            )))
            .await
            .unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @r###"

        135g/4¾oz plain flour
        1 tsp baking powder
        ½ tsp salt
        2 tbsp caster sugar
        130ml/4½fl oz milk
        1 large egg
        lightly beaten
        2 tbsp melted butter (allowed to cool slightly)
        plus extra for cooking
        "###);

        let response = api_dispatch
            .dispatch(ApiCommand::Read(Read::All))
            .await
            .unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @r###"
        135g/4¾oz plain flour
        1 tsp baking powder
        ½ tsp salt
        2 tbsp caster sugar
        130ml/4½fl oz milk
        1 large egg
        lightly beaten
        2 tbsp melted butter (allowed to cool slightly)
        plus extra for cooking
        "###);

        let response = api_dispatch
            .dispatch(ApiCommand::Delete(Delete::Recipe(
                Recipe::new("fluffy american pancakes").unwrap(),
            )))
            .await
            .unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @r###"
        deleted recipe: 
        fluffy american pancakes
        "###);

        let response = api_dispatch
            .dispatch(ApiCommand::Read(Read::Recipes))
            .await
            .unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @"");

        let response = api_dispatch
            .dispatch(ApiCommand::Read(Read::All))
            .await
            .unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @"");
    }
}
