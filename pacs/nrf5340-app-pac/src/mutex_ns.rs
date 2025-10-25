#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    mutex: [Mutex; 16],
}
impl RegisterBlock {
    #[doc = "0x400..0x440 - Description collection: Mutex register"]
    #[inline(always)]
    pub const fn mutex(&self, n: usize) -> &Mutex {
        &self.mutex[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x440 - Description collection: Mutex register"]
    #[inline(always)]
    pub fn mutex_iter(&self) -> impl Iterator<Item = &Mutex> {
        self.mutex.iter()
    }
}
#[doc = "MUTEX (rw) register accessor: Description collection: Mutex register\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex`] module"]
#[doc(alias = "MUTEX")]
pub type Mutex = crate::Reg<mutex::MutexSpec>;
#[doc = "Description collection: Mutex register"]
pub mod mutex;
