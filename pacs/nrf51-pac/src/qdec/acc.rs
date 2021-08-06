#[doc = "Register `ACC` reader"]
pub struct R(crate::R<ACC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Accumulated valid transitions register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc](index.html) module"]
pub struct ACC_SPEC;
impl crate::RegisterSpec for ACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acc::R](R) reader structure"]
impl crate::Readable for ACC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACC to value 0"]
impl crate::Resettable for ACC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
