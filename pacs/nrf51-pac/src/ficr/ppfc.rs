#[doc = "Register `PPFC` reader"]
pub struct R(crate::R<PPFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Pre-programmed factory code present.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PPFC_A {
    #[doc = "255: Not present."]
    NOTPRESENT = 255,
    #[doc = "0: Present."]
    PRESENT = 0,
}
impl From<PPFC_A> for u8 {
    #[inline(always)]
    fn from(variant: PPFC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PPFC` reader - Pre-programmed factory code present."]
pub struct PPFC_R(crate::FieldReader<u8, PPFC_A>);
impl PPFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PPFC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PPFC_A> {
        match self.bits {
            255 => Some(PPFC_A::NOTPRESENT),
            0 => Some(PPFC_A::PRESENT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        **self == PPFC_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == PPFC_A::PRESENT
    }
}
impl core::ops::Deref for PPFC_R {
    type Target = crate::FieldReader<u8, PPFC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Pre-programmed factory code present."]
    #[inline(always)]
    pub fn ppfc(&self) -> PPFC_R {
        PPFC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Pre-programmed factory code present.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppfc](index.html) module"]
pub struct PPFC_SPEC;
impl crate::RegisterSpec for PPFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppfc::R](R) reader structure"]
impl crate::Readable for PPFC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PPFC to value 0xffff_ffff"]
impl crate::Resettable for PPFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
