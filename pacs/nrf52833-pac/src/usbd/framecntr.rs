#[doc = "Register `FRAMECNTR` reader"]
pub struct R(crate::R<FRAMECNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMECNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMECNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMECNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAMECNTR` reader - Returns the current value of the start of frame counter"]
pub struct FRAMECNTR_R(crate::FieldReader<u16, u16>);
impl FRAMECNTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FRAMECNTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMECNTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - Returns the current value of the start of frame counter"]
    #[inline(always)]
    pub fn framecntr(&self) -> FRAMECNTR_R {
        FRAMECNTR_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Returns the current value of the start of frame counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framecntr](index.html) module"]
pub struct FRAMECNTR_SPEC;
impl crate::RegisterSpec for FRAMECNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framecntr::R](R) reader structure"]
impl crate::Readable for FRAMECNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRAMECNTR to value 0"]
impl crate::Resettable for FRAMECNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
