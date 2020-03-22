use super::schema::dog;

#[derive(Queryable, Serialize)]
pub struct Dog {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="dog"]
pub struct NewDog<'a> {
    pub name: &'a str,
}
