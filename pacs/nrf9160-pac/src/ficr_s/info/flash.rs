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
#[doc = "Field `FLASH` reader - Flash variant"]
pub type FLASH_R = crate::FieldReader<u32, FLASH_A>;
#[doc = "Flash variant\n\nValue on reset: 1024"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FLASH_A {
    #[doc = "1024: 1 MByte FLASH"]
    K1024 = 1024,
}
impl From<FLASH_A> for u32 {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as _
    }
}
impl FLASH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_A> {
        match self.bits {
            1024 => Some(FLASH_A::K1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `K1024`"]
    #[inline(always)]
    pub fn is_k1024(&self) -> bool {
        *self == FLASH_A::K1024
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
#[doc = "`reset()` method sets FLASH to value 0x0400"]
impl crate::Resettable for FLASH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
