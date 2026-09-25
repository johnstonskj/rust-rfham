
// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! ddl {
    ($conn:expr, $ddl:expr) => {
        println!("Executing DDL: {}", $ddl.replace("\n", " "));
        tracing::trace!("Executing DDL: {}", $ddl.replace("\n", " "));
        $conn.execute(
            &$ddl, ()
        ).map_err(|e|{
            tracing::error!("SQlite error executing DDL: {e}");
            $crate::error::LogError::SqlDefinition(
                $ddl.replace("\n", " "), e
            )
        })?;
    };
}

macro_rules! create_table {
    (
        $name:ident (
            $( $stanza:expr ),+
        )
    ) => {
        format!(
            "CREATE TABLE IF NOT EXISTS {} (\n{}\n)", 
            stringify!($name),
            [
                $(
                    $stanza
                ),+
            ].into_iter().collect::<Vec<_>>().join(",\n")
        )
    };
}

macro_rules! check {
    ($check:expr ) => {
        format!(" CHECK ({})", $check)
    };
    (
        length $field:ident = $len:literal
    ) => {
        check!(
            check!(and: 
                check!(eq: check!(length: $field), $len)
            )
        )
    };
    (
        length $field:ident >= $len:literal
    ) => {
        check!(
            check!(and: 
                check!(gte: check!(length: $field), $len)
            )
        )
    };
    (
        length $field:ident <= $len:literal
    ) => {
        check!(
            check!(and: 
                check!(lte: check!(length: $field), $len)
            )
        )
    };
    (
        length $min:literal <= $field:ident <= $max:literal
    ) => {
        check!(
            check!(and: 
                check!(gte: check!(length: $field), $min),
                check!(lte: check!(length: $field), $max)
            )
        )
    };
    (and: $( $check:expr ),* ) => {
        check!(join: " AND " : $( $check ),* )
    };
    (or: $( $check:expr ),* ) => {
        check!(join: " OR " : $( $check ),* )
    };
    (join: $op:literal : $( $check:expr ),* ) => {
        [
            $(
                $check
            ),*
        ].into_iter().collect::<Vec<_>>().join($op)
    };
    (length: $name:ident) => {
        format!("length({})", stringify!($name))
    };
    (eq: $lhs:expr, $rhs:expr) => {
        format!("{} = {}", $lhs, $rhs)
    };
    (lt: $lhs:expr, $rhs:expr) => {
        format!("{} < {}", $lhs, $rhs)
    };
    (lte: $lhs:expr, $rhs:expr) => {
        format!("{} <= {}", $lhs, $rhs)
    };
    (gt: $lhs:expr, $rhs:expr) => {
        format!("{} > {}", $lhs, $rhs)
    };
    (gte: $lhs:expr, $rhs:expr) => {
        format!("{} >= {}", $lhs, $rhs)
    };
    (regexp: $column:expr, $pattern:expr) => {
        format!("{} REGEXP '{}'", $column, $pattern)
    };
}

macro_rules! row {
    (
        primary_key $field:ident
    ) => {
        format!(
            "    {} INTEGER PRIMARY KEY",
            stringify!($field)
        )
    };
    (
        primary_key
    ) => {
        row!(primary_key id)
    };
    (
        foreign_key $field:ident => $ref_table:ident : $ref_field:ident
    ) => {
        format!(
            "    FOREIGN KEY ({}) REFERENCES {}({})",
            stringify!($field),
            stringify!($ref_table),
            stringify!($ref_field)
        )
    };
    (
        external_key $field:ident
    ) => {
        row!(uuid $field unique not_null genid)
    };
    (
        external_key
    ) => {
        row!(external_key external_id)
    };
    (
        uuid $field:ident $( $modifiers:ident )*
    ) => {
        format!(
            "    {} TEXT{}{}",
            stringify!($field),
            row!(@modifiers $( $modifiers )* ),
            check!(length $field = 36)
        )
    };
    (
        label $field:ident $( $modifiers:ident )*
    ) => {
        format!(
            "    {} TEXT{}{}",
            stringify!($field),
            row!(@modifiers $( $modifiers )* ),
            check!(length $field <= 40)
        )
    };
    (
        semver $field:ident $( $modifiers:ident )*
    ) => {
        format!(
            "    {} TEXT{}{}",
            stringify!($field),
            row!(@modifiers $( $modifiers )* ),
            check!(
                check!(regexp: 
                    stringify!($field), 
                    "^[0-9]+(\\.[0-9]+(\\.[0-9]+)?)?(-[a-zA-Z][a-zA-Z0-9_]*)?$"
                )
            )
        )
    };
    (
        grid $field:ident $( $modifiers:ident )*
    ) => {
        format!(
            "    {} TEXT{}{}",
            stringify!($field),
            row!(@modifiers $( $modifiers )* ),
            check!(
                check!(and:
                    check!(regexp: 
                        stringify!($field), 
                        "^[a-rA-R]{2}([0-9]{2}([a-xA-X]{2}([0-9]{2})?)?([a-xA-X]{2}([0-9]{2})?([a-xA-X]{2}([0-9]{2})?)?)?)?$"
                    ),
                    check!(gte: check!(length: $field), 2),
                    check!(lte: check!(length: $field), 10)
                )
            )
        )
    };
    (
        callsign $field:ident $( $modifiers:ident )*
    ) => {
        format!(
            "    {} TEXT{}{}",
            stringify!($field),
            row!(@modifiers $( $modifiers )* ),
            check!(length 3 <= $field <= 20)
        )
    };
    (
        timestamp $field:ident $( $modifiers:ident )*
    ) => {
        format!(
            "    {} TEXT{}{}",
            stringify!($field),
            row!(@modifiers $( $modifiers )* ),
            check!(length 19 <= $field <= 30)
        )
    };
    (
        timestamp $field:ident before $other:ident $( $modifiers:ident )*
    ) => {
        format!(
            "    {} TEXT{}{}",
            stringify!($field),
            row!(@modifiers $( $modifiers )* ),
            check!(length 19 <= $field <= 30)
        )
    };
    (
        timestamp $field:ident after $other:ident $( $modifiers:ident )*
    ) => {
        format!(
            "    {} TEXT{}{}",
            stringify!($field),
            row!(@modifiers $( $modifiers )* ),
            check!(length 19 <= $field <= 30)
        )
    };
    (integer $field:ident $( $modifiers:ident )*) => {
        row!(@native INTEGER $field $( $modifiers )*)
    };
    (real $field:ident $( $modifiers:ident )*) => {
        row!(@native REAL $field $( $modifiers )*)
    };
    (text $field:ident $( $modifiers:ident )*) => {
        row!(@native TEXT $field $( $modifiers )*)
    };
    (@native $type:ident $field:ident $( $modifiers:ident )*) => {
        format!(
            "    {} {}{}",
            stringify!($field),
            stringify!($type),
            row!(@modifiers $( $modifiers )* )
        )
    };
    (@modifiers $( $modifier:ident )*) => {
        [
            $(
                row!(@modifier $modifier)
            ),*
        ].into_iter().collect::<Vec<&str>>().join("")
    };
    (@modifier not_null) => {
        " NOT NULL"
    };
    (@modifier now) => {
        " DEFAULT CURRENT_TIMESTAMP"
    };
    (@modifier genid) => {
        " GENERATED ALWAYS AS (gen_random_uuid()) STORED"
    };
    (@modifier unique) => {
        " UNIQUE"
    };
    (@modifier) => {
        ""
    };
}

macro_rules! create_index {
    (
        unique $name:ident on $table:ident ( $( $column:ident ),+ )
    ) => {
        format!(
            "CREATE UNIQUE INDEX IF NOT EXISTS {} ON {} ({})",
            stringify!($name),
            stringify!($table),
            vec![$( stringify!($column) ),+].join(", ")
        )
    };
    (
        $name:ident on $table:ident ( $( $column:ident ),+ )
    ) => {
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON {} ({})",
            stringify!($name),
            stringify!($table),
            vec![$( stringify!($column) ),+].join(", ")
        )
    };
}

macro_rules! insert_into {
    (
        $conn:expr, $table:ident ( $( $column:ident => $value:expr ),+ )
    ) => {{
        let columns = vec![$( stringify!($column) ),+];
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            stringify!($table),
            columns.join(", "),
            (1..=columns.len()).map(|i| format!("?{i}")).collect::<Vec<String>>().join(", ")
        );
        tracing::trace!("Executing insert: {}", sql.replace("\n", " "));
        $conn.execute(
            &sql, 
            (
                $(
                    &$value,
                ),+
            )
        ).map_err(|e|{
            tracing::error!("SQlite error executing insert: {e}");
            $crate::error::LogError::SqlDefinition(
                sql.replace("\n", " "), e
            )
        })?;

        
    }};
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_create_table_macro_pri_key() {
        assert_eq!(
            "CREATE TABLE IF NOT EXISTS TestTable (
    id INTEGER PRIMARY KEY
)",
            create_table!(TestTable (
                row!(primary_key id)
            ))
        );
     
        assert_eq!(
            "CREATE TABLE IF NOT EXISTS TestTable (
    id INTEGER PRIMARY KEY
)",
            create_table!(TestTable (
                row!(primary_key)
            ))
        );
    }

    #[test]
    fn test_create_table_macro_uuid() {
        assert_eq!(
            "CREATE TABLE IF NOT EXISTS TestTable (
    id INTEGER PRIMARY KEY,
    uuid_1 TEXT CHECK (length(uuid_1) = 36),
    uuid_2 TEXT UNIQUE CHECK (length(uuid_2) = 36),
    uuid_3 TEXT NOT NULL CHECK (length(uuid_3) = 36),
    uuid_4 TEXT UNIQUE NOT NULL CHECK (length(uuid_4) = 36)
)",
            create_table!(TestTable (
                row!(primary_key id),
                row!(uuid uuid_1),
                row!(uuid uuid_2 unique),
                row!(uuid uuid_3 not_null),
                row!(uuid uuid_4 unique not_null)
            ))
        );
    }

    #[test]
    fn test_create_table_macro_uuid_genid() {
        assert_eq!(
            "CREATE TABLE IF NOT EXISTS TestTable (
    id INTEGER PRIMARY KEY,
    uuid_1 TEXT GENERATED ALWAYS AS (gen_random_uuid()) STORED CHECK (length(uuid_1) = 36),
    uuid_2 TEXT UNIQUE GENERATED ALWAYS AS (gen_random_uuid()) STORED CHECK (length(uuid_2) = 36),
    uuid_3 TEXT NOT NULL GENERATED ALWAYS AS (gen_random_uuid()) STORED CHECK (length(uuid_3) = 36),
    uuid_4 TEXT UNIQUE NOT NULL GENERATED ALWAYS AS (gen_random_uuid()) STORED CHECK (length(uuid_4) = 36)
)",
            create_table!(TestTable (
                row!(primary_key id),
                row!(uuid uuid_1 genid),
                row!(uuid uuid_2 unique genid),
                row!(uuid uuid_3 not_null genid),
                row!(uuid uuid_4 unique not_null genid)
            ))
        );
    }

    #[test]
    fn test_create_table_macro_external_key() {
        assert_eq!(
            "CREATE TABLE IF NOT EXISTS TestTable (
    id INTEGER PRIMARY KEY,
    external_id TEXT UNIQUE NOT NULL GENERATED ALWAYS AS (gen_random_uuid()) STORED CHECK (length(external_id) = 36),
    sync_id TEXT UNIQUE NOT NULL GENERATED ALWAYS AS (gen_random_uuid()) STORED CHECK (length(sync_id) = 36)
)",
            create_table!(TestTable (
                row!(primary_key id),
                row!(external_key),
                row!(external_key sync_id)
            ))
        );
    }

    #[test]
    fn test_create_table_macro_label() {
        assert_eq!(
            "CREATE TABLE IF NOT EXISTS TestTable (
    id INTEGER PRIMARY KEY,
    label_1 TEXT CHECK (length(label_1) <= 40),
    label_2 TEXT UNIQUE CHECK (length(label_2) <= 40),
    label_3 TEXT NOT NULL CHECK (length(label_3) <= 40),
    label_4 TEXT UNIQUE NOT NULL CHECK (length(label_4) <= 40)
)",
            create_table!(TestTable (
                row!(primary_key id),
                row!(label label_1),
                row!(label label_2 unique),
                row!(label label_3 not_null),
                row!(label label_4 unique not_null)
            ))
        );
    }

    #[test]
    fn test_create_table_macro_semver() {
        assert_eq!(
            "CREATE TABLE IF NOT EXISTS TestTable (
    id INTEGER PRIMARY KEY,
    semver_1 TEXT CHECK (semver_1 REGEXP '^[0-9]+(\\.[0-9]+(\\.[0-9]+)?)?(-[a-zA-Z][a-zA-Z0-9_]*)?$'),
    semver_2 TEXT UNIQUE CHECK (semver_2 REGEXP '^[0-9]+(\\.[0-9]+(\\.[0-9]+)?)?(-[a-zA-Z][a-zA-Z0-9_]*)?$'),
    semver_3 TEXT NOT NULL CHECK (semver_3 REGEXP '^[0-9]+(\\.[0-9]+(\\.[0-9]+)?)?(-[a-zA-Z][a-zA-Z0-9_]*)?$'),
    semver_4 TEXT UNIQUE NOT NULL CHECK (semver_4 REGEXP '^[0-9]+(\\.[0-9]+(\\.[0-9]+)?)?(-[a-zA-Z][a-zA-Z0-9_]*)?$')
)",
            create_table!(TestTable (
                row!(primary_key id),
                row!(semver semver_1),
                row!(semver semver_2 unique),
                row!(semver semver_3 not_null),
                row!(semver semver_4 unique not_null)
            ))
        );
    }

    #[test]
    fn test_create_table_macro_timestamp() {
        assert_eq!(
            "CREATE TABLE IF NOT EXISTS TestTable (
    id INTEGER PRIMARY KEY,
    created TEXT UNIQUE NOT NULL DEFAULT CURRENT_TIMESTAMP CHECK (length(created) >= 19 AND length(created) <= 30),
    created TEXT UNIQUE DEFAULT CURRENT_TIMESTAMP CHECK (length(created) >= 19 AND length(created) <= 30),
    created TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP CHECK (length(created) >= 19 AND length(created) <= 30),
    created TEXT DEFAULT CURRENT_TIMESTAMP CHECK (length(created) >= 19 AND length(created) <= 30),
    started TEXT CHECK (length(started) >= 19 AND length(started) <= 30),
    ended TEXT CHECK (length(ended) >= 19 AND length(ended) <= 30)
)",
            create_table!(TestTable (
                row!(primary_key id),
                row!(timestamp created unique not_null now),
                row!(timestamp created unique now),
                row!(timestamp created not_null now),
                row!(timestamp created now),
                row!(timestamp started),
                row!(timestamp ended)
            ))
        );
    }

    #[test]
    fn test_create_table_macro_foreign_key() {
        assert_eq!(
            "CREATE TABLE IF NOT EXISTS TestTable (
    id INTEGER PRIMARY KEY,
    other_id INTEGER,
    FOREIGN KEY (other_id) REFERENCES OtherTable(id)
)",
            create_table!(TestTable (
                row!(primary_key id),
                row!(integer other_id),
                row!(foreign_key other_id => OtherTable : id)
            ))
        );
    }

    #[test]
    fn test_create_index_macro_one() {
        assert_eq!(
            "CREATE INDEX IF NOT EXISTS TestIndex ON TestTable (external_id)",
            create_index!(TestIndex on TestTable (
                external_id
            ))
        );
    }

    #[test]
    fn test_create_index_macro_multiple() {
        assert_eq!(
            "CREATE INDEX IF NOT EXISTS TestIndex ON TestTable (col_1, col_2)",
            create_index!(TestIndex on TestTable (
                col_1,
                col_2
            ))
        );
    }

    #[test]
    fn test_create_index_macro_unique() {
        assert_eq!(
            "CREATE UNIQUE INDEX IF NOT EXISTS TestIndex ON TestTable (external_id)",
            create_index!(unique TestIndex on TestTable (
                external_id
            ))
        );
    }
}
