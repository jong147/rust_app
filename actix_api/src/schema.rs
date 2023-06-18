// @generated automatically by Diesel CLI.

diesel::table! {
    conductoras (id) {
        id -> Int4,
        #[max_length = 50]
        nombre -> Varchar,
        edad -> Nullable<Int4>,
        #[max_length = 9]
        telefono -> Varchar,
        #[max_length = 35]
        correo -> Varchar,
        #[max_length = 50]
        area -> Varchar,
    }
}
