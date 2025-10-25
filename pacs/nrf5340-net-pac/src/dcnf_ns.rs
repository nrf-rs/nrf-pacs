#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0420],
    cpuid: Cpuid,
}
impl RegisterBlock {
    #[doc = "0x420 - CPU ID of this subsystem"]
    #[inline(always)]
    pub const fn cpuid(&self) -> &Cpuid {
        &self.cpuid
    }
}
#[doc = "CPUID (r) register accessor: CPU ID of this subsystem\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuid`] module"]
#[doc(alias = "CPUID")]
pub type Cpuid = crate::Reg<cpuid::CpuidSpec>;
#[doc = "CPU ID of this subsystem"]
pub mod cpuid;
