#[doc = "Register `CAP` reader"]
pub type R = crate::R<CapSpec>;
#[doc = "Show ARM TrustZone status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tzm {
    #[doc = "0: ARM TrustZone support not available"]
    NotAvailable = 0,
    #[doc = "1: ARM TrustZone support is available"]
    Enabled = 1,
}
impl From<Tzm> for bool {
    #[inline(always)]
    fn from(variant: Tzm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TZM` reader - Show ARM TrustZone status"]
pub type TzmR = crate::BitReader<Tzm>;
impl TzmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tzm {
        match self.bits {
            false => Tzm::NotAvailable,
            true => Tzm::Enabled,
        }
    }
    #[doc = "ARM TrustZone support not available"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == Tzm::NotAvailable
    }
    #[doc = "ARM TrustZone support is available"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tzm::Enabled
    }
}
impl R {
    #[doc = "Bit 0 - Show ARM TrustZone status"]
    #[inline(always)]
    pub fn tzm(&self) -> TzmR {
        TzmR::new((self.bits & 1) != 0)
    }
}
#[doc = "Show implemented features for the current device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapSpec;
impl crate::RegisterSpec for CapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap::R`](R) reader structure"]
impl crate::Readable for CapSpec {}
#[doc = "`reset()` method sets CAP to value 0x01"]
impl crate::Resettable for CapSpec {
    const RESET_VALUE: u32 = 0x01;
}
