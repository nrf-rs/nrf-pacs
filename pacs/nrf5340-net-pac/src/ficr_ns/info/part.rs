#[doc = "Register `PART` reader"]
pub struct R(crate::R<PART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PART` reader - Part code"]
pub type PART_R = crate::FieldReader<u32, PART_A>;
#[doc = "Part code\n\nValue on reset: 21312"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PART_A {
    #[doc = "21312: nRF5340"]
    N5340 = 21312,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<PART_A> for u32 {
    #[inline(always)]
    fn from(variant: PART_A) -> Self {
        variant as _
    }
}
impl PART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PART_A> {
        match self.bits {
            21312 => Some(PART_A::N5340),
            4294967295 => Some(PART_A::UNSPECIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `N5340`"]
    #[inline(always)]
    pub fn is_n5340(&self) -> bool {
        *self == PART_A::N5340
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == PART_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Part code"]
    #[inline(always)]
    pub fn part(&self) -> PART_R {
        PART_R::new(self.bits)
    }
}
#[doc = "Part code\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [part](index.html) module"]
pub struct PART_SPEC;
impl crate::RegisterSpec for PART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [part::R](R) reader structure"]
impl crate::Readable for PART_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PART to value 0x5340"]
impl crate::Resettable for PART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5340
    }
}
