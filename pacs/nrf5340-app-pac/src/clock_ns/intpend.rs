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
#[doc = "Read pending status of interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTED_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<HFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKSTARTED` reader - Read pending status of interrupt for event HFCLKSTARTED"]
pub struct HFCLKSTARTED_R(crate::FieldReader<bool, HFCLKSTARTED_A>);
impl HFCLKSTARTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFCLKSTARTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKSTARTED_A {
        match self.bits {
            false => HFCLKSTARTED_A::NOTPENDING,
            true => HFCLKSTARTED_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == HFCLKSTARTED_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == HFCLKSTARTED_A::PENDING
    }
}
impl core::ops::Deref for HFCLKSTARTED_R {
    type Target = crate::FieldReader<bool, HFCLKSTARTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTED_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<LFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` reader - Read pending status of interrupt for event LFCLKSTARTED"]
pub struct LFCLKSTARTED_R(crate::FieldReader<bool, LFCLKSTARTED_A>);
impl LFCLKSTARTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFCLKSTARTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFCLKSTARTED_A {
        match self.bits {
            false => LFCLKSTARTED_A::NOTPENDING,
            true => LFCLKSTARTED_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == LFCLKSTARTED_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == LFCLKSTARTED_A::PENDING
    }
}
impl core::ops::Deref for LFCLKSTARTED_R {
    type Target = crate::FieldReader<bool, LFCLKSTARTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Read pending status of interrupt for event DONE"]
pub struct DONE_R(crate::FieldReader<bool, DONE_A>);
impl DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::NOTPENDING,
            true => DONE_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == DONE_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == DONE_A::PENDING
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event HFCLKAUDIOSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKAUDIOSTARTED_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<HFCLKAUDIOSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKAUDIOSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKAUDIOSTARTED` reader - Read pending status of interrupt for event HFCLKAUDIOSTARTED"]
pub struct HFCLKAUDIOSTARTED_R(crate::FieldReader<bool, HFCLKAUDIOSTARTED_A>);
impl HFCLKAUDIOSTARTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFCLKAUDIOSTARTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKAUDIOSTARTED_A {
        match self.bits {
            false => HFCLKAUDIOSTARTED_A::NOTPENDING,
            true => HFCLKAUDIOSTARTED_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == HFCLKAUDIOSTARTED_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == HFCLKAUDIOSTARTED_A::PENDING
    }
}
impl core::ops::Deref for HFCLKAUDIOSTARTED_R {
    type Target = crate::FieldReader<bool, HFCLKAUDIOSTARTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event HFCLK192MSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLK192MSTARTED_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<HFCLK192MSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLK192MSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLK192MSTARTED` reader - Read pending status of interrupt for event HFCLK192MSTARTED"]
pub struct HFCLK192MSTARTED_R(crate::FieldReader<bool, HFCLK192MSTARTED_A>);
impl HFCLK192MSTARTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFCLK192MSTARTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLK192MSTARTED_A {
        match self.bits {
            false => HFCLK192MSTARTED_A::NOTPENDING,
            true => HFCLK192MSTARTED_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == HFCLK192MSTARTED_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == HFCLK192MSTARTED_A::PENDING
    }
}
impl core::ops::Deref for HFCLK192MSTARTED_R {
    type Target = crate::FieldReader<bool, HFCLK192MSTARTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&self) -> HFCLKSTARTED_R {
        HFCLKSTARTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LFCLKSTARTED_R {
        LFCLKSTARTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read pending status of interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Read pending status of interrupt for event HFCLKAUDIOSTARTED"]
    #[inline(always)]
    pub fn hfclkaudiostarted(&self) -> HFCLKAUDIOSTARTED_R {
        HFCLKAUDIOSTARTED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read pending status of interrupt for event HFCLK192MSTARTED"]
    #[inline(always)]
    pub fn hfclk192mstarted(&self) -> HFCLK192MSTARTED_R {
        HFCLK192MSTARTED_R::new(((self.bits >> 9) & 0x01) != 0)
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
