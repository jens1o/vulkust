use super::debug::Debug;
use super::types::Id;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

pub static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub fn create_id() -> Id {
    return NEXT_ID.fetch_add(1, Ordering::Relaxed);
}

pub trait Object: Debug {
    fn get_id(&self) -> Id;
    fn get_name(&self) -> Option<&str>;
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    id: Id,
    name: Option<String>,
}

impl Base {
    pub fn builder() -> Builder {
        Builder {
            id: None,
            name: None,
        }
    }
}

pub struct Builder {
    id: Option<Id>,
    name: Option<String>,
}

impl Builder {
    pub fn id(self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Base {
        let id = if let Some(id) = self.id {
            id
        } else {
            create_id()
        };
        Base {
            id,
            name: self.name,
        }
    }
}

impl Object for Base {
    fn get_id(&self) -> Id {
        self.id
    }

    fn get_name(&self) -> Option<&str> {
        self.name.as_ref()
    }
}

impl<T> Object for Arc<T>
where
    T: Object,
{
    fn get_id(&self) -> Id {
        self.get_id()
    }

    fn get_name(&self) -> Option<&str> {
        self.get_name()
    }
}

impl<T> Object for Mutex<T>
where
    T: Object,
{
    fn get_id(&self) -> Id {
        vxresult!(self.lock()).get_id()
    }

    fn get_name(&self) -> Option<&str> {
        vxresult!(self.lock()).get_name()
    }
}

impl<T> Object for RwLock<T>
where
    T: Object,
{
    fn get_id(&self) -> Id {
        vxresult!(self.read()).get_id()
    }

    fn get_name(&self) -> Option<&str> {
        vxresult!(self.read()).get_name()
    }
}

macro_rules! create_has_base {
    () => {
        pub trait HasBase {
            fn get_base(&self) -> &Base;
            fn get_mut_base(&mut self) -> &mut Base;
        }

        impl<T> HasBase for Arc<Mutex<T>>
        where
            T: HasBase,
        {
            fn get_base(&self) -> &Base {
                vxresult!(self.lock()).get_base()
            }

            fn get_mut_base(&mut self) -> &mut Base {
                vxresult!(self.lock()).get_mut_base()
            }
        }

        impl<T> HasBase for Mutex<T>
        where
            T: HasBase,
        {
            fn get_base(&self) -> &Base {
                vxresult!(self.lock()).get_base()
            }

            fn get_mut_base(&mut self) -> &mut Base {
                vxresult!(self.lock()).get_mut_base()
            }
        }

        impl<T> HasBase for Arc<RwLock<T>>
        where
            T: HasBase,
        {
            fn get_base(&self) -> &Base {
                vxresult!(self.read()).get_base()
            }

            fn get_mut_base(&mut self) -> &mut Base {
                vxresult!(self.write()).get_mut_base()
            }
        }

        impl<T> HasBase for RwLock<T>
        where
            T: HasBase,
        {
            fn get_base(&self) -> &Base {
                vxresult!(self.read()).get_base()
            }

            fn get_mut_base(&mut self) -> &mut Base {
                vxresult!(self.write()).get_mut_base()
            }
        }
    };
}

create_has_base!();
