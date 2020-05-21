table! {
    score_log (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        data -> Nullable<Mediumtext>,
    }
}

table! {
    score_logs (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        data -> Nullable<Mediumtext>,
    }
}

table! {
    users (id) {
        id -> Integer,
        player_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
    }
}

joinable!(score_log -> users (user_id));
joinable!(score_logs -> users (user_id));

allow_tables_to_appear_in_same_query!(score_log, score_logs, users,);
