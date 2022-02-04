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
#[doc = "Read pending status of interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE0_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE0_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE0` reader - Read pending status of interrupt for event RECEIVE\\[0\\]"]
pub struct RECEIVE0_R(crate::FieldReader<bool, RECEIVE0_A>);
impl RECEIVE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE0_A {
        match self.bits {
            false => RECEIVE0_A::NOTPENDING,
            true => RECEIVE0_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE0_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE0_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE0_R {
    type Target = crate::FieldReader<bool, RECEIVE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE1_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE1_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE1` reader - Read pending status of interrupt for event RECEIVE\\[1\\]"]
pub struct RECEIVE1_R(crate::FieldReader<bool, RECEIVE1_A>);
impl RECEIVE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE1_A {
        match self.bits {
            false => RECEIVE1_A::NOTPENDING,
            true => RECEIVE1_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE1_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE1_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE1_R {
    type Target = crate::FieldReader<bool, RECEIVE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE2_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE2_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE2` reader - Read pending status of interrupt for event RECEIVE\\[2\\]"]
pub struct RECEIVE2_R(crate::FieldReader<bool, RECEIVE2_A>);
impl RECEIVE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE2_A {
        match self.bits {
            false => RECEIVE2_A::NOTPENDING,
            true => RECEIVE2_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE2_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE2_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE2_R {
    type Target = crate::FieldReader<bool, RECEIVE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE3_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE3_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE3` reader - Read pending status of interrupt for event RECEIVE\\[3\\]"]
pub struct RECEIVE3_R(crate::FieldReader<bool, RECEIVE3_A>);
impl RECEIVE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE3_A {
        match self.bits {
            false => RECEIVE3_A::NOTPENDING,
            true => RECEIVE3_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE3_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE3_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE3_R {
    type Target = crate::FieldReader<bool, RECEIVE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE4_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE4_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE4` reader - Read pending status of interrupt for event RECEIVE\\[4\\]"]
pub struct RECEIVE4_R(crate::FieldReader<bool, RECEIVE4_A>);
impl RECEIVE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE4_A {
        match self.bits {
            false => RECEIVE4_A::NOTPENDING,
            true => RECEIVE4_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE4_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE4_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE4_R {
    type Target = crate::FieldReader<bool, RECEIVE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE5_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE5_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE5` reader - Read pending status of interrupt for event RECEIVE\\[5\\]"]
pub struct RECEIVE5_R(crate::FieldReader<bool, RECEIVE5_A>);
impl RECEIVE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE5_A {
        match self.bits {
            false => RECEIVE5_A::NOTPENDING,
            true => RECEIVE5_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE5_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE5_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE5_R {
    type Target = crate::FieldReader<bool, RECEIVE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE6_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE6_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE6` reader - Read pending status of interrupt for event RECEIVE\\[6\\]"]
pub struct RECEIVE6_R(crate::FieldReader<bool, RECEIVE6_A>);
impl RECEIVE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE6_A {
        match self.bits {
            false => RECEIVE6_A::NOTPENDING,
            true => RECEIVE6_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE6_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE6_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE6_R {
    type Target = crate::FieldReader<bool, RECEIVE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE7_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE7_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE7` reader - Read pending status of interrupt for event RECEIVE\\[7\\]"]
pub struct RECEIVE7_R(crate::FieldReader<bool, RECEIVE7_A>);
impl RECEIVE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE7_A {
        match self.bits {
            false => RECEIVE7_A::NOTPENDING,
            true => RECEIVE7_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE7_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE7_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE7_R {
    type Target = crate::FieldReader<bool, RECEIVE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&self) -> RECEIVE0_R {
        RECEIVE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&self) -> RECEIVE1_R {
        RECEIVE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&self) -> RECEIVE2_R {
        RECEIVE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read pending status of interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&self) -> RECEIVE3_R {
        RECEIVE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read pending status of interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&self) -> RECEIVE4_R {
        RECEIVE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read pending status of interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&self) -> RECEIVE5_R {
        RECEIVE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read pending status of interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&self) -> RECEIVE6_R {
        RECEIVE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read pending status of interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&self) -> RECEIVE7_R {
        RECEIVE7_R::new(((self.bits >> 7) & 0x01) != 0)
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
