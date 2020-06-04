table! {
    quoths (id) {
        id -> Integer,
        author -> Nullable<Integer>,
        starred_by -> Nullable<Integer>,
        content -> Text,
        legacy -> Bool,
        legacy_author_fallback -> Nullable<Text>,
    }
}
