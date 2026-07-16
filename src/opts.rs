use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{collections::HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptEntry {
    name: String,
    valid_options: Vec<Value>,
    default_value: Value,
    current: Value,
}

impl OptEntry {
    pub fn new<V: Into<Value>>(
        name: impl Into<String>,
        valid_options: Vec<V>,
        default_value: V,
        current: Option<V>,
    ) -> Self {
        let d = Into::<Value>::into(default_value);
        Self {
            name: name.into(),
            valid_options: valid_options.into_iter().map(Into::into).collect(),
            current: match current {
                Some(current) => current.into(),
                None => d.clone(),
            },

            default_value: d,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_valid_options_as<T: DeserializeOwned>(&self) -> Result<Vec<T>, serde_json::Error> {
        self.valid_options
            .iter()
            .map(|o| serde_json::from_value::<T>(o.clone()))
            .collect()
    }

    pub fn get_default_as<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value::<T>(self.default_value.clone())
    }

    pub fn get_current_as<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value::<T>(self.current.clone())
    }

    pub fn set_current<T: Serialize>(&mut self, opt: T) -> Result<(), serde_json::Error> {
        let v = serde_json::to_value(opt)?;
        self.current = v;

        Ok(())
    }
}

pub type Opts = HashMap<String, OptEntry>;

pub trait CreateDefaultOpts {
    fn create_default_options() -> Self;
}

pub trait GetDefaultOpt {
    fn get_default_opt(&self, opt: &DefaultOpts) -> &OptEntry;
    fn get_default_opt_mut(&mut self, opt: &DefaultOpts) -> &mut OptEntry;
}

macro_rules! create_default_opts {
    ($($ident:ident : [$($value_opt:literal),+ $(,)?] => $default:literal),+ $(,)?) => {
        #[derive(Debug)]
        pub enum DefaultOpts {
            $($ident)*
        }

        impl CreateDefaultOpts for Opts {
            fn create_default_options() -> Self {
                let mut opts = HashMap::new();

                $(
                    opts.insert(
                        stringify!($ident).to_string(),
                        OptEntry::new(stringify!($ident), vec![$($value_opt),*], $default, None)
                    );
                )*

                opts
            }
        }

    };
}


impl std::fmt::Display for DefaultOpts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl GetDefaultOpt for Opts {
    fn get_default_opt(&self, opt: &DefaultOpts) -> &OptEntry {
        self.get(&opt.to_string()).expect("Failed to get a default key!")
    }

    fn get_default_opt_mut(&mut self, opt: &DefaultOpts) -> &mut OptEntry {
        self.get_mut(&opt.to_string()).expect("Failed to get a default key!")
    }
}

create_default_opts! {
    ShouldPrintEveryLine : [true, false] => true,
}
