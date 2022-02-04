#[doc = "Register `MICSTATUS` reader"]
pub struct R(crate::R<MICSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MICSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MICSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MICSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "The result of the MIC check performed during the previous decryption operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MICSTATUS_A {
    #[doc = "0: MIC check failed"]
    CHECKFAILED = 0,
    #[doc = "1: MIC check passed"]
    CHECKPASSED = 1,
}
impl From<MICSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: MICSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MICSTATUS` reader - The result of the MIC check performed during the previous decryption operation"]
pub struct MICSTATUS_R(crate::FieldReader<bool, MICSTATUS_A>);
impl MICSTATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MICSTATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MICSTATUS_A {
        match self.bits {
            false => MICSTATUS_A::CHECKFAILED,
            true => MICSTATUS_A::CHECKPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `CHECKFAILED`"]
    #[inline(always)]
    pub fn is_check_failed(&self) -> bool {
        **self == MICSTATUS_A::CHECKFAILED
    }
    #[doc = "Checks if the value of the field is `CHECKPASSED`"]
    #[inline(always)]
    pub fn is_check_passed(&self) -> bool {
        **self == MICSTATUS_A::CHECKPASSED
    }
}
impl core::ops::Deref for MICSTATUS_R {
    type Target = crate::FieldReader<bool, MICSTATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The result of the MIC check performed during the previous decryption operation"]
    #[inline(always)]
    pub fn micstatus(&self) -> MICSTATUS_R {
        MICSTATUS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "MIC check result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micstatus](index.html) module"]
pub struct MICSTATUS_SPEC;
impl crate::RegisterSpec for MICSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [micstatus::R](R) reader structure"]
impl crate::Readable for MICSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MICSTATUS to value 0"]
impl crate::Resettable for MICSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
