use crate::el_var_map::ElVarMap;
use crate::cmp_var_map::CmpVarMap;
use crate::s_var_map::SVarMap;
use crate::var_map::VarMap;
use crate::c_var_map::CVarMap;
use crate::cache_map::CacheMap;
use crate::block_call_stack::__BlockCallStack;
use crate::component_call_stack::__ComponentCallStack;
use crate::tracked_call_stack::__TrackedCallStack;
use crate::relations::__Relations;
use crate::element::Element;
use std::cell::RefCell;
// use std::any::Any;

thread_local! {
    pub(crate) static CACHES: RefCell<CacheMap> = RefCell::new(CacheMap::new());
    pub(crate) static S_VARS: RefCell<SVarMap> = RefCell::new(SVarMap::new());
    pub(crate) static VARS: RefCell<VarMap> = RefCell::new(VarMap::new());
    pub(crate) static EL_VARS: RefCell<ElVarMap> = RefCell::new(ElVarMap::new());
    pub(crate) static CMP_VARS: RefCell<CmpVarMap> = RefCell::new(CmpVarMap::new());
    pub static C_VARS: RefCell<CVarMap> = RefCell::new(CVarMap::new());
    pub(crate) static ROOT_CMP: RefCell<Option<Box<dyn Fn() -> Box<dyn Element>>>> = RefCell::new(None);
    pub(crate) static BLOCK_CALL_STACK: RefCell<__BlockCallStack> = RefCell::new(__BlockCallStack::default());
    pub(crate) static COMPONENT_CALL_STACK: RefCell<__ComponentCallStack> = RefCell::new(__ComponentCallStack::default());
    pub(crate) static TRACKED_CALL_STACK: RefCell<__TrackedCallStack> = RefCell::new(__TrackedCallStack::default());
    pub(crate) static RELATIONS: RefCell<__Relations> = RefCell::new(__Relations::default());
}
