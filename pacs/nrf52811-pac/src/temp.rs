#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    _reserved2: [u8; 0xf8],
    events_datardy: EventsDatardy,
    _reserved3: [u8; 0x0200],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved5: [u8; 0x01fc],
    temp: Temp,
    _reserved6: [u8; 0x14],
    a0: A0,
    a1: A1,
    a2: A2,
    a3: A3,
    a4: A4,
    a5: A5,
    _reserved12: [u8; 0x08],
    b0: B0,
    b1: B1,
    b2: B2,
    b3: B3,
    b4: B4,
    b5: B5,
    _reserved18: [u8; 0x08],
    t0: T0,
    t1: T1,
    t2: T2,
    t3: T3,
    t4: T4,
}
impl RegisterBlock {
    #[doc = "0x00 - Start temperature measurement"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stop temperature measurement"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x100 - Temperature measurement complete, data ready"]
    #[inline(always)]
    pub const fn events_datardy(&self) -> &EventsDatardy {
        &self.events_datardy
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x508 - Temperature in degC (0.25deg steps)"]
    #[inline(always)]
    pub const fn temp(&self) -> &Temp {
        &self.temp
    }
    #[doc = "0x520 - Slope of 1st piece wise linear function"]
    #[inline(always)]
    pub const fn a0(&self) -> &A0 {
        &self.a0
    }
    #[doc = "0x524 - Slope of 2nd piece wise linear function"]
    #[inline(always)]
    pub const fn a1(&self) -> &A1 {
        &self.a1
    }
    #[doc = "0x528 - Slope of 3rd piece wise linear function"]
    #[inline(always)]
    pub const fn a2(&self) -> &A2 {
        &self.a2
    }
    #[doc = "0x52c - Slope of 4th piece wise linear function"]
    #[inline(always)]
    pub const fn a3(&self) -> &A3 {
        &self.a3
    }
    #[doc = "0x530 - Slope of 5th piece wise linear function"]
    #[inline(always)]
    pub const fn a4(&self) -> &A4 {
        &self.a4
    }
    #[doc = "0x534 - Slope of 6th piece wise linear function"]
    #[inline(always)]
    pub const fn a5(&self) -> &A5 {
        &self.a5
    }
    #[doc = "0x540 - y-intercept of 1st piece wise linear function"]
    #[inline(always)]
    pub const fn b0(&self) -> &B0 {
        &self.b0
    }
    #[doc = "0x544 - y-intercept of 2nd piece wise linear function"]
    #[inline(always)]
    pub const fn b1(&self) -> &B1 {
        &self.b1
    }
    #[doc = "0x548 - y-intercept of 3rd piece wise linear function"]
    #[inline(always)]
    pub const fn b2(&self) -> &B2 {
        &self.b2
    }
    #[doc = "0x54c - y-intercept of 4th piece wise linear function"]
    #[inline(always)]
    pub const fn b3(&self) -> &B3 {
        &self.b3
    }
    #[doc = "0x550 - y-intercept of 5th piece wise linear function"]
    #[inline(always)]
    pub const fn b4(&self) -> &B4 {
        &self.b4
    }
    #[doc = "0x554 - y-intercept of 6th piece wise linear function"]
    #[inline(always)]
    pub const fn b5(&self) -> &B5 {
        &self.b5
    }
    #[doc = "0x560 - End point of 1st piece wise linear function"]
    #[inline(always)]
    pub const fn t0(&self) -> &T0 {
        &self.t0
    }
    #[doc = "0x564 - End point of 2nd piece wise linear function"]
    #[inline(always)]
    pub const fn t1(&self) -> &T1 {
        &self.t1
    }
    #[doc = "0x568 - End point of 3rd piece wise linear function"]
    #[inline(always)]
    pub const fn t2(&self) -> &T2 {
        &self.t2
    }
    #[doc = "0x56c - End point of 4th piece wise linear function"]
    #[inline(always)]
    pub const fn t3(&self) -> &T3 {
        &self.t3
    }
    #[doc = "0x570 - End point of 5th piece wise linear function"]
    #[inline(always)]
    pub const fn t4(&self) -> &T4 {
        &self.t4
    }
}
#[doc = "TASKS_START (w) register accessor: Start temperature measurement\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start temperature measurement"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop temperature measurement\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop temperature measurement"]
pub mod tasks_stop;
#[doc = "EVENTS_DATARDY (rw) register accessor: Temperature measurement complete, data ready\n\nYou can [`read`](crate::Reg::read) this register and get [`events_datardy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_datardy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_datardy`] module"]
#[doc(alias = "EVENTS_DATARDY")]
pub type EventsDatardy = crate::Reg<events_datardy::EventsDatardySpec>;
#[doc = "Temperature measurement complete, data ready"]
pub mod events_datardy;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "TEMP (r) register accessor: Temperature in degC (0.25deg steps)\n\nYou can [`read`](crate::Reg::read) this register and get [`temp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp`] module"]
#[doc(alias = "TEMP")]
pub type Temp = crate::Reg<temp::TempSpec>;
#[doc = "Temperature in degC (0.25deg steps)"]
pub mod temp;
#[doc = "A0 (rw) register accessor: Slope of 1st piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a0`] module"]
pub type A0 = crate::Reg<a0::A0Spec>;
#[doc = "Slope of 1st piece wise linear function"]
pub mod a0;
#[doc = "A1 (rw) register accessor: Slope of 2nd piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a1`] module"]
pub type A1 = crate::Reg<a1::A1Spec>;
#[doc = "Slope of 2nd piece wise linear function"]
pub mod a1;
#[doc = "A2 (rw) register accessor: Slope of 3rd piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2`] module"]
pub type A2 = crate::Reg<a2::A2Spec>;
#[doc = "Slope of 3rd piece wise linear function"]
pub mod a2;
#[doc = "A3 (rw) register accessor: Slope of 4th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a3`] module"]
pub type A3 = crate::Reg<a3::A3Spec>;
#[doc = "Slope of 4th piece wise linear function"]
pub mod a3;
#[doc = "A4 (rw) register accessor: Slope of 5th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a4`] module"]
pub type A4 = crate::Reg<a4::A4Spec>;
#[doc = "Slope of 5th piece wise linear function"]
pub mod a4;
#[doc = "A5 (rw) register accessor: Slope of 6th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a5`] module"]
pub type A5 = crate::Reg<a5::A5Spec>;
#[doc = "Slope of 6th piece wise linear function"]
pub mod a5;
#[doc = "B0 (rw) register accessor: y-intercept of 1st piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0`] module"]
pub type B0 = crate::Reg<b0::B0Spec>;
#[doc = "y-intercept of 1st piece wise linear function"]
pub mod b0;
#[doc = "B1 (rw) register accessor: y-intercept of 2nd piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b1`] module"]
pub type B1 = crate::Reg<b1::B1Spec>;
#[doc = "y-intercept of 2nd piece wise linear function"]
pub mod b1;
#[doc = "B2 (rw) register accessor: y-intercept of 3rd piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b2`] module"]
pub type B2 = crate::Reg<b2::B2Spec>;
#[doc = "y-intercept of 3rd piece wise linear function"]
pub mod b2;
#[doc = "B3 (rw) register accessor: y-intercept of 4th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b3`] module"]
pub type B3 = crate::Reg<b3::B3Spec>;
#[doc = "y-intercept of 4th piece wise linear function"]
pub mod b3;
#[doc = "B4 (rw) register accessor: y-intercept of 5th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b4`] module"]
pub type B4 = crate::Reg<b4::B4Spec>;
#[doc = "y-intercept of 5th piece wise linear function"]
pub mod b4;
#[doc = "B5 (rw) register accessor: y-intercept of 6th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b5`] module"]
pub type B5 = crate::Reg<b5::B5Spec>;
#[doc = "y-intercept of 6th piece wise linear function"]
pub mod b5;
#[doc = "T0 (rw) register accessor: End point of 1st piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0`] module"]
pub type T0 = crate::Reg<t0::T0Spec>;
#[doc = "End point of 1st piece wise linear function"]
pub mod t0;
#[doc = "T1 (rw) register accessor: End point of 2nd piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t1`] module"]
pub type T1 = crate::Reg<t1::T1Spec>;
#[doc = "End point of 2nd piece wise linear function"]
pub mod t1;
#[doc = "T2 (rw) register accessor: End point of 3rd piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t2`] module"]
pub type T2 = crate::Reg<t2::T2Spec>;
#[doc = "End point of 3rd piece wise linear function"]
pub mod t2;
#[doc = "T3 (rw) register accessor: End point of 4th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t3`] module"]
pub type T3 = crate::Reg<t3::T3Spec>;
#[doc = "End point of 4th piece wise linear function"]
pub mod t3;
#[doc = "T4 (rw) register accessor: End point of 5th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t4`] module"]
pub type T4 = crate::Reg<t4::T4Spec>;
#[doc = "End point of 5th piece wise linear function"]
pub mod t4;
