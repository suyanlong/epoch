table! {
    admin_roles (id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        id -> Bigint,
        user_name -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        role -> Nullable<Varchar>,
    }
}

table! {
    assets (id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        id -> Bigint,
        user_id -> Bigint,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        available_asset -> Nullable<Decimal>,
        total_asset -> Nullable<Decimal>,
    }
}

table! {
    behavior_details (id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        id -> Bigint,
        user_id -> Nullable<Bigint>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        a_factor -> Nullable<Bigint>,
        h_factor -> Nullable<Bigint>,
        factor -> Nullable<Integer>,
        state -> Nullable<Integer>,
    }
}

table! {
    nt_tx_details (id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        id -> Bigint,
        user_id -> Bigint,
        amount -> Nullable<Decimal>,
        tx_type -> Nullable<Integer>,
        tx_state -> Nullable<Integer>,
        comment -> Nullable<Varchar>,
        object_id -> Nullable<Bigint>,
    }
}

table! {
    nt_user_stashes (id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        id -> Bigint,
        user_name -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        mobile -> Nullable<Varchar>,
        id_card -> Varchar,
        user_type -> Nullable<Integer>,
        node_type -> Nullable<Integer>,
        is_ok -> Nullable<Bool>,
    }
}

table! {
    nt_users (id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        id -> Bigint,
        user_name -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        mobile -> Nullable<Varchar>,
        id_card -> Varchar,
        user_type -> Nullable<Integer>,
        node_type -> Nullable<Integer>,
        is_ok -> Nullable<Bool>,
    }
}

table! {
    pay_types (type_) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        #[sql_name = "type"]
        type_ -> Integer,
        uint -> Nullable<Integer>,
        gos -> Nullable<Varchar>,
        rmb -> Nullable<Varchar>,
    }
}

table! {
    recharge_records (id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        id -> Bigint,
        user_id -> Nullable<Bigint>,
        amount -> Nullable<Decimal>,
        order_id -> Nullable<Varchar>,
        order_type -> Nullable<Integer>,
        state -> Nullable<Integer>,
        comment -> Nullable<Varchar>,
    }
}

table! {
    space_txes (id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        id -> Bigint,
        user_id -> Bigint,
        tx_state -> Nullable<Integer>,
        space_amount -> Nullable<Decimal>,
        expired_at -> Nullable<Timestamp>,
        tx_type -> Nullable<Integer>,
        object_id -> Nullable<Bigint>,
    }
}

table! {
    speed_types (user_id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        user_id -> Bigint,
        speed_type -> Nullable<Integer>,
        first_login -> Nullable<Bool>,
    }
}

table! {
    tasks (id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        id -> Bigint,
        task_name -> Nullable<Varchar>,
        star_time -> Nullable<Bigint>,
        end_time -> Nullable<Bigint>,
        start_time -> Nullable<Timestamp>,
    }
}

table! {
    tops (user_id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        user_id -> Bigint,
        top0 -> Nullable<Bigint>,
        top1 -> Nullable<Bigint>,
        top2 -> Nullable<Bigint>,
        top3 -> Nullable<Bigint>,
    }
}

table! {
    user_relations (id) {
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        id -> Bigint,
        user_id -> Bigint,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        object_id -> Bigint,
    }
}

allow_tables_to_appear_in_same_query!(
    admin_roles,
    assets,
    behavior_details,
    nt_tx_details,
    nt_user_stashes,
    nt_users,
    pay_types,
    recharge_records,
    space_txes,
    speed_types,
    tasks,
    tops,
    user_relations,
);
