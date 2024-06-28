#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "SEQ")]
pub struct Seq {
    ptr: Ptr,
    cnt: Cnt,
    refresh: Refresh,
    enddelay: Enddelay,
}
impl Seq {
    #[doc = "0x00 - Description cluster: Beginning address in RAM of this sequence"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x04 - Description cluster: Number of values (duty cycles) in this sequence"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x08 - Description cluster: Number of additional PWM periods between samples loaded into compare register"]
    #[inline(always)]
    pub const fn refresh(&self) -> &Refresh {
        &self.refresh
    }
    #[doc = "0x0c - Description cluster: Time added after the sequence"]
    #[inline(always)]
    pub const fn enddelay(&self) -> &Enddelay {
        &self.enddelay
    }
}
#[doc = "PTR (rw) register accessor: Description cluster: Beginning address in RAM of this sequence\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Description cluster: Beginning address in RAM of this sequence"]
pub mod ptr;
#[doc = "CNT (rw) register accessor: Description cluster: Number of values (duty cycles) in this sequence\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Description cluster: Number of values (duty cycles) in this sequence"]
pub mod cnt;
#[doc = "REFRESH (rw) register accessor: Description cluster: Number of additional PWM periods between samples loaded into compare register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refresh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refresh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refresh`]
module"]
#[doc(alias = "REFRESH")]
pub type Refresh = crate::Reg<refresh::RefreshSpec>;
#[doc = "Description cluster: Number of additional PWM periods between samples loaded into compare register"]
pub mod refresh;
#[doc = "ENDDELAY (rw) register accessor: Description cluster: Time added after the sequence\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enddelay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enddelay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enddelay`]
module"]
#[doc(alias = "ENDDELAY")]
pub type Enddelay = crate::Reg<enddelay::EnddelaySpec>;
#[doc = "Description cluster: Time added after the sequence"]
pub mod enddelay;
