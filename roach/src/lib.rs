use core::ffi::c_void;
use std::thread::sleep;
use std::time::Duration;
use transcend::ptr::resolve_fn;
use windows::core::w;
use windows::core::PCWSTR;

struct CNamesPoolTable {
    get: unsafe extern "C" fn() -> *mut CNamesPool,
    add_entry: unsafe extern "thiscall" fn(*mut CNamesPool, PCWSTR),
}

struct CRTTISystemTable {
    get: unsafe extern "C" fn() -> *mut CRTTISystem,
    register_global_function: unsafe extern "thiscall" fn(*mut CRTTISystem, *mut CFunction),
}

struct CFunction {}
struct CNamesPool {}
struct CRTTISystem {}

unsafe extern "C" fn my_callback(p1: u64, p2: u64, p3: u64) {
    // Your callback implementation here
    println!("Callback called with p1: {}, p2: {}, p3: {}", p1, p2, p3);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn plugin() {
    sleep(Duration::from_secs(120));

    let Allocator: unsafe extern "C" fn(size: usize, alignment: usize) -> *mut CFunction =
        resolve_fn(0x2846B0);

    let NamesPoolGet: unsafe extern "C" fn() -> *mut CNamesPool = resolve_fn(0x2843A0);

    let NamesPoolAddEntry: unsafe extern "thiscall" fn(*mut CNamesPool, PCWSTR) -> u32 =
        resolve_fn(0x145A3A0);

    let CFunctionConstructor: unsafe extern "thiscall" fn(
        *mut CFunction,
        u32,
        *mut c_void,
    ) -> *mut CFunction = resolve_fn(0x141496FA0);

    let CRTTISystemGet: unsafe extern "C" fn() -> *mut CRTTISystem = resolve_fn(0x285D60);

    let CRTTISystemRegisterGlobalFunction: unsafe extern "thiscall" fn(
        *mut CRTTISystem,
        *mut CFunction,
    ) = resolve_fn(0x146A5f0);

    let names_pool = NamesPoolGet();
    let name_hash = NamesPoolAddEntry(names_pool, w!("TestFunction"));

    let memory = Allocator(0xC0, 0x10);
    let function = CFunctionConstructor(memory, name_hash, my_callback as *mut c_void);

    let rtti_system = CRTTISystemGet();
    CRTTISystemRegisterGlobalFunction(rtti_system, function);
}