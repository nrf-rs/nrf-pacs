#[doc = "Register `A5` reader"]
pub struct R(crate::R<A5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `A` reader - A (slope definition) register."]
pub struct A_R(crate::FieldReader<u16, u16>);
impl A_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - A (slope definition) register."]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Slope definition A5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a5](index.html) module"]
pub struct A5_SPEC;
impl crate::RegisterSpec for A5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a5::R](R) reader structure"]
impl crate::Readable for A5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets A5 to value 0xffff_ffff"]
impl crate::Resettable for A5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
