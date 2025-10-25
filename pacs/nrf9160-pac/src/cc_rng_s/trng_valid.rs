#[doc = "Register `TRNG_VALID` reader"]
pub type R = crate::R<TrngValidSpec>;
#[doc = "A value of 1 indicates that collection of bits in the TRNG is completed, and data can be read from EHR_DATA registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EhrData {
    #[doc = "0: Collection of bits not valid."]
    NotValid = 0,
    #[doc = "1: Collection of bits valid."]
    Valid = 1,
}
impl From<EhrData> for bool {
    #[inline(always)]
    fn from(variant: EhrData) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHR_DATA` reader - A value of 1 indicates that collection of bits in the TRNG is completed, and data can be read from EHR_DATA registers."]
pub type EhrDataR = crate::BitReader<EhrData>;
impl EhrDataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EhrData {
        match self.bits {
            false => EhrData::NotValid,
            true => EhrData::Valid,
        }
    }
    #[doc = "Collection of bits not valid."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == EhrData::NotValid
    }
    #[doc = "Collection of bits valid."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == EhrData::Valid
    }
}
impl R {
    #[doc = "Bit 0 - A value of 1 indicates that collection of bits in the TRNG is completed, and data can be read from EHR_DATA registers."]
    #[inline(always)]
    pub fn ehr_data(&self) -> EhrDataR {
        EhrDataR::new((self.bits & 1) != 0)
    }
}
#[doc = "This register indicates if TRNG entropy collection is valid.\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_valid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngValidSpec;
impl crate::RegisterSpec for TrngValidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_valid::R`](R) reader structure"]
impl crate::Readable for TrngValidSpec {}
#[doc = "`reset()` method sets TRNG_VALID to value 0"]
impl crate::Resettable for TrngValidSpec {}
