use diesel::prelude::*;

#[derive(Insertable, Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::model)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Model {
    pub name: String,
}