//!
//! Macros for CAT protocol implementations.
//!

///
/// Elecraft CAT commands are defined using the `define_command!` macro, which generates a command
/// structure with the specified fields.
////
/// # Forms
///
/// ```rust
/// define_cat_command!(
///     "doc string" => CommandType {
///         "field doc string" field_name: field_type,
///         ...
///     }
/// );
/// ```
///
/// The resulting struct will derive `Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`, and `Hash` traits.
///
/// ```rust
/// define_cat_command!(
///     "doc string" => CommandType no_copy {
///         "field doc string" field_name: field_type,
///         ...
///     }
/// );
/// ```
///
/// In this form, the `no_copy` keyword indicates that the struct should **not** derive the `Copy`
/// trait. This is useful for structs that contain fields that are not `Copy`, such as `String` or
/// `Vec`.
///
/// ```rust
/// define_cat_command!(
///     "doc string" => CommandType { state }
/// );
/// ```
///
/// This form is a shorthand for commands that have a single boolean field named `on`, which
/// represents the On/Off state. This also adds two constructor functions, `turn_on` and `turn_off`,
/// to create instances of the command with the `on` field set to `true` or `false`, respectively.
///
/// ```rust
/// define_cat_command!(
///     "doc string" => CommandType
/// );
/// ```
///
/// Defines a command struct with no fields; this is useful for many *get* commands.
///
/// # Arguments
///
/// * `"doc string"` is a documentation string that will be applied to the command structure.
/// * `CommandType` is the name of the command structure that will implement the `Command` trait.
/// * `"field doc string"` is an optional documentation string that will be applied to the command
///   structure field.
/// * `field_name` is the name of the command structure field.
/// * `field_type` is the type of the command structure field.
///
#[macro_export]
macro_rules! define_cat_command {
    (
        $doc_str:literal => $cmd_type:ident no_copy {
            $( $( $field_doc:literal )? $field:ident : $type:ty),*
        }
    ) => {
        #[doc = $doc_str]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $cmd_type {
            $(
                $(
                    #[doc = $field_doc]
                )?
                pub $field: $type
            ),*
        }
    };
    (
        $doc_str:literal =>  $cmd_type:ident {
            $( $( $field_doc:literal )? $field:ident : $type:ty),*
        }
    ) => {
        #[doc = $doc_str]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $cmd_type {
            $(
                $(
                    #[doc = $field_doc]
                )?
                pub $field: $type
            ),*
        }
    };
    (
        $doc_str:literal => $cmd_type:ident { state }
    ) => {
        define_cat_command!($doc_str => $cmd_type {
            "Represents the On/Off state." on: bool
        });
    };
    (
        $doc_str:literal => $cmd_type:ident
    ) => {
        #[doc = $doc_str]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $cmd_type;
    };
}

// ------------------------------------------------------------------------------------------------

///
/// Provide an implementation of the `Command` trait for a command struct. The command struct can
/// easily be defined using the `define_cat_command!` macro.
///
/// # Forms
///
/// ```rust
/// impl_cat_command!(
///     CommandType => "id" with Some argument_bytes, if validate_args
/// );
/// impl_cat_command!(
///     CommandType => "id" with Some argument_bytes
/// );
/// impl_cat_command!(
///     CommandType => "id" with argument_bytes, if validate_args
/// );
/// impl_cat_command!(
///     CommandType => "id" with argument_bytes
/// );
/// ```
/// Primary form, this provides a lot of control over the validation of arguments and the generation
/// of argument bytes.
///
/// ```rust
/// impl_cat_command!(
///     CommandType => "id" for boolean field_name
/// );
/// ```
///
/// The `for boolean field_name` variant is a shorthand for commands that have a single boolean
/// field, which will be converted to '1' or '0' in the command message.
///
/// ```rust
/// impl_cat_command!(
///     CommandType => "id" format field_name uint width, if validate_args
/// );
/// impl_cat_command!(
///     CommandType => "id" format field_name uint width
/// );
/// impl_cat_command!(
///     CommandType => "id" format field_name int width, if validate_args
/// );
/// impl_cat_command!(
///     CommandType => "id" format field_name int width
/// );
/// ```
///
/// The `format` variants are shorthands for commands that have a single integer field that will be
/// sent as an ASCII-encoded decimal string in the command message. The `uint` variant is for
/// unsigned integers, and the `int` variant is for signed integers. The `width` parameter specifies
/// the minimum width of the ASCII-encoded decimal string, it will be padded with leading zeros if
/// necessary.
///
/// ```rust
/// impl_cat_command!(
///     CommandType => "id" for state
/// );
/// ```
///
/// The `for state` variant is a shorthand for commands that have a single boolean field named `on`,
/// which represents the On/Off state. It also adds two constructor functions, `turn_on` and
/// `turn_off`, to create instances of the command with the `on` field set to `true` or `false`,
/// respectively.
///
/// ```rust
/// impl_cat_command!(
///     CommandType => "id" for as byte field_name, if validate_args
/// );
/// impl_cat_command!(
///     CommandType => "id" for as byte field_name
/// );
/// ```
///
/// The `for as byte field_name` variant is a shorthand for commands that have a single field that
/// will be sent as a single byte in the command message. The field must be of a type that can be
/// cast to `u8`.
///
/// ```rust
/// impl_cat_command!(CommandType => "id");
/// impl_cat_command!(CommandType);
/// ```
///
/// The final two variants are used by manual implementations of the `Command` trait, where the
/// command ID and argument bytes are handled manually, and the `Command` trait is implemented
/// directly on the command structure.
///
/// # Arguments
///
/// * `CommandType` is the name of the command structure that will implement the `Command` trait.
/// * `"id"` is the command identifier literal string that will be sent to the transceiver.
/// * `argument_bytes` is a function that takes a reference to the command structure and returns an
///   `Option<Vec<u8>>` wrapped in a `Result`. The bytes returned will be sent as the body of the
///   command message.
/// * `validate_args` is an optional function that takes a reference to the command structure and
///   returns a `Result<(), RigError>`. It is used to validate the command arguments before
///   processing and sending the command. Specifically, it is called before `argument_bytes`.
///
#[macro_export]
macro_rules! impl_cat_command {
    (
        $type:ident => $id:literal
        with Some $arg_fn:expr
        $(, if $valid_fn:expr )?
    ) => {
        impl_cat_command!($type => $id with |arg|{
            Ok(Some($arg_fn(arg)))
        } $(, if $valid_fn)?);
    };
    (
        $type:ident => $id:literal
        with $arg_fn:expr
        $(, if $valid_fn:expr )?
    ) => {
        impl $crate::protocol::Command for $type {
            impl_cat_command!($type);
            impl_cat_command!(@id $id);

            $(
                #[inline(always)]
                fn validate(&self) -> Result<(), RigError> {
                    $valid_fn(self)
                }
            )?

            fn argument_bytes(&self) -> Result<Option<Vec<u8>>, RigError> {
                $arg_fn(self)
            }
        }
    };
    (
        $type:ident => $id:literal
        format $field_name:ident uint $width:literal
        $(, if $valid_fn:expr )?
    ) => {
        impl_cat_command!($type => $id with Some |cmd: &$type| {
            $crate::protocol::cat::common::format_uint_ascii(cmd.$field_name, $width)
        } $(, if $valid_fn)?);
    };
    (
        $type:ident => $id:literal
        format $field_name:ident int $width:literal
        $(, if $valid_fn:expr )?
    ) => {
        impl_cat_command!($type => $id with Some |cmd: &$type| {
            $crate::protocol::cat::common::format_int_ascii(cmd.$field_name, $width)
        } $(, if $valid_fn)?);
    };
    ($type:ident => $id:literal for boolean $field:ident) => {
        impl $crate::protocol::Command for $type {
            impl_cat_command!($type);
            impl_cat_command!(@id $id);

            fn argument_bytes(&self) -> Result<Option<Vec<u8>>, RigError> {
                Ok(Some(
                    vec![
                        if self.$field {
                            b'1'
                        } else {

                             b'0'
                        }
                    ]
                ))
            }
        }
    };
     ($type:ident => $id:literal for state) => {
        impl_cat_command!($type => $id for boolean on);
        impl $type {
            #[inline(always)]
            pub const fn turn_on() -> Self {
                Self { on: true }
            }
            #[inline(always)]
            pub const fn turn_off() -> Self {
                Self { on: false }
            }
        }
    };
    ($type:ident => $id:literal for as byte $field:ident $(, if $valid_fn:expr )?) => {
        impl $crate::protocol::Command for $type {
            impl_cat_command!($type);
            impl_cat_command!(@id $id);

            $(
                #[inline(always)]
                fn validate(&self) -> Result<(), RigError> {
                    $valid_fn(self)
                }
            )?

            #[allow(trivial_numeric_casts)]
            fn argument_bytes(&self) -> Result<Option<Vec<u8>>, RigError> {
                Ok(Some(vec![self.$field as u8]))
            }
        }
    };
    ($type:ident => $id:literal) => {
        impl $crate::protocol::Command for $type {
            impl_cat_command!($type);
            impl_cat_command!(@id $id);
        }
    };
    ($type:ident) => {
        const MESSAGE_TERMINATOR: u8 = b';';

        fn message_preamble(&self) -> Option<&[u8]> {
            None
        }

        fn to_message(&self) -> Result<Vec<u8>, RigError> {
            Ok($crate::protocol::cat::make_message(
                self.command_id(),
                self.argument_bytes()?,
                Self::MESSAGE_TERMINATOR,
            ))
        }
    };
    (@id $id:literal) => {
            fn command_id(&self) -> &[u8] {
                $id
            }
   };
}

// ------------------------------------------------------------------------------------------------

///
/// Provide an implementation of the `CommandWithResponse` trait for a command struct. The command
/// should also implement the `Command` trait, which can be easily defined using the
/// `impl_cat_command!` macro.
///
/// # Forms
///
/// ```rust
/// impl_cat_command_with_response!(
///     CommandType => 1, parse_fn => ResponseType
/// )
/// ```
///
/// The primary form, this provides direct control over the parsing of the response bytes into the
/// expected response type.
///
/// ```rust
/// impl_cat_command_with_response!(
///     CommandType => try_from 1, ResponseType
/// )
/// ```
/// This form indicates that the response type implements the `TryFrom<&[u8]>` trait, and the
/// corresponding number of response bytes will be passed to the trait implementation.
///
/// ```rust
/// impl_cat_command_with_response!(
///     CommandType => try_from enum ResponseType
/// )
/// ```
///
/// This form indicates that the response type is an enum that implements the `strum` crate's
/// `FromRepr` trait, and a single response byte will be converted into the expected response type
/// using this trait.
///
/// ```rust
/// impl_cat_command_with_response!(
///     CommandType => string
/// )
/// ```
///
/// This form indicates that the response type is a `String`, and all bytes before the terminating
/// `';'` will be converted into a UTF-8 string.
///
/// ```rust
/// impl_cat_command_with_response!(
///     CommandType => boolean
/// )
/// ```
///
/// This form indicates that the response type is a `bool`, and a single response byte will be
/// converted, using `b'0'` for false and `b'1'` for true.
///
/// # Arguments
///
/// * `CommandType` is the name of the command structure that implements the
///   `CommandWithResponse` trait.
/// * `1` is the expected length of the response in bytes.
/// * `parse_fn` is a function that takes a byte slice and returns a
///   `Result<ResponseType, RigError>`. It is used to parse the response bytes into the expected
///   response type.
/// * `ResponseType` is the type of the expected response.
///
#[macro_export]
macro_rules! impl_cat_command_with_response {
    (
        $type:ident => $len:literal, $parse_fn:expr => $response_type:ty
    ) => {
        impl $crate::protocol::CommandWithResponse for $type {
            type Response = $response_type;

            fn expected_response_length(&self) -> usize {
                $len
            }

            fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError> {
                let response =
                    $crate::protocol::cat::common::validate_response(
                        bytes,
                        <Self as $crate::protocol::Command>::command_id(self),
                        self.expected_response_length()
                    )?;
                $parse_fn(response)
            }
        }
    };
    (
        $type:ident => try_from $len:literal $inner:ty
    ) => {
        impl $crate::protocol::CommandWithResponse for $type {
            type Response = $inner;

            fn expected_response_length(&self) -> usize {
                $len
            }

            fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError> {
                let response =
                    $crate::protocol::cat::common::validate_response(
                        bytes,
                        <Self as $crate::protocol::Command>::command_id(self),
                        self.expected_response_length()
                    )?;
                <$inner as ::std::convert::TryFrom<&[u8]>>::try_from(response)
            }
        }
    };
    (
        $type:ident => try_from enum $inner:ty
    ) => {
        impl $crate::protocol::CommandWithResponse for $type {
            type Response = $inner;

            fn expected_response_length(&self) -> usize {
                1
            }

            fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError> {
                if self.expected_response_length() != 1 {
                    Err($crate::error::invalid_response_length(
                        1,
                        self.expected_response_length(),
                    ))
                } else {
                    let response =
                        $crate::protocol::cat::common::validate_response(
                            bytes,
                            <Self as $crate::protocol::Command>::command_id(self),
                            self.expected_response_length()
                        )?;
                    <$inner>::from_repr(response[0]).ok_or_else(
                        || {
                            $crate::error::enum_parse(
                                $crate::protocol::cat::common::dbg_string_from_ascii(response),
                                stringify!($inner),
                            )
                        }
                    )
                }
            }
        }
    };
    (
        $type:ident => string
    ) => {
        impl $crate::protocol::CommandWithResponse for $type {
            type Response = String;

            fn expected_response_length(&self) -> usize {
                0
            }

            fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError> {
                $crate::protocol::cat::common::string_from_ascii(bytes)
            }
        }
    };
    (
        $type:ident => boolean
    ) => {
        impl $crate::protocol::CommandWithResponse for $type {
            type Response = bool;

            fn expected_response_length(&self) -> usize {
                1
            }

            fn parse(&self, bytes: &[u8]) -> Result<Self::Response, RigError> {
                let response =
                    $crate::protocol::cat::common::validate_response(
                        bytes,
                        <Self as $crate::protocol::Command>::command_id(self),
                        self.expected_response_length()
                    )?;
                match response[0] {
                    b'0' => Ok(false),
                    b'1' => Ok(true),
                    _ => {
                        ::tracing::error!(
                            "Couldn't convert {:02X?} into a value of type bool, error: expecting '1' or '0'",
                            response[0]
                        );
                        Err(RigError::InvalidResponseData {
                            data: response.to_vec(),
                        })
                    }
                }
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------

///
/// For commands that have a enumeration argument, this macro provides a set of constructor
/// functions that create instances of the command matching the enumeration variants.
///
/// # Forms
///
/// ```rust
/// impl_set_cat_command_from_enum!(
///     CommandType, EnumType => field_name {
///         VariantName => constructor_method_name,
///         ...
///     }
/// );
/// ```
///
/// The primary form, each variant named (not all are required) will have a constructor method
/// added.
///
/// ```rust
/// impl_set_cat_command_from_enum!(
///     CommandType, boolean => field_name
///         true_constructor_method_name,
///         false_constructor_method_name
/// );
/// ```
///
/// This form is a shorthand for commands that have a single boolean field, effectively treating the
/// `bool` type as an enumeration.
///
/// # Arguments
///
/// * `CommandType` is the name of the command structure that implements the `Command` trait.
/// * `EnumType` is the name of the enum type that represents the possible values for the command.
/// * `field_name` is the name of the field in the command structure that will be set by the macro.
/// * `VariantName` is the name of the variant in the enum type that corresponds to a specific value
///   for the command.
/// * `constructor_method_name` is the name of the method that will be generated to create an
///   instance whose `field_name` is set to the corresponding `VariantName`.
///
/// **If** the macro uses the second form, with `boolean => field_name`:
///
/// * `true_constructor_method_name` is the name of the method that will be generated to create an
///   instance whose `field_name` is set to `true`.
/// * `false_constructor_method_name` is the name of the method that will be generated to create an
///   instance whose `field_name` is set to `false`.
///
#[macro_export]
macro_rules! impl_set_cat_command_from_enum {
    (
        $type:ident, boolean => $field:ident, $true_method:ident, $false_method:ident
    ) => {
        impl $type {
            #[inline(always)]
            pub const fn $true_method() -> Self {
                Self {
                    $field: true,
                }
            }
            #[inline(always)]
            pub const fn $false_method() -> Self {
                Self {
                    $field: false,
                }
            }
        }
    };
    (
        $type:ident, $enum:ident => $field:ident {
            $( $variant:ident => $( $method_doc:literal, )? $method:ident ),+
        }
    ) => {
        impl $type {
            $(
                $(
                    #[doc = $method_doc]
                )?
                #[inline(always)]
                pub const fn $method() -> Self {
                    Self {
                        $field: $enum::$variant
                    }
                }
            )+
        }
    };
}
// ------------------------------------------------------------------------------------------------

///
/// Parse a single bit flag ('0' as false, non-'0' as true) from a byte within an array of bytes, or
/// from a single byte.
///
/// # Forms
///
/// ```rust
/// parse_bit_flag!(
///     bytes [ byte_idx : bit_idx ] == state
/// )
/// parse_bit_flag!(
///     bytes [ byte_idx : bit_idx ] ON
/// )
/// parse_bit_flag!(
///     bytes [ byte_idx : bit_idx ] OFF
/// )
/// parse_bit_flag!(
///     bytes [ byte_idx : bit_idx ]
/// )
/// ```
///
/// This form is used to parse a single bit flag from a byte within an array of bytes.
///
/// ```rust
/// parse_bit_flag!(
///     byte [ bit_idx ] == state
/// )
/// parse_bit_flag!(
///     byte [ bit_idx ] ON
/// )
/// parse_bit_flag!(
///     byte [ bit_idx ] OFF
/// )
/// parse_bit_flag!(
///     byte [ bit_idx ]
/// )
/// ```
///
/// This form is used to parse a single bit flag from a single byte.
///
/// # Arguments
///
/// * `bytes` — the identifier of a byte array containing the packed flags.
/// * `byte_idx` — the index of the byte in the array containing the flag.
/// * `bit_idx` — the index of the bit in the byte containing the flag.
/// * `state` — the expected state of the flag, either `0` or `1`.
/// * `byte` — the byte containing the flag when not using an array.
/// * `ON` — shorthand for `== 1`, `OFF` — shorthand for `== 0`.
///
#[macro_export]
macro_rules! parse_bit_flag {
    ($bytes:ident [ $byte:literal : $bit:literal ] == $state:literal) => {
        ((1 << $bit) & $bytes[$byte]) == $state
    };
    ($bytes:ident [ $byte:literal : $bit:literal ] ON) => {
        parse_bit_flag!($bytes[$byte:$bit] == 1)
    };
    ($bytes:ident [ $byte:literal : $bit:literal ] OFF) => {
        parse_bit_flag!($bytes[$byte:$bit] == 0)
    };
    // --------------------------------------------------------------------------------------------
    ($byte:ident [ $bit:literal ] == $state:literal) => {
        ((1 << $bit) & $byte) == $state
    };
    ($byte:ident [ $bit:literal ] ON) => {
        parse_bit_flag!($byte[$bit] == 1)
    };
    ($byte:ident [ $bit:literal ] OFF) => {
        parse_bit_flag!($byte[$bit] == 0)
    };
}

// ------------------------------------------------------------------------------------------------

///
/// Parse a single option byte from an `OM` response.
///
/// Evaluates to `true` if the byte matches `$present`, `false` if it is `b'-'`,
/// or returns `Err(RigError::InvalidResponseData)` for any other value.
///
/// # Form
///
/// ```rust
/// parse_installed_option!(option_name => bytes [ index ] == present)
/// ```
///
/// # Arguments
///
/// TBD
///
///
#[macro_export]
macro_rules! parse_installed_option {
    ($name:ident => $bytes:ident [ $index:literal ] == $present:literal) => {
        match $bytes[$index] {
            $present => true,
            b'-' => false,
            _ => {
                ::tracing::error!(
                    "byte value {:02X?} not valid for option {}",
                    $bytes[$index],
                    stringify!($name)
                );
                return Err($crate::error::RigError::InvalidResponseData {
                    data: $bytes.to_vec(),
                });
            }
        }
    };
}
