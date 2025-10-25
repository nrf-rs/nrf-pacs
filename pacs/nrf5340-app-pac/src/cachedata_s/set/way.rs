#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "WAY")]
pub struct Way {
    data0: Data0,
    data1: Data1,
    data2: Data2,
    data3: Data3,
}
impl Way {
    #[doc = "0x00 - Description cluster: Cache data bits \\[31:0\\] of SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn data0(&self) -> &Data0 {
        &self.data0
    }
    #[doc = "0x04 - Description cluster: Cache data bits \\[63:32\\] of SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn data1(&self) -> &Data1 {
        &self.data1
    }
    #[doc = "0x08 - Description cluster: Cache data bits \\[95:64\\] of SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn data2(&self) -> &Data2 {
        &self.data2
    }
    #[doc = "0x0c - Description cluster: Cache data bits \\[127:96\\] of SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn data3(&self) -> &Data3 {
        &self.data3
    }
}
#[doc = "DATA0 (rw) register accessor: Description cluster: Cache data bits \\[31:0\\] of SET\\[n\\], WAY\\[o\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`] module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "Description cluster: Cache data bits \\[31:0\\] of SET\\[n\\], WAY\\[o\\]."]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: Description cluster: Cache data bits \\[63:32\\] of SET\\[n\\], WAY\\[o\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`] module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "Description cluster: Cache data bits \\[63:32\\] of SET\\[n\\], WAY\\[o\\]."]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: Description cluster: Cache data bits \\[95:64\\] of SET\\[n\\], WAY\\[o\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2`] module"]
#[doc(alias = "DATA2")]
pub type Data2 = crate::Reg<data2::Data2Spec>;
#[doc = "Description cluster: Cache data bits \\[95:64\\] of SET\\[n\\], WAY\\[o\\]."]
pub mod data2;
#[doc = "DATA3 (rw) register accessor: Description cluster: Cache data bits \\[127:96\\] of SET\\[n\\], WAY\\[o\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3`] module"]
#[doc(alias = "DATA3")]
pub type Data3 = crate::Reg<data3::Data3Spec>;
#[doc = "Description cluster: Cache data bits \\[127:96\\] of SET\\[n\\], WAY\\[o\\]."]
pub mod data3;
