//! A collection of macros intended to make defining newtypes for identifiers nice and easy.

/// The `gw2rs_id_generic!` macro provides standard impls for identifier newtypes.
/// This macro is really only useful when used as part of one of the data-specific
/// id type macros, but its probably fine to use on its own for any "newtype" struct
macro_rules! gw2rs_id_generic {
    ($name: ident) => {
        impl AsRef<$name> for $name {
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

/// The `gw2rs_id_u64` macro is provided to make defining u64-based newtypes for API identifiers
/// easy.
macro_rules! gw2rs_id_u64 {
    ($name: ident) => {
        #[allow(missing_docs)]
        #[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
        pub struct $name(pub u64);

        gw2rs_id_generic!($name);

        impl From<u64> for $name {
            fn from(id_as_u64: u64) -> $name {
                $name(id_as_u64)
            }
        }

        impl From<$name> for u64 {
            fn from(id_as_newtype: $name) -> u64 {
                id_as_newtype.0
            }
        }
    };
}

/// The `gw2rs_id_string` macro is provided to streamline the process of defining String-based
/// newtypes for various identifier values returned from the ArenaNet API.
macro_rules! gw2rs_id_string {
    ($name: ident) => {
        #[allow(missing_docs)]
        #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
        pub struct $name(pub String);

        gw2rs_id_generic!($name);

        impl<T: AsRef<str>> From<T> for $name {
            fn from(v: T) -> $name {
                $name(String::from(v.as_ref()))
            }
        }
    };
}
