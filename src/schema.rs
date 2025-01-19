// @generated automatically by Diesel CLI.

diesel::table! {
    test_table (id) {
        id -> Uuid,
        name -> Text,
    }
}
