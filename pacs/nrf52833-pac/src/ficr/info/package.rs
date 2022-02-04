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
#[doc = "Package option\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PACKAGE_A {
    #[doc = "8199: QDxx - 40-pin QFN"]
    QD = 8199,
    #[doc = "8196: QIxx - 73-pin aQFN"]
    QI = 8196,
    #[doc = "8200: CJxx - WLCSP"]
    CJ = 8200,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<PACKAGE_A> for u32 {
    #[inline(always)]
    fn from(variant: PACKAGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PACKAGE` reader - Package option"]
pub struct PACKAGE_R(crate::FieldReader<u32, PACKAGE_A>);
impl PACKAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PACKAGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PACKAGE_A> {
        match self.bits {
            8199 => Some(PACKAGE_A::QD),
            8196 => Some(PACKAGE_A::QI),
            8200 => Some(PACKAGE_A::CJ),
            4294967295 => Some(PACKAGE_A::UNSPECIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `QD`"]
    #[inline(always)]
    pub fn is_qd(&self) -> bool {
        **self == PACKAGE_A::QD
    }
    #[doc = "Checks if the value of the field is `QI`"]
    #[inline(always)]
    pub fn is_qi(&self) -> bool {
        **self == PACKAGE_A::QI
    }
    #[doc = "Checks if the value of the field is `CJ`"]
    #[inline(always)]
    pub fn is_cj(&self) -> bool {
        **self == PACKAGE_A::CJ
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        **self == PACKAGE_A::UNSPECIFIED
    }
}
impl core::ops::Deref for PACKAGE_R {
    type Target = crate::FieldReader<u32, PACKAGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "`reset()` method sets PACKAGE to value 0xffff_ffff"]
impl crate::Resettable for PACKAGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
