use crate::schema::clothes;

#[derive(Queryable, Insertable)]
#[table_name="clothes"]
pub struct Clothes {
    pub id: i32,
    pub gender: String,
    pub master_category: String,
    pub sub_category: String,
    pub article_type: String,
    pub base_color: String,
    pub season: String,
    pub usage: String,
    pub product_display_name: String
}