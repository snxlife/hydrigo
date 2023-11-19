#[macro_export]
macro_rules! impl_attribute {
    ($an: ident { $($gf: ident ( $init: expr ) : $sf: ident ( $t: ty )), * $(,)? }) => {
        #[allow(unused)]
        pub struct $an {
            $(
                pub $gf: $t,
            ) *
        }

        impl $an {
            $(
                #[allow(unused)]
                pub fn $sf(&mut self, value: $t) {
                    self.$gf = value;
                }
            ) *
        }

        impl std::default::Default for $an {
            fn default() -> Self {
                Self {
                    $(
                        $gf: $init,
                    ) *
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_dynclone {
    (trait $trait: ident : $($t: tt) *) => {
        pub trait $trait: $($t) * + dyn_clone::DynClone {}
        impl<T> $trait for T
        where
            T: $($t) * + dyn_clone::DynClone
        {}
        dyn_clone::clone_trait_object!($trait);
    };
}
