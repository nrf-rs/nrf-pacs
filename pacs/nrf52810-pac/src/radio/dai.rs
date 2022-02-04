#[doc = "Register `DAI` reader"]
pub struct R(crate::R<DAI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DAI` reader - Device address match index"]
pub struct DAI_R(crate::FieldReader<u8, u8>);
impl DAI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Device address match index"]
    #[inline(always)]
    pub fn dai(&self) -> DAI_R {
        DAI_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Device address match index\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dai](index.html) module"]
pub struct DAI_SPEC;
impl crate::RegisterSpec for DAI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dai::R](R) reader structure"]
impl crate::Readable for DAI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAI to value 0"]
impl crate::Resettable for DAI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
