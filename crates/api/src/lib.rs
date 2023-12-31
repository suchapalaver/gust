use std::fmt::{self, Display};

use common::{
    commands::ApiCommand,
    item::{Item, Name, Section},
    items::Items,
    list::List,
    recipes::{Ingredients, Recipe},
};
use persistence::store::{Store, StoreDispatch, StoreError, StoreResponse, StoreType};

use futures::FutureExt;
use thiserror::Error;
use tokio::sync::{
    mpsc::{self, error::SendError},
    oneshot,
};
use tracing::{error, info, instrument, trace, warn};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("API shut down before reply")]
    ApiShutdownRx,

    #[error("API shut down before send: {0}")]
    ApiShutdownTx(#[from] SendError<ApiSendWithReply>),

    #[error("{0}")]
    RecvError(#[from] oneshot::error::RecvError),

    #[error("store error: {0}")]
    StoreError(#[from] StoreError),
}

pub struct Api {
    store: StoreDispatch,
}

impl Api {
    pub async fn init(store: StoreType) -> Result<ApiDispatch, ApiError> {
        info!("Initializing API with store type: {:?}", store);

        let store = Store::from_store_type(store).await?.init().await?;

        let api = Api { store };

        let (tx, mut rx) = mpsc::channel::<ApiSendWithReply>(10);
        let dispatch = ApiDispatch { tx };

        tokio::task::spawn(async move {
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
    async fn execute(&self, command: ApiCommand) -> Result<ApiResponse, ApiError> {
        let (tx, rx) = oneshot::channel();
        self.store.send((command, tx)).await?;
        let res = rx.await??;
        Ok(res.into())
    }
}

type ApiSendWithReply = (ApiCommand, mpsc::Sender<Result<ApiResponse, ApiError>>);

#[derive(Debug, Clone)]
/// A clonable API handle
pub struct ApiDispatch {
    tx: mpsc::Sender<ApiSendWithReply>,
}

impl ApiDispatch {
    #[instrument]
    pub async fn dispatch(&self, command: ApiCommand) -> Result<ApiResponse, ApiError> {
        let (reply_tx, mut reply_rx) = mpsc::channel(1);
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
    AddedChecklistItem(Name),
    AddedItem(Name),
    AddedListItem(Name),
    AddedListRecipe(Recipe),
    AddedRecipe(Recipe),
    Checklist(Vec<Item>),
    DeletedRecipe(Recipe),
    DeletedChecklistItem(Name),
    FetchedRecipe((Recipe, Ingredients)),
    ItemAlreadyAdded(Name),
    Items(Items),
    JsonToSqlite,
    List(List),
    NothingReturned(ApiCommand),
    Recipes(Vec<Recipe>),
    RecipeIngredients(Option<Ingredients>),
    RefreshList,
    Sections(Vec<Section>),
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddedChecklistItem(name) => writeln!(f, "\nchecklist item added: {name}"),
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
            Self::ItemAlreadyAdded(item) => writeln!(f, "\nitem already added: {item}"),
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
                if let Some(ingredients) = ingredients {
                    writeln!(f)?;
                    for ingredient in ingredients.iter() {
                        writeln!(f, "{ingredient}")?;
                    }
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

impl From<StoreResponse> for ApiResponse {
    fn from(res: StoreResponse) -> Self {
        match res {
            StoreResponse::AddedChecklistItem(item) => Self::AddedChecklistItem(item),
            StoreResponse::AddedItem(item) => Self::AddedItem(item),
            StoreResponse::AddedListItem(item) => Self::AddedListItem(item),
            StoreResponse::AddedListRecipe(item) => Self::AddedListRecipe(item),
            StoreResponse::AddedRecipe(item) => Self::AddedRecipe(item),
            StoreResponse::Checklist(item) => Self::Checklist(item),
            StoreResponse::DeletedRecipe(item) => Self::DeletedRecipe(item),
            StoreResponse::DeletedChecklistItem(item) => Self::DeletedChecklistItem(item),
            StoreResponse::FetchedRecipe(item) => Self::FetchedRecipe(item),
            StoreResponse::ItemAlreadyAdded(item) => Self::ItemAlreadyAdded(item),
            StoreResponse::Items(item) => Self::Items(item),
            StoreResponse::JsonToSqlite => Self::JsonToSqlite,
            StoreResponse::List(item) => Self::List(item),
            StoreResponse::NothingReturned(item) => Self::NothingReturned(item),
            StoreResponse::Recipes(item) => Self::Recipes(item),
            StoreResponse::RecipeIngredients(item) => Self::RecipeIngredients(item),
            StoreResponse::RefreshList => Self::RefreshList,
            StoreResponse::Sections(item) => Self::Sections(item),
        }
    }
}

#[cfg(test)]
mod tests {
    use common::commands::{Add, Delete, Read};

    use super::*;

    #[tokio::test]
    async fn serve_api() {
        let api = Api::init(StoreType::SqliteInMem).await.unwrap();

        let response = api
            .dispatch(ApiCommand::Add(Add::Recipe { recipe: Recipe::new("fluffy american pancakes").unwrap(), ingredients: Ingredients::from_input_string("135g/4¾oz plain flour, 1 tsp baking powder, ½ tsp salt, 2 tbsp caster sugar, 130ml/4½fl oz milk, 1 large egg, lightly beaten, 2 tbsp melted butter (allowed to cool slightly), plus extra for cooking") }))
            .await
            .unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @r###"

        recipe added: fluffy american pancakes
        "###);

        let response = api.dispatch(ApiCommand::Read(Read::Recipes)).await.unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @"fluffy american pancakes");

        let response = api
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

        let response = api.dispatch(ApiCommand::Read(Read::All)).await.unwrap();

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

        let response = api
            .dispatch(ApiCommand::Delete(Delete::Recipe(
                Recipe::new("fluffy american pancakes").unwrap(),
            )))
            .await
            .unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @r###"
        deleted recipe: 
        fluffy american pancakes
        "###);

        let response = api.dispatch(ApiCommand::Read(Read::Recipes)).await.unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @"");

        let response = api.dispatch(ApiCommand::Read(Read::All)).await.unwrap();

        insta::assert_display_snapshot!(response.to_string().trim(), @"");
    }
}
