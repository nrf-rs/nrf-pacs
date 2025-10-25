#[doc = "Register `SEMSTAT` reader"]
pub type R = crate::R<SemstatSpec>;
#[doc = "Semaphore status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Semstat {
    #[doc = "0: Semaphore is free"]
    Free = 0,
    #[doc = "1: Semaphore is assigned to CPU"]
    Cpu = 1,
    #[doc = "2: Semaphore is assigned to SPI slave"]
    Spis = 2,
    #[doc = "3: Semaphore is assigned to SPI but a handover to the CPU is pending"]
    Cpupending = 3,
}
impl From<Semstat> for u8 {
    #[inline(always)]
    fn from(variant: Semstat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Semstat {
    type Ux = u8;
}
impl crate::IsEnum for Semstat {}
#[doc = "Field `SEMSTAT` reader - Semaphore status"]
pub type SemstatR = crate::FieldReader<Semstat>;
impl SemstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Semstat {
        match self.bits {
            0 => Semstat::Free,
            1 => Semstat::Cpu,
            2 => Semstat::Spis,
            3 => Semstat::Cpupending,
            _ => unreachable!(),
        }
    }
    #[doc = "Semaphore is free"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == Semstat::Free
    }
    #[doc = "Semaphore is assigned to CPU"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == Semstat::Cpu
    }
    #[doc = "Semaphore is assigned to SPI slave"]
    #[inline(always)]
    pub fn is_spis(&self) -> bool {
        *self == Semstat::Spis
    }
    #[doc = "Semaphore is assigned to SPI but a handover to the CPU is pending"]
    #[inline(always)]
    pub fn is_cpupending(&self) -> bool {
        *self == Semstat::Cpupending
    }
}
impl R {
    #[doc = "Bits 0:1 - Semaphore status"]
    #[inline(always)]
    pub fn semstat(&self) -> SemstatR {
        SemstatR::new((self.bits & 3) as u8)
    }
}
#[doc = "Semaphore status register\n\nYou can [`read`](crate::Reg::read) this register and get [`semstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SemstatSpec;
impl crate::RegisterSpec for SemstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`semstat::R`](R) reader structure"]
impl crate::Readable for SemstatSpec {}
#[doc = "`reset()` method sets SEMSTAT to value 0x01"]
impl crate::Resettable for SemstatSpec {
    const RESET_VALUE: u32 = 0x01;
}
