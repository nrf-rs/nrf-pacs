#[doc = "Register `CODEPAGESIZE` reader"]
pub struct R(crate::R<CODEPAGESIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODEPAGESIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODEPAGESIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODEPAGESIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Code memory page size in bytes\n\nValue on reset: 4096"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CODEPAGESIZE_A {
    #[doc = "4096: 4 kByte"]
    K4096 = 4096,
}
impl From<CODEPAGESIZE_A> for u32 {
    #[inline(always)]
    fn from(variant: CODEPAGESIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CODEPAGESIZE` reader - Code memory page size in bytes"]
pub struct CODEPAGESIZE_R(crate::FieldReader<u32, CODEPAGESIZE_A>);
impl CODEPAGESIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CODEPAGESIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CODEPAGESIZE_A> {
        match self.bits {
            4096 => Some(CODEPAGESIZE_A::K4096),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `K4096`"]
    #[inline(always)]
    pub fn is_k4096(&self) -> bool {
        **self == CODEPAGESIZE_A::K4096
    }
}
impl core::ops::Deref for CODEPAGESIZE_R {
    type Target = crate::FieldReader<u32, CODEPAGESIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Code memory page size in bytes"]
    #[inline(always)]
    pub fn codepagesize(&self) -> CODEPAGESIZE_R {
        CODEPAGESIZE_R::new(self.bits)
    }
}
#[doc = "Code memory page size in bytes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codepagesize](index.html) module"]
pub struct CODEPAGESIZE_SPEC;
impl crate::RegisterSpec for CODEPAGESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codepagesize::R](R) reader structure"]
impl crate::Readable for CODEPAGESIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CODEPAGESIZE to value 0x1000"]
impl crate::Resettable for CODEPAGESIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
