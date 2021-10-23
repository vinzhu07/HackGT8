use crate::schema::clothes;
use crate::schema::swipes;

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

#[derive(Queryable, Insertable)]
#[table_name="swipes"]
pub struct Swipes {
    pub cloth_id: i32,
    pub love_status: bool
}