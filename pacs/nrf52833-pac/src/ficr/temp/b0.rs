#[doc = "Register `B0` reader"]
pub struct R(crate::R<B0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `B` reader - B (y-intercept)"]
pub struct B_R(crate::FieldReader<u16, u16>);
impl B_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:13 - B (y-intercept)"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "Y-intercept B0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0](index.html) module"]
pub struct B0_SPEC;
impl crate::RegisterSpec for B0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b0::R](R) reader structure"]
impl crate::Readable for B0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets B0 to value 0xffff_ffff"]
impl crate::Resettable for B0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
