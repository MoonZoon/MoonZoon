use crate::l_var_map::LVarMap;
use crate::s_var_map::SVarMap;
use crate::c_var_map::CVarMap;
use crate::cache_map::CacheMap;
use crate::block_call_stack::__BlockCallStack;
use crate::tracked_call_stack::__TrackedCallStack;
use crate::relations::__Relations;
use crate::element::Element;
use std::cell::RefCell;
// use std::any::Any;

thread_local! {
    pub(crate) static CACHES: RefCell<CacheMap> = RefCell::new(CacheMap::new());
    pub(crate) static SVARS: RefCell<SVarMap> = RefCell::new(SVarMap::new());
    pub(crate) static LVARS: RefCell<LVarMap> = RefCell::new(LVarMap::new());
    pub(crate) static CVARS: RefCell<CVarMap> = RefCell::new(CVarMap::new());
    pub(crate) static ROOT_CMP: RefCell<Option<Box<dyn Fn() -> Box<dyn Element>>>> = RefCell::new(None);
    pub(crate) static BLOCK_CALL_STACK: RefCell<__BlockCallStack> = RefCell::new(__BlockCallStack::default());
    pub(crate) static TRACKED_CALL_STACK: RefCell<__TrackedCallStack> = RefCell::new(__TrackedCallStack::default());
    pub(crate) static RELATIONS: RefCell<__Relations> = RefCell::new(__Relations::default());
}
