#[doc = "Register `SIZERAMBLOCKS` reader"]
pub struct R(crate::R<SIZERAMBLOCKS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIZERAMBLOCKS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIZERAMBLOCKS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIZERAMBLOCKS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Size of RAM blocks in bytes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sizeramblocks](index.html) module"]
pub struct SIZERAMBLOCKS_SPEC;
impl crate::RegisterSpec for SIZERAMBLOCKS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sizeramblocks::R](R) reader structure"]
impl crate::Readable for SIZERAMBLOCKS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIZERAMBLOCKS to value 0xffff_ffff"]
impl crate::Resettable for SIZERAMBLOCKS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
