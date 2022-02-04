#[doc = "Register `FLASH` reader"]
pub struct R(crate::R<FLASH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Flash variant\n\nValue on reset: 192"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FLASH_A {
    #[doc = "192: 192 kByte flash"]
    K192 = 192,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<FLASH_A> for u32 {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH` reader - Flash variant"]
pub struct FLASH_R(crate::FieldReader<u32, FLASH_A>);
impl FLASH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FLASH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_A> {
        match self.bits {
            192 => Some(FLASH_A::K192),
            4294967295 => Some(FLASH_A::UNSPECIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `K192`"]
    #[inline(always)]
    pub fn is_k192(&self) -> bool {
        **self == FLASH_A::K192
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        **self == FLASH_A::UNSPECIFIED
    }
}
impl core::ops::Deref for FLASH_R {
    type Target = crate::FieldReader<u32, FLASH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Flash variant"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(self.bits)
    }
}
#[doc = "Flash variant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash](index.html) module"]
pub struct FLASH_SPEC;
impl crate::RegisterSpec for FLASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash::R](R) reader structure"]
impl crate::Readable for FLASH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLASH to value 0xc0"]
impl crate::Resettable for FLASH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
