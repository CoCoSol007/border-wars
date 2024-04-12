use std::marker::PhantomData;

use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use uuid::Uuid;

pub trait UuidKey: From<Uuid> + Copy {
    fn to_uuid(&self) -> Uuid;
}

#[macro_export]
macro_rules! new_id_type {
    ( $($vis:vis struct $name:ident;)* ) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)]
            $vis struct $name(::uuid::Uuid);

            impl ::core::convert::From<::uuid::Uuid> for $name {
                fn from(k: ::uuid::Uuid) -> Self {
                    $name(k)
                }
            }

            impl $crate::utils::UuidKey for $name {
                fn to_uuid(&self) -> ::uuid::Uuid {
                    self.0
                }
            }
        )*
    };
}

pub struct IdDashMap<K: UuidKey, V>(DashMap<Uuid, V>, PhantomData<K>);

impl<K: UuidKey, V> IdDashMap<K, V> {
    pub fn new() -> Self {
        Self(DashMap::new(), PhantomData)
    }

    pub fn insert(&self, value: V) -> K {
        loop {
            let id = Uuid::new_v4();
            let Entry::Vacant(entry) = self.0.entry(id) else {
                continue;
            };
            entry.insert(value);
            return K::from(id);
        }
    }

    pub fn insert_with_id(&self, create_value: impl FnOnce(K) -> V) -> K {
        loop {
            let id = Uuid::new_v4();
            let Entry::Vacant(entry) = self.0.entry(id) else {
                continue;
            };
            let id = K::from(id);
            let value = create_value(id);
            entry.insert(value);
            return id;
        }
    }

    pub fn remove(&self, id: K) {
        self.0.remove(&id.to_uuid());
    }
}

impl<K: UuidKey, V> Default for IdDashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
