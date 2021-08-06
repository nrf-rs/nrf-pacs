#[doc = "Register `DEVARCH` reader"]
pub struct R(crate::R<DEVARCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVARCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVARCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVARCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Architecture` reader - Contains the CTI device architecture."]
pub struct ARCHITECTURE_R(crate::FieldReader<bool, bool>);
impl ARCHITECTURE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARCHITECTURE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARCHITECTURE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Contains the CTI device architecture."]
    #[inline(always)]
    pub fn architecture(&self) -> ARCHITECTURE_R {
        ARCHITECTURE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Device Architecture register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devarch](index.html) module"]
pub struct DEVARCH_SPEC;
impl crate::RegisterSpec for DEVARCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devarch::R](R) reader structure"]
impl crate::Readable for DEVARCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVARCH to value 0x4770_1a14"]
impl crate::Resettable for DEVARCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4770_1a14
    }
}
