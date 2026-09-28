use crate::app::App;
use crate::as_user_view;
use crate::users_roles_groups_query;
use crate::users_view;
use datasources::{collect_reader, DatasourceError, FlatToNestedMapper, Mapper, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListUsersRolesGroups;

impl ListUsersRolesGroups {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        let query = users_roles_groups_query::create(app.provider());
        let reader = app.command().execute_reader_statement(&query, None).await?;
        let rows = collect_reader(reader.as_ref()).await?;
        let mapped = Mapper::map(&FlatToNestedMapper::new(users_view::type_map()), rows);
        Ok(serde_json::to_string(&as_user_view::from_mapped(&mapped))
            .map_err(|error| DatasourceError::message(error.to_string()))?)
    }
}
