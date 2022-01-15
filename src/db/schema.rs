table! {
    subscriptions (id) {
        id -> Binary,
        email -> Text,
        name -> Text,
        subscribed_at -> Timestamp,
    }
}
