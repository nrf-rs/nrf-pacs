#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    profiling: (),
    _reserved1: [u8; 0x0100],
    enable: Enable,
    invalidate: Invalidate,
    erase: Erase,
    profilingenable: Profilingenable,
    profilingclear: Profilingclear,
    mode: Mode,
    debuglock: Debuglock,
    erasestatus: Erasestatus,
    writelock: Writelock,
}
impl RegisterBlock {
    #[doc = "0x400..0x420 - Unspecified"]
    #[inline(always)]
    pub const fn profiling(&self, n: usize) -> &Profiling {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1024)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x420 - Unspecified"]
    #[inline(always)]
    pub fn profiling_iter(&self) -> impl Iterator<Item = &Profiling> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1024)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x500 - Enable cache."]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504 - Invalidate the cache."]
    #[inline(always)]
    pub const fn invalidate(&self) -> &Invalidate {
        &self.invalidate
    }
    #[doc = "0x508 - Erase the cache."]
    #[inline(always)]
    pub const fn erase(&self) -> &Erase {
        &self.erase
    }
    #[doc = "0x50c - Enable the profiling counters."]
    #[inline(always)]
    pub const fn profilingenable(&self) -> &Profilingenable {
        &self.profilingenable
    }
    #[doc = "0x510 - Clear the profiling counters."]
    #[inline(always)]
    pub const fn profilingclear(&self) -> &Profilingclear {
        &self.profilingclear
    }
    #[doc = "0x514 - Cache mode. Switching from Cache to Ram mode causes the RAM to be cleared. Switching from RAM to Cache mode causes the cache to be invalidated."]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x518 - Lock debug mode."]
    #[inline(always)]
    pub const fn debuglock(&self) -> &Debuglock {
        &self.debuglock
    }
    #[doc = "0x51c - Cache erase status."]
    #[inline(always)]
    pub const fn erasestatus(&self) -> &Erasestatus {
        &self.erasestatus
    }
    #[doc = "0x520 - Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode."]
    #[inline(always)]
    pub const fn writelock(&self) -> &Writelock {
        &self.writelock
    }
}
#[doc = "Unspecified"]
pub use self::profiling::Profiling;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod profiling;
#[doc = "ENABLE (rw) register accessor: Enable cache.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable cache."]
pub mod enable;
#[doc = "INVALIDATE (w) register accessor: Invalidate the cache.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`invalidate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@invalidate`] module"]
#[doc(alias = "INVALIDATE")]
pub type Invalidate = crate::Reg<invalidate::InvalidateSpec>;
#[doc = "Invalidate the cache."]
pub mod invalidate;
#[doc = "ERASE (w) register accessor: Erase the cache.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erase::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erase`] module"]
#[doc(alias = "ERASE")]
pub type Erase = crate::Reg<erase::EraseSpec>;
#[doc = "Erase the cache."]
pub mod erase;
#[doc = "PROFILINGENABLE (rw) register accessor: Enable the profiling counters.\n\nYou can [`read`](crate::Reg::read) this register and get [`profilingenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`profilingenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@profilingenable`] module"]
#[doc(alias = "PROFILINGENABLE")]
pub type Profilingenable = crate::Reg<profilingenable::ProfilingenableSpec>;
#[doc = "Enable the profiling counters."]
pub mod profilingenable;
#[doc = "PROFILINGCLEAR (w) register accessor: Clear the profiling counters.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`profilingclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@profilingclear`] module"]
#[doc(alias = "PROFILINGCLEAR")]
pub type Profilingclear = crate::Reg<profilingclear::ProfilingclearSpec>;
#[doc = "Clear the profiling counters."]
pub mod profilingclear;
#[doc = "MODE (rw) register accessor: Cache mode. Switching from Cache to Ram mode causes the RAM to be cleared. Switching from RAM to Cache mode causes the cache to be invalidated.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Cache mode. Switching from Cache to Ram mode causes the RAM to be cleared. Switching from RAM to Cache mode causes the cache to be invalidated."]
pub mod mode;
#[doc = "DEBUGLOCK (rw) register accessor: Lock debug mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`debuglock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debuglock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debuglock`] module"]
#[doc(alias = "DEBUGLOCK")]
pub type Debuglock = crate::Reg<debuglock::DebuglockSpec>;
#[doc = "Lock debug mode."]
pub mod debuglock;
#[doc = "ERASESTATUS (rw) register accessor: Cache erase status.\n\nYou can [`read`](crate::Reg::read) this register and get [`erasestatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasestatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erasestatus`] module"]
#[doc(alias = "ERASESTATUS")]
pub type Erasestatus = crate::Reg<erasestatus::ErasestatusSpec>;
#[doc = "Cache erase status."]
pub mod erasestatus;
#[doc = "WRITELOCK (rw) register accessor: Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`writelock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writelock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writelock`] module"]
#[doc(alias = "WRITELOCK")]
pub type Writelock = crate::Reg<writelock::WritelockSpec>;
#[doc = "Lock cache updates. Prevents updating of cache content on cache misses, but will continue to lookup instruction/data fetches in content already present in the cache. Ignored in RAM mode."]
pub mod writelock;
