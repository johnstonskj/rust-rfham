#![allow(rustdoc::private_doc_tests)]
// ------------------------------------------------------------------------------------------------
// Macros for CI-V Protocol Implementations
// ------------------------------------------------------------------------------------------------

///
/// ```rust
/// define_command!(
///     "doc string" => CommandType {
///         "field doc string" field_name: field_type,
///     }
/// );
/// define_command!(
///     "doc string" => CommandType { state }
/// );
/// define_command!(
///     "doc string" => CommandType
/// );
/// ```
///
/// * `"doc string"` is a documentation string that will be applied to the command structure.
/// * `CommandType` is the name of the command structure that will implement the `Command` trait.
/// * `"field doc string"` is an optional documentation string that will be applied to the command
///   structure field.
/// * `field_name` is the name of the command structure field.
/// * `field_type` is the type of the command structure field.
/// * `{ state }` is a shorthand for commands that have a single boolean field named `on`, which
///   represents the On/Off state.
///
#[allow(unused)]
macro_rules! define_command {
    (
        $doc_str:literal => $cmd_type:ident $( {
            $( $( $field_doc:literal )? $field:ident : $type:ty ),+
        } )?
    ) => {
        #[doc = $doc_str]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $cmd_type {
            #[doc = "Address message is being sent to."]
            pub to_address: $crate::protocol::civ::BusAddress,
            $( $(
                $(
                    #[doc = $field_doc]
                )?
                pub $field: $type
            ),+ )?
        }
    };
    (
        $doc_str:literal => $cmd_type:ident { state }
    ) => {
        define_command!($doc_str => $cmd_type {
            "Represents the On/Off state." on: bool
        });
    };
}

// ------------------------------------------------------------------------------------------------

///
/// ```text
/// impl_command!(
///     impl_type '=>'
///     id_byte ( ':' subid_byte )?
///     ( 'with' arg_bytes_fn )?
/// )
/// ```
///
/// * `id_byte: u8`
/// * `subid_byte: u8`
/// * `arg_bytes_fn: fn(&self) -> Result<Option<Vec<u8>>, RigError>`
///
#[allow(unused)]
macro_rules! impl_command {
    ($type:ident => $id:literal $( : $sub:literal )? with Some $arg_fn:expr) => {
        impl_command!($type => $id $( : $sub )? with |arg|{
            Ok(Some($arg_fn(arg)))
        });
    };
    ($type:ident => $id:literal $( : $sub:literal )? with $arg_fn:expr) => {
        impl $crate::protocol::Command for $type {
            impl_command!($type);
            impl_command!(@id $id $( : $sub )?);

            fn argument_bytes(&self) -> Result<Option<Vec<u8>>, $crate::error::RigError> {
                $arg_fn(self)
            }
        }

        impl_command!(@constructors $type);
    };
    ($type:ident => $id:literal $( : $sub:literal )?) => {
        impl $crate::protocol::Command for $type {
            impl_command!($type);
            impl_command!(@id $id $( : $sub )?);
        }

        impl_command!(@constructors $type);
    };
    ($type:ident) => {
        const MESSAGE_TERMINATOR: u8 = 0xFD;

        fn message_preamble(&self) -> Option<&[u8]> {
            None
        }

        fn to_message(&self) -> Result<Vec<u8>, $crate::error::RigError> {
            $crate::protocol::civ::make_message(
                self, self.to_address.into(), Self::MESSAGE_TERMINATOR
            )
        }
    };
    (@id $id:literal $( : $sub:literal )? ) => {
            fn command_id(&self) -> &[u8] {
                &[$id $(, $sub)? ]
            }
   };
   (@constructors $type:ident) => {
        impl $crate::protocol::civ::CivCommand for $type {
            fn send_to(address: $crate::protocol::civ::BusAddress) -> Self {
                Self {
                    to_address: address,
                    ..Self::default()
                }
            }

            fn broadcast() -> Self {
                Self::send_to($crate::protocol::civ::BusAddress::broadcast())
            }

            fn send_to_address(&self) -> $crate::protocol::civ::BusAddress {
                self.to_address
            }
        }
   };
}
