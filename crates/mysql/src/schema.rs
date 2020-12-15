table! {
    hashes (id) {
        id -> Integer,
        md5 -> Varchar,
        sha256 -> Varchar,
    }
}

table! {
    scores (id) {
        id -> Integer,
        user_id -> Integer,
        sha256 -> Varchar,
        mode -> Integer,
        clear -> Nullable<Integer>,
        epg -> Nullable<Integer>,
        lpg -> Nullable<Integer>,
        egr -> Nullable<Integer>,
        lgr -> Nullable<Integer>,
        egd -> Nullable<Integer>,
        lgd -> Nullable<Integer>,
        ebd -> Nullable<Integer>,
        lbd -> Nullable<Integer>,
        epr -> Nullable<Integer>,
        lpr -> Nullable<Integer>,
        ems -> Nullable<Integer>,
        lms -> Nullable<Integer>,
        combo -> Nullable<Integer>,
        min_bp -> Nullable<Integer>,
        play_count -> Nullable<Integer>,
        clear_count -> Nullable<Integer>,
        date -> Nullable<Datetime>,
    }
}

table! {
    score_snaps (id) {
        id -> Integer,
        user_id -> Integer,
        sha256 -> Varchar,
        mode -> Integer,
        date -> Datetime,
        clear -> Nullable<Integer>,
        old_clear -> Nullable<Integer>,
        score -> Nullable<Integer>,
        old_score -> Nullable<Integer>,
        combo -> Nullable<Integer>,
        old_combo -> Nullable<Integer>,
        min_bp -> Nullable<Integer>,
        old_min_bp -> Nullable<Integer>,
    }
}

table! {
    songs (sha256) {
        sha256 -> Varchar,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        artist -> Nullable<Varchar>,
        sub_artist -> Nullable<Varchar>,
        notes -> Nullable<Integer>,
        length -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        gmail_address -> Varchar,
        name -> Varchar,
        registered_date -> Nullable<Datetime>,
    }
}

joinable!(score_snaps -> users (user_id));
joinable!(scores -> users (user_id));

allow_tables_to_appear_in_same_query!(
    hashes,
    scores,
    score_snaps,
    songs,
    users,
);
