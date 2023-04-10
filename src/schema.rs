// @generated automatically by Diesel CLI.

diesel::table! {
    action (id) {
        id -> Integer,
        label -> Nullable<Varchar>,
    }
}

diesel::table! {
    catalog (id) {
        id -> Integer,
        label -> Nullable<Varchar>,
        note -> Nullable<Varchar>,
    }
}

diesel::table! {
    role (id) {
        id -> Integer,
        label -> Nullable<Varchar>,
        note -> Nullable<Varchar>,
    }
}

diesel::table! {
    role_action_catalog (id_role, id_action, id_catalog) {
        id_role -> Integer,
        id_action -> Integer,
        id_catalog -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        hash -> Varchar,
    }
}

diesel::table! {
    user_role (id_user, id_role) {
        id_user -> Integer,
        id_role -> Integer,
    }
}

diesel::table! {
    user_session (id) {
        id -> Integer,
        id_user -> Integer,
        jwt -> Varchar,
        expiration -> Timestamp,
        ip_address -> Varchar,
        last_login -> Timestamp,
        last_online -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    action,
    catalog,
    role,
    role_action_catalog,
    user,
    user_role,
    user_session,
);
