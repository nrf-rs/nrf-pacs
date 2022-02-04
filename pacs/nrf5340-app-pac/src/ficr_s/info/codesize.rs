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
#[doc = "Code memory size in number of pages\n\nValue on reset: 256"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CODESIZE_A {
    #[doc = "256: 256 pages"]
    P256 = 256,
}
impl From<CODESIZE_A> for u32 {
    #[inline(always)]
    fn from(variant: CODESIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CODESIZE` reader - Code memory size in number of pages"]
pub struct CODESIZE_R(crate::FieldReader<u32, CODESIZE_A>);
impl CODESIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CODESIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CODESIZE_A> {
        match self.bits {
            256 => Some(CODESIZE_A::P256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P256`"]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        **self == CODESIZE_A::P256
    }
}
impl core::ops::Deref for CODESIZE_R {
    type Target = crate::FieldReader<u32, CODESIZE_A>;
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
#[doc = "`reset()` method sets CODESIZE to value 0x0100"]
impl crate::Resettable for CODESIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
