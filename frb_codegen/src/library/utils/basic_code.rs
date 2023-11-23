use serde::Serialize;
use std::ops::AddAssign;

pub(crate) trait BasicCode {
    fn body(&self) -> &str;

    fn new_body(body: String) -> Self;
}

#[doc(hidden)]
#[macro_export]
macro_rules! basic_code_impl {
    ($name:ident) => {
        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                value.to_owned().into()
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self += rhs;
                self
            }
        }

        impl std::iter::FromIterator<$name> for $name {
            fn from_iter<A: IntoIterator<Item = $name>>(iter: A) -> Self {
                iter.into_iter().fold(Default::default(), |a, b| a + b)
            }
        }

        impl crate::utils::basic_code::BasicCode for $name {
            fn body(&self) -> &str {
                &self.body
            }

            fn new_body(body: String) -> Self {
                Self {
                    body,
                    ..Default::default()
                }
            }
        }
    };
}

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct DartBasicCode {
    pub file_top: String,
    pub import: String,
    pub part: String,
    pub body: String,
}

basic_code_impl!(DartBasicCode);

impl From<String> for DartBasicCode {
    fn from(body: String) -> Self {
        Self {
            body,
            ..Default::default()
        }
    }
}

impl AddAssign for DartBasicCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.file_top += &rhs.file_top;
        self.import += &rhs.import;
        self.part += &rhs.part;
        self.body += &rhs.body;
    }
}
