table! {
    incidents (data_id) {
        data_id -> Int8,
        iso -> Int8,
        event_id_cnty -> Varchar,
        event_id_no_cnty -> Int8,
        event_date -> Date,
        year -> Int8,
        time_precision -> Int8,
        event_type -> Varchar,
        sub_event_type -> Varchar,
        actor1 -> Varchar,
        assoc_actor_1 -> Varchar,
        inter1 -> Int8,
        actor2 -> Varchar,
        assoc_actor_2 -> Varchar,
        inter2 -> Int8,
        interaction -> Varchar,
        region -> Varchar,
        country -> Varchar,
        admin1 -> Varchar,
        admin2 -> Varchar,
        admin3 -> Varchar,
        location -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
        geo_precision -> Int8,
        source -> Varchar,
        source_scale -> Varchar,
        notes -> Varchar,
        fatalities -> Int8,
        timestamp -> Int8,
        iso3 -> Varchar,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    incidents,
    spatial_ref_sys,
);
