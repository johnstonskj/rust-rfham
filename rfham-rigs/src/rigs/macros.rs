macro_rules! brand_mod {
    ($name:literal) => {
        pub fn brand_name() -> ::rfham_core::Name {
            <::rfham_core::Name as rfham_core::StringLike>::new_unchecked($name)
        }
    };
}

#[allow(unused_macro_rules)]
macro_rules! rig_mod {
    ($mod_name:ident as $display_name:literal) => {
        urn_mod!(rig $mod_name as $display_name);
    };
    ($name:literal) => {
        urn_mod!(rig $name);
    };
}

#[allow(unused_macro_rules)]
macro_rules! amp_mod {
    ($mod_name:ident as $display_name:literal) => {
        urn_mod!(amplifier $mod_name as $display_name);
    };
    ($name:literal) => {
        urn_mod!(amplifier $name);
    };
}

macro_rules! urn_mod {
    ($kind:ident $mod_name:ident as $display_name:literal) => {
        pub mod $mod_name {
            urn_mod!($kind $display_name);
        }
    };
    ($kind:ident $name:literal) => {
        pub fn model_name() -> ::rfham_core::Name {
            <::rfham_core::Name as rfham_core::StringLike>::new_unchecked($name)
        }
        pub fn model_urn() -> ::rfham_iri::UniversalRigName {
            ::rfham_iri::UniversalRigName:: $kind(super::brand_name(), model_name())
        }
    };
}
