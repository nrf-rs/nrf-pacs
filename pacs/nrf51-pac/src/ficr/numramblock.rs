#[doc = "Register `NUMRAMBLOCK` reader"]
pub struct R(crate::R<NUMRAMBLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NUMRAMBLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NUMRAMBLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NUMRAMBLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Number of individualy controllable RAM blocks.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [numramblock](index.html) module"]
pub struct NUMRAMBLOCK_SPEC;
impl crate::RegisterSpec for NUMRAMBLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [numramblock::R](R) reader structure"]
impl crate::Readable for NUMRAMBLOCK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NUMRAMBLOCK to value 0xffff_ffff"]
impl crate::Resettable for NUMRAMBLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
