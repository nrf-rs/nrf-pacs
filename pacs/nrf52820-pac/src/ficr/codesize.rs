#[doc = "Register `CODESIZE` reader"]
pub struct R(crate::R<CODESIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODESIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODESIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODESIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CODESIZE` reader - Code memory size in number of pages"]
pub struct CODESIZE_R(crate::FieldReader<u32, u32>);
impl CODESIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CODESIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CODESIZE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Code memory size in number of pages"]
    #[inline(always)]
    pub fn codesize(&self) -> CODESIZE_R {
        CODESIZE_R::new(self.bits)
    }
}
#[doc = "Code memory size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codesize](index.html) module"]
pub struct CODESIZE_SPEC;
impl crate::RegisterSpec for CODESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codesize::R](R) reader structure"]
impl crate::Readable for CODESIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CODESIZE to value 0xffff_ffff"]
impl crate::Resettable for CODESIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
