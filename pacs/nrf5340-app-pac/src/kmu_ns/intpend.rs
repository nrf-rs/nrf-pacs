#[doc = "Register `INTPEND` reader"]
pub struct R(crate::R<INTPEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Read pending status of interrupt for event KEYSLOT_PUSHED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_PUSHED_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<KEYSLOT_PUSHED_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_PUSHED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_PUSHED` reader - Read pending status of interrupt for event KEYSLOT_PUSHED"]
pub struct KEYSLOT_PUSHED_R(crate::FieldReader<bool, KEYSLOT_PUSHED_A>);
impl KEYSLOT_PUSHED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEYSLOT_PUSHED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_PUSHED_A {
        match self.bits {
            false => KEYSLOT_PUSHED_A::NOTPENDING,
            true => KEYSLOT_PUSHED_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == KEYSLOT_PUSHED_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == KEYSLOT_PUSHED_A::PENDING
    }
}
impl core::ops::Deref for KEYSLOT_PUSHED_R {
    type Target = crate::FieldReader<bool, KEYSLOT_PUSHED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event KEYSLOT_REVOKED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_REVOKED_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<KEYSLOT_REVOKED_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_REVOKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_REVOKED` reader - Read pending status of interrupt for event KEYSLOT_REVOKED"]
pub struct KEYSLOT_REVOKED_R(crate::FieldReader<bool, KEYSLOT_REVOKED_A>);
impl KEYSLOT_REVOKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEYSLOT_REVOKED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_REVOKED_A {
        match self.bits {
            false => KEYSLOT_REVOKED_A::NOTPENDING,
            true => KEYSLOT_REVOKED_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == KEYSLOT_REVOKED_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == KEYSLOT_REVOKED_A::PENDING
    }
}
impl core::ops::Deref for KEYSLOT_REVOKED_R {
    type Target = crate::FieldReader<bool, KEYSLOT_REVOKED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event KEYSLOT_ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_ERROR_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<KEYSLOT_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYSLOT_ERROR` reader - Read pending status of interrupt for event KEYSLOT_ERROR"]
pub struct KEYSLOT_ERROR_R(crate::FieldReader<bool, KEYSLOT_ERROR_A>);
impl KEYSLOT_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEYSLOT_ERROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_ERROR_A {
        match self.bits {
            false => KEYSLOT_ERROR_A::NOTPENDING,
            true => KEYSLOT_ERROR_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == KEYSLOT_ERROR_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == KEYSLOT_ERROR_A::PENDING
    }
}
impl core::ops::Deref for KEYSLOT_ERROR_R {
    type Target = crate::FieldReader<bool, KEYSLOT_ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn keyslot_pushed(&self) -> KEYSLOT_PUSHED_R {
        KEYSLOT_PUSHED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn keyslot_revoked(&self) -> KEYSLOT_REVOKED_R {
        KEYSLOT_REVOKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn keyslot_error(&self) -> KEYSLOT_ERROR_R {
        KEYSLOT_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Pending interrupts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpend](index.html) module"]
pub struct INTPEND_SPEC;
impl crate::RegisterSpec for INTPEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intpend::R](R) reader structure"]
impl crate::Readable for INTPEND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for INTPEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
