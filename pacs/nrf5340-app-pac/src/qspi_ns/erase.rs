#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "ERASE")]
pub struct Erase {
    ptr: Ptr,
    len: Len,
}
impl Erase {
    #[doc = "0x00 - Start address of flash block to be erased"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x04 - Size of block to be erased."]
    #[inline(always)]
    pub const fn len(&self) -> &Len {
        &self.len
    }
}
#[doc = "PTR (rw) register accessor: Start address of flash block to be erased\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`] module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Start address of flash block to be erased"]
pub mod ptr;
#[doc = "LEN (rw) register accessor: Size of block to be erased.\n\nYou can [`read`](crate::Reg::read) this register and get [`len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@len`] module"]
#[doc(alias = "LEN")]
pub type Len = crate::Reg<len::LenSpec>;
#[doc = "Size of block to be erased."]
pub mod len;
