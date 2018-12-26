use juniper::{Context as JuniperContext, FieldResult, FieldError};
use juniper_rocket::{GraphQLResponse, GraphQLRequest};

use crate::models::{Todo, NewTodo};
use crate::db::PrimaryDb;

pub struct Context {
    pub connection: PrimaryDb
}

impl JuniperContext for Context {}

graphql_object!(Todo: () |&self| {
    description: "A todo item that can be marked as completed"

    field id() -> i32 as "The unique id of the todo item" {
        self.id
    }

    field title() -> &str as "The user-editable title" {
        &self.title
    }

    field completed() -> bool as "Determines whether the user has completed the item or not" {
        self.completed
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field todoItems(&executor) -> FieldResult<Vec<Todo>> {
        use crate::schema::todos::dsl;
        use diesel::{RunQueryDsl, QueryDsl};

        dsl::todos.order(dsl::id)
            .load::<Todo>(&*executor.context().connection)
            .map_err(Into::into)
    }
});


pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    field add_todo(&executor, title: String, completed: bool) -> FieldResult<Todo>
        as "Create a new todo item and return it"
    {
        use crate::schema::todos::dsl;

        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

    (*(executor.context().connection)).immediate_transaction(|| {
            let new_post = NewTodo {
                title: &title,
                completed: completed,
            };

            diesel::insert_into(crate::schema::todos::table).values(&new_post)
                .execute(&*executor.context().connection)?;

            dsl::todos.order(dsl::id.desc())
                .first::<Todo>(&*executor.context().connection)
        }).map_err(Into::into)
    }

    field update_todo(&executor, id: i32, completed: Option<bool>, title: Option<String>) -> FieldResult<Option<Todo>>
        as "Update an existing todo item.\
        \
        Will only updated the provided fields - if either `completed` or `title`\
        are omitted or null, they will be ignored.\
        \
        The mutation will return null if no todo item with the specified ID could be found."
    {
        use crate::schema::todos::dsl;
        use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};

        let updated = diesel::update(dsl::todos.find(id))
            .set((
                completed.map(|completed| dsl::completed.eq(completed)),
                title.map(|title| dsl::title.eq(title)),
            ))
            .execute(&*executor.context().connection)?;

        if updated == 0 {
            Ok(None)
        } else {
            Ok(Some(dsl::todos.find(id)
                .get_result::<Todo>(&*executor.context().connection)?))
        }
    }
});
