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
#[doc = "Part code\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PART_A {
    #[doc = "37216: nRF9160"]
    N9160 = 37216,
    #[doc = "37152: nRF9120"]
    N9120 = 37152,
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
            37216 => Some(PART_A::N9160),
            37152 => Some(PART_A::N9120),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `N9160`"]
    #[inline(always)]
    pub fn is_n9160(&self) -> bool {
        *self == PART_A::N9160
    }
    #[doc = "Checks if the value of the field is `N9120`"]
    #[inline(always)]
    pub fn is_n9120(&self) -> bool {
        *self == PART_A::N9120
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
#[doc = "`reset()` method sets PART to value 0xffff_ffff"]
impl crate::Resettable for PART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
