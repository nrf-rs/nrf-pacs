#[doc = "Register `RNG_HW_FLAGS` reader"]
pub type R = crate::R<RngHwFlagsSpec>;
#[doc = "Data width supported by the entropy collector\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EhrWidth {
    #[doc = "0: 128 bits EHR width"]
    _128bits = 0,
    #[doc = "1: 192 bits EHR width"]
    _192bits = 1,
}
impl From<EhrWidth> for bool {
    #[inline(always)]
    fn from(variant: EhrWidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHR_WIDTH` reader - Data width supported by the entropy collector"]
pub type EhrWidthR = crate::BitReader<EhrWidth>;
impl EhrWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EhrWidth {
        match self.bits {
            false => EhrWidth::_128bits,
            true => EhrWidth::_192bits,
        }
    }
    #[doc = "128 bits EHR width"]
    #[inline(always)]
    pub fn is_128bits(&self) -> bool {
        *self == EhrWidth::_128bits
    }
    #[doc = "192 bits EHR width"]
    #[inline(always)]
    pub fn is_192bits(&self) -> bool {
        *self == EhrWidth::_192bits
    }
}
#[doc = "Field `CRNGT_EXISTS` reader - If this flag is set, the engine include support for continuous random number generator test."]
pub type CrngtExistsR = crate::BitReader;
#[doc = "Field `AUTOCORR_EXISTS` reader - If this flag is set, the engine include support for autocorrelation test."]
pub type AutocorrExistsR = crate::BitReader;
#[doc = "Field `BYPASS_EXISTS` reader - If this flag is set, the engine include support for bypassing TRNG tests."]
pub type BypassExistsR = crate::BitReader;
#[doc = "Field `PRNG_EXISTS` reader - If this flag is set, the engine include a pseudo-random number generator."]
pub type PrngExistsR = crate::BitReader;
#[doc = "Field `KAT_EXISTS` reader - If this flag is set, the engine include support for known answer tests."]
pub type KatExistsR = crate::BitReader;
#[doc = "Field `RESEEDING_EXISTS` reader - If this flag is set, the engine include support for automatic reseeding."]
pub type ReseedingExistsR = crate::BitReader;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RngUse5Sboxes {
    #[doc = "0: 20 SBOX AES"]
    Disable = 0,
    #[doc = "1: 5 SBOX AES"]
    Enable = 1,
}
impl From<RngUse5Sboxes> for bool {
    #[inline(always)]
    fn from(variant: RngUse5Sboxes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG_USE_5_SBOXES` reader - "]
pub type RngUse5SboxesR = crate::BitReader<RngUse5Sboxes>;
impl RngUse5SboxesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RngUse5Sboxes {
        match self.bits {
            false => RngUse5Sboxes::Disable,
            true => RngUse5Sboxes::Enable,
        }
    }
    #[doc = "20 SBOX AES"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RngUse5Sboxes::Disable
    }
    #[doc = "5 SBOX AES"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RngUse5Sboxes::Enable
    }
}
impl R {
    #[doc = "Bit 0 - Data width supported by the entropy collector"]
    #[inline(always)]
    pub fn ehr_width(&self) -> EhrWidthR {
        EhrWidthR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this flag is set, the engine include support for continuous random number generator test."]
    #[inline(always)]
    pub fn crngt_exists(&self) -> CrngtExistsR {
        CrngtExistsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this flag is set, the engine include support for autocorrelation test."]
    #[inline(always)]
    pub fn autocorr_exists(&self) -> AutocorrExistsR {
        AutocorrExistsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this flag is set, the engine include support for bypassing TRNG tests."]
    #[inline(always)]
    pub fn bypass_exists(&self) -> BypassExistsR {
        BypassExistsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this flag is set, the engine include a pseudo-random number generator."]
    #[inline(always)]
    pub fn prng_exists(&self) -> PrngExistsR {
        PrngExistsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this flag is set, the engine include support for known answer tests."]
    #[inline(always)]
    pub fn kat_exists(&self) -> KatExistsR {
        KatExistsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If this flag is set, the engine include support for automatic reseeding."]
    #[inline(always)]
    pub fn reseeding_exists(&self) -> ReseedingExistsR {
        ReseedingExistsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rng_use_5_sboxes(&self) -> RngUse5SboxesR {
        RngUse5SboxesR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Hardware configuration of RNG engine. Reset value holds the supported features.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_hw_flags::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngHwFlagsSpec;
impl crate::RegisterSpec for RngHwFlagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_hw_flags::R`](R) reader structure"]
impl crate::Readable for RngHwFlagsSpec {}
#[doc = "`reset()` method sets RNG_HW_FLAGS to value 0x0f"]
impl crate::Resettable for RngHwFlagsSpec {
    const RESET_VALUE: u32 = 0x0f;
}
