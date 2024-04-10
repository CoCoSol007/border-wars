use std::marker::PhantomData;

use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use uuid::Uuid;

pub trait UuidKey: From<Uuid> {
    fn to_uuid(&self) -> Uuid;
}

pub struct SlotDashMap<K: UuidKey, V>(DashMap<Uuid, V>, PhantomData<K>);

impl<K: UuidKey, V> SlotDashMap<K, V> {
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
}

impl<K: UuidKey, V> Default for SlotDashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! new_key_type {
    ( $(#[$outer:meta])* $vis:vis struct $name:ident; $($rest:tt)* ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Default,
                 Eq, PartialEq, Ord, PartialOrd,
                 Hash, Debug)]
        #[repr(transparent)]
        $vis struct $name(::uuid::Uuid);

        impl ::core::convert::From<::uuid::Uuid> for $name {
            fn from(k: ::uuid::Uuid) -> Self {
                $name(k)
            }
        }

        impl $crate::UuidKey for $name {
            fn to_uuid(&self) -> ::uuid::Uuid {
                self.0
            }
        }

        $crate::new_key_type!($($rest)*);
    };

    () => {}
}
