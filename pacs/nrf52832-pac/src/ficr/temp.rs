#[repr(C)]
#[doc = "Registers storing factory TEMP module linearization coefficients"]
#[doc(alias = "TEMP")]
pub struct Temp {
    a0: A0,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    b0: B0,
    b1: B1,
    b2: B2,
    b3: B3,
    b4: B4,
    b5: B5,
    t0: T0,
    t1: T1,
    t2: T2,
    t3: T3,
    t4: T4,
}
impl Temp {
    #[doc = "0x00 - Slope definition A0."]
    #[inline(always)]
    pub const fn a0(&self) -> &A0 {
        &self.a0
    }
    #[doc = "0x04 - Slope definition A1."]
    #[inline(always)]
    pub const fn a1(&self) -> &A1 {
        &self.a1
    }
    #[doc = "0x08 - Slope definition A2."]
    #[inline(always)]
    pub const fn a2(&self) -> &A2 {
        &self.a2
    }
    #[doc = "0x0c - Slope definition A3."]
    #[inline(always)]
    pub const fn a3(&self) -> &A3 {
        &self.a3
    }
    #[doc = "0x10 - Slope definition A4."]
    #[inline(always)]
    pub const fn a4(&self) -> &A4 {
        &self.a4
    }
    #[doc = "0x14 - Slope definition A5."]
    #[inline(always)]
    pub const fn a5(&self) -> &A5 {
        &self.a5
    }
    #[doc = "0x18 - y-intercept B0."]
    #[inline(always)]
    pub const fn b0(&self) -> &B0 {
        &self.b0
    }
    #[doc = "0x1c - y-intercept B1."]
    #[inline(always)]
    pub const fn b1(&self) -> &B1 {
        &self.b1
    }
    #[doc = "0x20 - y-intercept B2."]
    #[inline(always)]
    pub const fn b2(&self) -> &B2 {
        &self.b2
    }
    #[doc = "0x24 - y-intercept B3."]
    #[inline(always)]
    pub const fn b3(&self) -> &B3 {
        &self.b3
    }
    #[doc = "0x28 - y-intercept B4."]
    #[inline(always)]
    pub const fn b4(&self) -> &B4 {
        &self.b4
    }
    #[doc = "0x2c - y-intercept B5."]
    #[inline(always)]
    pub const fn b5(&self) -> &B5 {
        &self.b5
    }
    #[doc = "0x30 - Segment end T0."]
    #[inline(always)]
    pub const fn t0(&self) -> &T0 {
        &self.t0
    }
    #[doc = "0x34 - Segment end T1."]
    #[inline(always)]
    pub const fn t1(&self) -> &T1 {
        &self.t1
    }
    #[doc = "0x38 - Segment end T2."]
    #[inline(always)]
    pub const fn t2(&self) -> &T2 {
        &self.t2
    }
    #[doc = "0x3c - Segment end T3."]
    #[inline(always)]
    pub const fn t3(&self) -> &T3 {
        &self.t3
    }
    #[doc = "0x40 - Segment end T4."]
    #[inline(always)]
    pub const fn t4(&self) -> &T4 {
        &self.t4
    }
}
#[doc = "A0 (r) register accessor: Slope definition A0.\n\nYou can [`read`](crate::Reg::read) this register and get [`a0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a0`] module"]
pub type A0 = crate::Reg<a0::A0Spec>;
#[doc = "Slope definition A0."]
pub mod a0;
#[doc = "A1 (r) register accessor: Slope definition A1.\n\nYou can [`read`](crate::Reg::read) this register and get [`a1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a1`] module"]
pub type A1 = crate::Reg<a1::A1Spec>;
#[doc = "Slope definition A1."]
pub mod a1;
#[doc = "A2 (r) register accessor: Slope definition A2.\n\nYou can [`read`](crate::Reg::read) this register and get [`a2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2`] module"]
pub type A2 = crate::Reg<a2::A2Spec>;
#[doc = "Slope definition A2."]
pub mod a2;
#[doc = "A3 (r) register accessor: Slope definition A3.\n\nYou can [`read`](crate::Reg::read) this register and get [`a3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a3`] module"]
pub type A3 = crate::Reg<a3::A3Spec>;
#[doc = "Slope definition A3."]
pub mod a3;
#[doc = "A4 (r) register accessor: Slope definition A4.\n\nYou can [`read`](crate::Reg::read) this register and get [`a4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a4`] module"]
pub type A4 = crate::Reg<a4::A4Spec>;
#[doc = "Slope definition A4."]
pub mod a4;
#[doc = "A5 (r) register accessor: Slope definition A5.\n\nYou can [`read`](crate::Reg::read) this register and get [`a5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a5`] module"]
pub type A5 = crate::Reg<a5::A5Spec>;
#[doc = "Slope definition A5."]
pub mod a5;
#[doc = "B0 (r) register accessor: y-intercept B0.\n\nYou can [`read`](crate::Reg::read) this register and get [`b0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0`] module"]
pub type B0 = crate::Reg<b0::B0Spec>;
#[doc = "y-intercept B0."]
pub mod b0;
#[doc = "B1 (r) register accessor: y-intercept B1.\n\nYou can [`read`](crate::Reg::read) this register and get [`b1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b1`] module"]
pub type B1 = crate::Reg<b1::B1Spec>;
#[doc = "y-intercept B1."]
pub mod b1;
#[doc = "B2 (r) register accessor: y-intercept B2.\n\nYou can [`read`](crate::Reg::read) this register and get [`b2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2`] module"]
pub type B2 = crate::Reg<b2::B2Spec>;
#[doc = "y-intercept B2."]
pub mod b2;
#[doc = "B3 (r) register accessor: y-intercept B3.\n\nYou can [`read`](crate::Reg::read) this register and get [`b3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b3`] module"]
pub type B3 = crate::Reg<b3::B3Spec>;
#[doc = "y-intercept B3."]
pub mod b3;
#[doc = "B4 (r) register accessor: y-intercept B4.\n\nYou can [`read`](crate::Reg::read) this register and get [`b4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b4`] module"]
pub type B4 = crate::Reg<b4::B4Spec>;
#[doc = "y-intercept B4."]
pub mod b4;
#[doc = "B5 (r) register accessor: y-intercept B5.\n\nYou can [`read`](crate::Reg::read) this register and get [`b5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b5`] module"]
pub type B5 = crate::Reg<b5::B5Spec>;
#[doc = "y-intercept B5."]
pub mod b5;
#[doc = "T0 (r) register accessor: Segment end T0.\n\nYou can [`read`](crate::Reg::read) this register and get [`t0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0`] module"]
pub type T0 = crate::Reg<t0::T0Spec>;
#[doc = "Segment end T0."]
pub mod t0;
#[doc = "T1 (r) register accessor: Segment end T1.\n\nYou can [`read`](crate::Reg::read) this register and get [`t1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1`] module"]
pub type T1 = crate::Reg<t1::T1Spec>;
#[doc = "Segment end T1."]
pub mod t1;
#[doc = "T2 (r) register accessor: Segment end T2.\n\nYou can [`read`](crate::Reg::read) this register and get [`t2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2`] module"]
pub type T2 = crate::Reg<t2::T2Spec>;
#[doc = "Segment end T2."]
pub mod t2;
#[doc = "T3 (r) register accessor: Segment end T3.\n\nYou can [`read`](crate::Reg::read) this register and get [`t3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3`] module"]
pub type T3 = crate::Reg<t3::T3Spec>;
#[doc = "Segment end T3."]
pub mod t3;
#[doc = "T4 (r) register accessor: Segment end T4.\n\nYou can [`read`](crate::Reg::read) this register and get [`t4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t4`] module"]
pub type T4 = crate::Reg<t4::T4Spec>;
#[doc = "Segment end T4."]
pub mod t4;
