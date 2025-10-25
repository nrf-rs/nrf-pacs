#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0420],
    cpuid: Cpuid,
    _reserved1: [u8; 0x1c],
    extperi: [Extperi; 1],
    _reserved2: [u8; 0x1c],
    extram: [Extram; 1],
    _reserved3: [u8; 0x1c],
    extcode: [Extcode; 1],
}
impl RegisterBlock {
    #[doc = "0x420 - CPU ID of this subsystem"]
    #[inline(always)]
    pub const fn cpuid(&self) -> &Cpuid {
        &self.cpuid
    }
    #[doc = "0x440 - Unspecified"]
    #[inline(always)]
    pub const fn extperi(&self, n: usize) -> &Extperi {
        &self.extperi[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x440 - Unspecified"]
    #[inline(always)]
    pub fn extperi_iter(&self) -> impl Iterator<Item = &Extperi> {
        self.extperi.iter()
    }
    #[doc = "0x460 - Unspecified"]
    #[inline(always)]
    pub const fn extram(&self, n: usize) -> &Extram {
        &self.extram[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x460 - Unspecified"]
    #[inline(always)]
    pub fn extram_iter(&self) -> impl Iterator<Item = &Extram> {
        self.extram.iter()
    }
    #[doc = "0x480 - Unspecified"]
    #[inline(always)]
    pub const fn extcode(&self, n: usize) -> &Extcode {
        &self.extcode[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x480 - Unspecified"]
    #[inline(always)]
    pub fn extcode_iter(&self) -> impl Iterator<Item = &Extcode> {
        self.extcode.iter()
    }
}
#[doc = "CPUID (r) register accessor: CPU ID of this subsystem\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuid`] module"]
#[doc(alias = "CPUID")]
pub type Cpuid = crate::Reg<cpuid::CpuidSpec>;
#[doc = "CPU ID of this subsystem"]
pub mod cpuid;
#[doc = "Unspecified"]
pub use self::extperi::Extperi;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod extperi;
#[doc = "Unspecified"]
pub use self::extram::Extram;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod extram;
#[doc = "Unspecified"]
pub use self::extcode::Extcode;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod extcode;
