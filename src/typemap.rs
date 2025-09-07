use std::{
    any::{Any, TypeId},
    cell::UnsafeCell,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use parking_lot::Mutex;
use rustc_hash::FxHashMap;

type InnerMap = FxHashMap<TypeId, Arc<dyn Any + Send + Sync>>;

pub struct TypeMap {
    frozen: AtomicBool,
    mutex: Mutex<()>,
    data: UnsafeCell<InnerMap>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            frozen: AtomicBool::new(false),
            mutex: Mutex::new(()),
            data: UnsafeCell::new(FxHashMap::default()),
        }
    }

    pub fn with_map_mut<R>(&self, f: impl FnOnce(&mut InnerMap) -> R) -> R {
        if self.frozen.load(Ordering::Acquire) {
            panic!("cannot perform mutable operation on a frozen typemap");
        } else {
            let _lock = self.mutex.lock();
            f(unsafe { &mut *self.data.get() })
        }
    }

    pub fn with_map<R>(&self, f: impl FnOnce(&InnerMap) -> R) -> R {
        if self.frozen.load(Ordering::Acquire) {
            f(unsafe { &mut *self.data.get() })
        } else {
            let _lock = self.mutex.lock();
            f(unsafe { &mut *self.data.get() })
        }
    }

    pub fn freeze(&mut self) {
        self.frozen.store(true, Ordering::Release);
    }

    pub fn len(&self) -> usize {
        self.with_map(|map| map.len())
    }

    pub fn is_empty(&self) -> bool {
        self.with_map(|map| map.is_empty())
    }

    pub fn clear(&self) {
        self.with_map_mut(|map| map.clear());
    }

    pub fn contains<T: Any>(&self) {
        self.with_map(|map| map.contains_key(&TypeId::of::<T>()));
    }

    pub fn insert<T: Any + Send + Sync>(&self, val: T) {
        self.with_map_mut(|map| map.insert(TypeId::of::<T>(), Arc::new(val)));
    }

    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.with_map(|map| {
            map.get(&TypeId::of::<T>())
                .map(|b| unsafe { &*((&**b) as *const _ as *const T) })
        })
    }

    pub fn get_owned<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        self.with_map(|map| {
            map.get(&TypeId::of::<T>())
                .map(|b| unsafe { b.clone().downcast_unchecked() })
        })
    }

    pub fn remove<T: Any + Send + Sync>(&self) {
        self.with_map_mut(|map| map.remove(&TypeId::of::<T>()));
    }
}

impl Default for TypeMap {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for TypeMap {}
unsafe impl Sync for TypeMap {}
