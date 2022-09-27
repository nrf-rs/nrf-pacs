#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400..0x410 - Unspecified"]
    pub profiling0: PROFILING,
    _reserved1: [u8; 0x10],
    #[doc = "0x420..0x430 - Unspecified"]
    pub profiling1: PROFILING,
    _reserved2: [u8; 0xd0],
    #[doc = "0x500 - Enable cache."]
    pub enable: ENABLE,
    #[doc = "0x504 - Invalidate the cache."]
    pub invalidate: INVALIDATE,
    #[doc = "0x508 - Erase the cache."]
    pub erase: ERASE,
    #[doc = "0x50c - Enable the profiling counters."]
    pub profilingenable: PROFILINGENABLE,
    #[doc = "0x510 - Clear the profiling counters."]
    pub profilingclear: PROFILINGCLEAR,
    #[doc = "0x514 - Cache mode. Switching from Cache to Ram mode causes the RAM to be cleared. Switching from RAM to Cache mode causes the cache to be invalidated."]
    pub mode: MODE,
    #[doc = "0x518 - Lock debug mode."]
    pub debuglock: DEBUGLOCK,
    #[doc = "0x51c - Cache erase status."]
    pub erasestatus: ERASESTATUS,
    #[doc = "0x520 - Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode."]
    pub writelock: WRITELOCK,
}
#[doc = "Unspecified"]
pub use profiling::PROFILING;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod profiling;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable cache."]
pub mod enable;
#[doc = "INVALIDATE (w) register accessor: an alias for `Reg<INVALIDATE_SPEC>`"]
pub type INVALIDATE = crate::Reg<invalidate::INVALIDATE_SPEC>;
#[doc = "Invalidate the cache."]
pub mod invalidate;
#[doc = "ERASE (w) register accessor: an alias for `Reg<ERASE_SPEC>`"]
pub type ERASE = crate::Reg<erase::ERASE_SPEC>;
#[doc = "Erase the cache."]
pub mod erase;
#[doc = "PROFILINGENABLE (rw) register accessor: an alias for `Reg<PROFILINGENABLE_SPEC>`"]
pub type PROFILINGENABLE = crate::Reg<profilingenable::PROFILINGENABLE_SPEC>;
#[doc = "Enable the profiling counters."]
pub mod profilingenable;
#[doc = "PROFILINGCLEAR (w) register accessor: an alias for `Reg<PROFILINGCLEAR_SPEC>`"]
pub type PROFILINGCLEAR = crate::Reg<profilingclear::PROFILINGCLEAR_SPEC>;
#[doc = "Clear the profiling counters."]
pub mod profilingclear;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Cache mode. Switching from Cache to Ram mode causes the RAM to be cleared. Switching from RAM to Cache mode causes the cache to be invalidated."]
pub mod mode;
#[doc = "DEBUGLOCK (rw) register accessor: an alias for `Reg<DEBUGLOCK_SPEC>`"]
pub type DEBUGLOCK = crate::Reg<debuglock::DEBUGLOCK_SPEC>;
#[doc = "Lock debug mode."]
pub mod debuglock;
#[doc = "ERASESTATUS (rw) register accessor: an alias for `Reg<ERASESTATUS_SPEC>`"]
pub type ERASESTATUS = crate::Reg<erasestatus::ERASESTATUS_SPEC>;
#[doc = "Cache erase status."]
pub mod erasestatus;
#[doc = "WRITELOCK (rw) register accessor: an alias for `Reg<WRITELOCK_SPEC>`"]
pub type WRITELOCK = crate::Reg<writelock::WRITELOCK_SPEC>;
#[doc = "Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode."]
pub mod writelock;
