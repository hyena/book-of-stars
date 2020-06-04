table! {
    quoths (id) {
        id -> BigInt,
        author -> Nullable<BigInt>,
        guild -> Nullable<BigInt>,
        starred_by -> Nullable<BigInt>,
        content -> Text,
        legacy -> Bool,
        legacy_author_fallback -> Nullable<Text>,
    }
}
