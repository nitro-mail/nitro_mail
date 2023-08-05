use crate::configs::IOOrToml;
use impl_tools::autoimpl;
use serde::de::DeserializeOwned;
use std::path::PathBuf;

#[derive(Debug)]
pub enum PathOrType<T> {
    Path(PathBuf),
    Record(T),
}
impl<C: Clone> Clone for PathOrType<C> {
    fn clone(&self) -> Self {
        match self {
            PathOrType::Path(p) => PathOrType::Path(p.clone()),
            PathOrType::Record(r) => PathOrType::Record(r.clone()),
        }
    }
}
impl<T: DeserializeOwned> PathOrType<T> {
    pub fn get_value_toml(self) -> Result<T, IOOrToml> {
        match self {
            PathOrType::Path(p) => {
                let config = std::fs::read_to_string(p)?;
                Ok(toml::from_str(&config)?)
            }
            PathOrType::Record(r) => Ok(r),
        }
    }
}

impl<P: PartialEq> PartialEq for PathOrType<P> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PathOrType::Path(p1), PathOrType::Path(p2)) => p1 == p2,
            (PathOrType::Record(r1), PathOrType::Record(r2)) => r1 == r2,
            _ => false,
        }
    }
}
impl<E: Eq> Eq for PathOrType<E> {}

mod _path_or_type_serde {
    use crate::configs::PathOrType;
    use serde::de::{MapAccess, SeqAccess, Visitor};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::path::PathBuf;

    impl<T: Serialize> Serialize for PathOrType<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                PathOrType::Path(p) => p.serialize(serializer),
                PathOrType::Record(r) => r.serialize(serializer),
            }
        }
    }

    struct PathOrTypeDeserializer<T>(std::marker::PhantomData<T>);
    impl<'de, D: Deserialize<'de>> Visitor<'de> for PathOrTypeDeserializer<D> {
        type Value = PathOrType<D>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a path or a type")
        }

        // TODO support deserializing a string that is not a path
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Self::visit_string(self, v.to_string())
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v.starts_with("value:") {}
            Ok(PathOrType::Path(PathBuf::from(v)))
        }
        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            Ok(PathOrType::Record(D::deserialize(
                serde::de::value::MapAccessDeserializer::new(map),
            )?))
        }
    }

    impl<'de, D: Deserialize<'de>> Deserialize<'de> for PathOrType<D> {
        fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
        where
            De: Deserializer<'de>,
        {
            deserializer.deserialize_any(PathOrTypeDeserializer(std::marker::PhantomData::<D>))
        }
    }
}
