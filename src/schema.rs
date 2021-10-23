table! {
    clothes (id) {
        id -> Integer,
        gender -> Text,
        master_category -> Text,
        sub_category -> Text,
        article_type -> Text,
        base_color -> Text,
        season -> Text,
        usage -> Text,
        product_display_name -> Text,
    }
}

table! {
    swipes (id) {
        id -> Integer,
        cloth_id -> Integer,
        love_status -> Bool,
    }
}

joinable!(swipes -> clothes (cloth_id));

allow_tables_to_appear_in_same_query!(
    clothes,
    swipes,
);
