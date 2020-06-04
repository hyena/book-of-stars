table! {
    quoths (id) {
        id -> Nullable<Integer>,
        author -> Nullable<Integer>,
        starred_by -> Nullable<Integer>,
        content -> Text,
        legacy -> Bool,
    }
}
