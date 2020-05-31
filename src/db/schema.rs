table! {
    authors (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    categories (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    paper_authors (paper_id, author_id) {
        paper_id -> Integer,
        author_id -> Integer,
    }
}

table! {
    papers (id) {
        id -> Integer,
        title -> Text,
        url -> Text,
        pdf_url -> Text,
        category_id -> Integer,
        summary -> Text,
        comment -> Text,
        accepted -> Integer,
        updated -> Timestamp,
        published -> Timestamp,
        created -> Timestamp,
    }
}

table! {
    slack_notifications (id) {
        id -> Integer,
        paper_id -> Integer,
        slack_url -> Text,
        updated_at -> Timestamp,
        send -> Bool,
        created -> Timestamp,
    }
}

joinable!(paper_authors -> authors (author_id));
joinable!(paper_authors -> papers (paper_id));
joinable!(papers -> categories (category_id));
joinable!(slack_notifications -> papers (paper_id));

allow_tables_to_appear_in_same_query!(
    authors,
    categories,
    paper_authors,
    papers,
    slack_notifications,
);
