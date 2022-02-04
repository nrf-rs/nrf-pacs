#[doc = "Register `SIZERAMBLOCK[%s]` reader"]
pub struct R(crate::R<SIZERAMBLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIZERAMBLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIZERAMBLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIZERAMBLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sizeramblock](index.html) module"]
pub struct SIZERAMBLOCK_SPEC;
impl crate::RegisterSpec for SIZERAMBLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sizeramblock::R](R) reader structure"]
impl crate::Readable for SIZERAMBLOCK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIZERAMBLOCK[%s]
to value 0xffff_ffff"]
impl crate::Resettable for SIZERAMBLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
