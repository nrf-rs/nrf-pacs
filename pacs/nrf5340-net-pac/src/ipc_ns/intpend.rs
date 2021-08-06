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
#[doc = "Read pending status of interrupt for event RECEIVE\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE8_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE8_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE8` reader - Read pending status of interrupt for event RECEIVE\\[8\\]"]
pub struct RECEIVE8_R(crate::FieldReader<bool, RECEIVE8_A>);
impl RECEIVE8_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE8_A {
        match self.bits {
            false => RECEIVE8_A::NOTPENDING,
            true => RECEIVE8_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE8_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE8_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE8_R {
    type Target = crate::FieldReader<bool, RECEIVE8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE9_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE9_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE9` reader - Read pending status of interrupt for event RECEIVE\\[9\\]"]
pub struct RECEIVE9_R(crate::FieldReader<bool, RECEIVE9_A>);
impl RECEIVE9_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE9_A {
        match self.bits {
            false => RECEIVE9_A::NOTPENDING,
            true => RECEIVE9_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE9_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE9_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE9_R {
    type Target = crate::FieldReader<bool, RECEIVE9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE10_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE10_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE10` reader - Read pending status of interrupt for event RECEIVE\\[10\\]"]
pub struct RECEIVE10_R(crate::FieldReader<bool, RECEIVE10_A>);
impl RECEIVE10_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE10_A {
        match self.bits {
            false => RECEIVE10_A::NOTPENDING,
            true => RECEIVE10_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE10_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE10_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE10_R {
    type Target = crate::FieldReader<bool, RECEIVE10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE11_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE11_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE11` reader - Read pending status of interrupt for event RECEIVE\\[11\\]"]
pub struct RECEIVE11_R(crate::FieldReader<bool, RECEIVE11_A>);
impl RECEIVE11_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE11_A {
        match self.bits {
            false => RECEIVE11_A::NOTPENDING,
            true => RECEIVE11_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE11_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE11_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE11_R {
    type Target = crate::FieldReader<bool, RECEIVE11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[12\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE12_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE12_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE12` reader - Read pending status of interrupt for event RECEIVE\\[12\\]"]
pub struct RECEIVE12_R(crate::FieldReader<bool, RECEIVE12_A>);
impl RECEIVE12_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE12_A {
        match self.bits {
            false => RECEIVE12_A::NOTPENDING,
            true => RECEIVE12_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE12_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE12_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE12_R {
    type Target = crate::FieldReader<bool, RECEIVE12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[13\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE13_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE13_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE13` reader - Read pending status of interrupt for event RECEIVE\\[13\\]"]
pub struct RECEIVE13_R(crate::FieldReader<bool, RECEIVE13_A>);
impl RECEIVE13_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE13_A {
        match self.bits {
            false => RECEIVE13_A::NOTPENDING,
            true => RECEIVE13_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE13_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE13_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE13_R {
    type Target = crate::FieldReader<bool, RECEIVE13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[14\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE14_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE14_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE14` reader - Read pending status of interrupt for event RECEIVE\\[14\\]"]
pub struct RECEIVE14_R(crate::FieldReader<bool, RECEIVE14_A>);
impl RECEIVE14_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE14_A {
        match self.bits {
            false => RECEIVE14_A::NOTPENDING,
            true => RECEIVE14_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE14_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE14_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE14_R {
    type Target = crate::FieldReader<bool, RECEIVE14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[15\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE15_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<RECEIVE15_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE15` reader - Read pending status of interrupt for event RECEIVE\\[15\\]"]
pub struct RECEIVE15_R(crate::FieldReader<bool, RECEIVE15_A>);
impl RECEIVE15_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE15_A {
        match self.bits {
            false => RECEIVE15_A::NOTPENDING,
            true => RECEIVE15_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RECEIVE15_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECEIVE15_A::PENDING
    }
}
impl core::ops::Deref for RECEIVE15_R {
    type Target = crate::FieldReader<bool, RECEIVE15_A>;
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
    #[doc = "Bit 8 - Read pending status of interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn receive8(&self) -> RECEIVE8_R {
        RECEIVE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read pending status of interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn receive9(&self) -> RECEIVE9_R {
        RECEIVE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Read pending status of interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn receive10(&self) -> RECEIVE10_R {
        RECEIVE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Read pending status of interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn receive11(&self) -> RECEIVE11_R {
        RECEIVE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Read pending status of interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn receive12(&self) -> RECEIVE12_R {
        RECEIVE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Read pending status of interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn receive13(&self) -> RECEIVE13_R {
        RECEIVE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Read pending status of interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn receive14(&self) -> RECEIVE14_R {
        RECEIVE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Read pending status of interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn receive15(&self) -> RECEIVE15_R {
        RECEIVE15_R::new(((self.bits >> 15) & 0x01) != 0)
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
