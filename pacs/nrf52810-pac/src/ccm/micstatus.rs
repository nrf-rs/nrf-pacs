#[doc = "Register `MICSTATUS` reader"]
pub type R = crate::R<MicstatusSpec>;
#[doc = "The result of the MIC check performed during the previous decryption operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Micstatus {
    #[doc = "0: MIC check failed"]
    CheckFailed = 0,
    #[doc = "1: MIC check passed"]
    CheckPassed = 1,
}
impl From<Micstatus> for bool {
    #[inline(always)]
    fn from(variant: Micstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MICSTATUS` reader - The result of the MIC check performed during the previous decryption operation"]
pub type MicstatusR = crate::BitReader<Micstatus>;
impl MicstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Micstatus {
        match self.bits {
            false => Micstatus::CheckFailed,
            true => Micstatus::CheckPassed,
        }
    }
    #[doc = "MIC check failed"]
    #[inline(always)]
    pub fn is_check_failed(&self) -> bool {
        *self == Micstatus::CheckFailed
    }
    #[doc = "MIC check passed"]
    #[inline(always)]
    pub fn is_check_passed(&self) -> bool {
        *self == Micstatus::CheckPassed
    }
}
impl R {
    #[doc = "Bit 0 - The result of the MIC check performed during the previous decryption operation"]
    #[inline(always)]
    pub fn micstatus(&self) -> MicstatusR {
        MicstatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "MIC check result\n\nYou can [`read`](crate::Reg::read) this register and get [`micstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MicstatusSpec;
impl crate::RegisterSpec for MicstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`micstatus::R`](R) reader structure"]
impl crate::Readable for MicstatusSpec {}
#[doc = "`reset()` method sets MICSTATUS to value 0"]
impl crate::Resettable for MicstatusSpec {}
