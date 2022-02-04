#[doc = "Register `T2` reader"]
pub struct R(crate::R<T2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T` reader - T (segment end) register"]
pub struct T_R(crate::FieldReader<u8, u8>);
impl T_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - T (segment end) register"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Segment end T2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2](index.html) module"]
pub struct T2_SPEC;
impl crate::RegisterSpec for T2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t2::R](R) reader structure"]
impl crate::Readable for T2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T2 to value 0xffff_ffff"]
impl crate::Resettable for T2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
