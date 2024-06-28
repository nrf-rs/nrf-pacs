#[doc = "Register `PACKAGE` reader"]
pub struct R(crate::R<PACKAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKAGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PACKAGE` reader - Package option"]
pub type PACKAGE_R = crate::FieldReader<u32, PACKAGE_A>;
#[doc = "Package option\n\nValue on reset: 8192"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PACKAGE_A {
    #[doc = "8194: CFxx - 236 ball wlCSP"]
    CF = 8194,
}
impl From<PACKAGE_A> for u32 {
    #[inline(always)]
    fn from(variant: PACKAGE_A) -> Self {
        variant as _
    }
}
impl PACKAGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PACKAGE_A> {
        match self.bits {
            8194 => Some(PACKAGE_A::CF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CF`"]
    #[inline(always)]
    pub fn is_cf(&self) -> bool {
        *self == PACKAGE_A::CF
    }
}
impl R {
    #[doc = "Bits 0:31 - Package option"]
    #[inline(always)]
    pub fn package(&self) -> PACKAGE_R {
        PACKAGE_R::new(self.bits)
    }
}
#[doc = "Package option\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [package](index.html) module"]
pub struct PACKAGE_SPEC;
impl crate::RegisterSpec for PACKAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [package::R](R) reader structure"]
impl crate::Readable for PACKAGE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACKAGE to value 0x2000"]
impl crate::Resettable for PACKAGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
