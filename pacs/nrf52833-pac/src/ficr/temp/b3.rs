#[doc = "Register `B3` reader"]
pub struct R(crate::R<B3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `B` reader - B (y-intercept)"]
pub type B_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - B (y-intercept)"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "Y-intercept B3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b3](index.html) module"]
pub struct B3_SPEC;
impl crate::RegisterSpec for B3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b3::R](R) reader structure"]
impl crate::Readable for B3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets B3 to value 0xffff_ffff"]
impl crate::Resettable for B3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
