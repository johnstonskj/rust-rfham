//!
//! Macros common to all protocol Implementations.
//!

///
/// Define an enumeration, with a representation type `u8`, that is used as a response value from,
/// or as an argument to, a protocol command.
///
/// At this time the macro is limited to ensuring consistency in the set of traits implemented for
/// all such enumerations.
///
/// # Form
///
/// ```rust
/// define_command_enum!(
///     "doc string" => CommandType {
///         "variant doc string" => VariantName = 0x01,
///         ...
///     }
/// );
/// ```
///
/// The resulting enum will derive `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`,
/// and `Hash` standard traits. Additionally, it will derive traits from the `strum` crate:
/// `EnumIs`, `EnumIter`, `FromRepr`, and `AsRefStr`.
///
/// # Arguments
///
/// * `"doc string"` is the documentation string for the enum.
/// * `CommandType` is the name of the command enum that will be generated.
/// * `"variant doc string"` is the *optional* documentation string for the enum variant.
/// * `VariantName` is the name of the enum variant.
/// * `0x01` is the representation value of the enum variant.
///
#[macro_export]
macro_rules! define_command_enum {
    (
        $doc:literal => $name:ident {
            $( $( $variant_doc:literal => )? $variant_name:ident = $value:literal),+
        }
    ) => {
        #[doc = $doc]
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
            strum::EnumIs, strum::EnumIter, strum::FromRepr, strum::AsRefStr
        )]
        #[repr(u8)]
        #[strum(serialize_all = "kebab-case")]
        pub enum $name {
            $(
                $( #[doc = $variant_doc] )?
                $variant_name = $value
            ),+
        }
    };
}

///
/// Define a structure that is used as a response value from, or as an argument to, a protocol
/// command.
///
/// At this time the macro is limited to ensuring consistency in the set of traits implemented for
/// all such structures.
///
/// # Forms
///
/// ```rust
/// define_command_struct!(
///     "doc string" => CommandType {
///         "field doc string" => field_name: FieldType,
///         ...
///     }
/// );
/// ```
///
/// The resulting struct will derive `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`, and `Hash` traits.
///
/// ```rust
/// define_command_struct!(
///     "doc string" => CommandType no_copy {
///         "field doc string" => field_name: FieldType,
///         ...
///     }
/// );
/// ```
///
/// In this form, the `no_copy` keyword indicates that the struct should **not** derive the `Copy`
/// trait. This is useful for structs that contain fields that are not `Copy`, such as `String` or
/// `Vec`.
///
/// # Arguments
///
/// * `"doc string"` is the documentation string for the struct.
/// * `CommandType` is the name of the command struct that will be generated.
/// * `"field doc string"` is the *optional* documentation string for the struct field.
/// * `field_name` is the name of the struct field.
/// * `FieldType` is the type of the struct field.
///
///
#[macro_export]
macro_rules! define_command_struct {
    (
        $doc_str:literal => $cmd_type:ident no_copy {
            $(
                $( $field_doc:literal => )?
                $field_name:ident : $field_type:ty
            ),+
        }
    ) => {
        #[doc = $doc_str]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $cmd_type {
            $(
                $(
                    #[doc = $field_doc]
                )?
                pub $field_name: $field_type
            ),+
        }
    };
    (
        $doc_str:literal => $cmd_type:ident {
            $(
                $( $field_doc:literal => )?
                $field_name:ident : $field_type:ty
            ),+
        }
    ) => {
        #[doc = $doc_str]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $cmd_type {
            $(
                $(
                    #[doc = $field_doc]
                )?
                pub $field_name: $field_type
            ),+
        }
    };
}
