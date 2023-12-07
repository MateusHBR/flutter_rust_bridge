use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash)]
pub(crate) enum CodecMode {
    Cst,
    Dco,
    Sse,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Hash)]
pub(crate) struct CodecModePack {
    pub dart2rust: CodecMode,
    pub rust2dart: CodecMode,
}

#[doc(hidden)]
#[macro_export]
macro_rules! codegen_codec_structs {
    ($enum_name:ident) => (
        crate::codegen_codec_structs!(
            @private

            $enum_name;

            Cst,
            Dco,
            Sse,
        );
    );
    (@private $enum_name:ident ; $($name:ident),*,) => (
        paste::paste! {
            #[enum_dispatch([<$enum_name Trait>])]
            pub(crate) enum $enum_name {
                $(
                $name([<$name $enum_name>]),
                )*
            }

            impl $enum_name {
                pub(crate) fn new(mode: CodecMode) -> Self {
                    match mode {
                        $(
                        CodecMode::$name => Self::$name([<$name $enum_name>] {}),
                        )*
                    }
                }
            }
        }
    )
}
