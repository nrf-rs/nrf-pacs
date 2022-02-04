#[doc = "Register `READYNEXT` reader"]
pub struct R(crate::R<READYNEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READYNEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READYNEXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READYNEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "NVMC can accept a new write operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READYNEXT_A {
    #[doc = "0: NVMC cannot accept any write operation"]
    BUSY = 0,
    #[doc = "1: NVMC is ready"]
    READY = 1,
}
impl From<READYNEXT_A> for bool {
    #[inline(always)]
    fn from(variant: READYNEXT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READYNEXT` reader - NVMC can accept a new write operation"]
pub struct READYNEXT_R(crate::FieldReader<bool, READYNEXT_A>);
impl READYNEXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READYNEXT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READYNEXT_A {
        match self.bits {
            false => READYNEXT_A::BUSY,
            true => READYNEXT_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == READYNEXT_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == READYNEXT_A::READY
    }
}
impl core::ops::Deref for READYNEXT_R {
    type Target = crate::FieldReader<bool, READYNEXT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - NVMC can accept a new write operation"]
    #[inline(always)]
    pub fn readynext(&self) -> READYNEXT_R {
        READYNEXT_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Ready flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readynext](index.html) module"]
pub struct READYNEXT_SPEC;
impl crate::RegisterSpec for READYNEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readynext::R](R) reader structure"]
impl crate::Readable for READYNEXT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets READYNEXT to value 0x01"]
impl crate::Resettable for READYNEXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
