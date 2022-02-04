#[doc = "Register `PERM` reader"]
pub struct R(crate::R<PERM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERM` writer"]
pub struct W(crate::W<PERM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PERM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL0_A {
    #[doc = "1: Channel 0 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 0 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL0_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL0` reader - Select secure attribute"]
pub struct CHANNEL0_R(crate::FieldReader<bool, CHANNEL0_A>);
impl CHANNEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL0_A {
        match self.bits {
            true => CHANNEL0_A::SECURE,
            false => CHANNEL0_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL0_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL0_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL0_R {
    type Target = crate::FieldReader<bool, CHANNEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL0` writer - Select secure attribute"]
pub struct CHANNEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 0 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL0_A::SECURE)
    }
    #[doc = "Channel 0 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL0_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL1_A {
    #[doc = "1: Channel 1 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 1 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL1_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL1` reader - Select secure attribute"]
pub struct CHANNEL1_R(crate::FieldReader<bool, CHANNEL1_A>);
impl CHANNEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL1_A {
        match self.bits {
            true => CHANNEL1_A::SECURE,
            false => CHANNEL1_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL1_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL1_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL1_R {
    type Target = crate::FieldReader<bool, CHANNEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL1` writer - Select secure attribute"]
pub struct CHANNEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 1 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL1_A::SECURE)
    }
    #[doc = "Channel 1 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL1_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL2_A {
    #[doc = "1: Channel 2 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 2 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL2_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL2` reader - Select secure attribute"]
pub struct CHANNEL2_R(crate::FieldReader<bool, CHANNEL2_A>);
impl CHANNEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL2_A {
        match self.bits {
            true => CHANNEL2_A::SECURE,
            false => CHANNEL2_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL2_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL2_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL2_R {
    type Target = crate::FieldReader<bool, CHANNEL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL2` writer - Select secure attribute"]
pub struct CHANNEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 2 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL2_A::SECURE)
    }
    #[doc = "Channel 2 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL2_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL3_A {
    #[doc = "1: Channel 3 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 3 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL3_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL3` reader - Select secure attribute"]
pub struct CHANNEL3_R(crate::FieldReader<bool, CHANNEL3_A>);
impl CHANNEL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL3_A {
        match self.bits {
            true => CHANNEL3_A::SECURE,
            false => CHANNEL3_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL3_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL3_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL3_R {
    type Target = crate::FieldReader<bool, CHANNEL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL3` writer - Select secure attribute"]
pub struct CHANNEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 3 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL3_A::SECURE)
    }
    #[doc = "Channel 3 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL3_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL4_A {
    #[doc = "1: Channel 4 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 4 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL4_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL4` reader - Select secure attribute"]
pub struct CHANNEL4_R(crate::FieldReader<bool, CHANNEL4_A>);
impl CHANNEL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL4_A {
        match self.bits {
            true => CHANNEL4_A::SECURE,
            false => CHANNEL4_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL4_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL4_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL4_R {
    type Target = crate::FieldReader<bool, CHANNEL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL4` writer - Select secure attribute"]
pub struct CHANNEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 4 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL4_A::SECURE)
    }
    #[doc = "Channel 4 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL4_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL5_A {
    #[doc = "1: Channel 5 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 5 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL5_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL5` reader - Select secure attribute"]
pub struct CHANNEL5_R(crate::FieldReader<bool, CHANNEL5_A>);
impl CHANNEL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL5_A {
        match self.bits {
            true => CHANNEL5_A::SECURE,
            false => CHANNEL5_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL5_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL5_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL5_R {
    type Target = crate::FieldReader<bool, CHANNEL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL5` writer - Select secure attribute"]
pub struct CHANNEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 5 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL5_A::SECURE)
    }
    #[doc = "Channel 5 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL5_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL6_A {
    #[doc = "1: Channel 6 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 6 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL6_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL6` reader - Select secure attribute"]
pub struct CHANNEL6_R(crate::FieldReader<bool, CHANNEL6_A>);
impl CHANNEL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL6_A {
        match self.bits {
            true => CHANNEL6_A::SECURE,
            false => CHANNEL6_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL6_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL6_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL6_R {
    type Target = crate::FieldReader<bool, CHANNEL6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL6` writer - Select secure attribute"]
pub struct CHANNEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 6 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL6_A::SECURE)
    }
    #[doc = "Channel 6 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL6_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL7_A {
    #[doc = "1: Channel 7 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 7 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL7_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL7` reader - Select secure attribute"]
pub struct CHANNEL7_R(crate::FieldReader<bool, CHANNEL7_A>);
impl CHANNEL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL7_A {
        match self.bits {
            true => CHANNEL7_A::SECURE,
            false => CHANNEL7_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL7_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL7_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL7_R {
    type Target = crate::FieldReader<bool, CHANNEL7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL7` writer - Select secure attribute"]
pub struct CHANNEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 7 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL7_A::SECURE)
    }
    #[doc = "Channel 7 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL7_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL8_A {
    #[doc = "1: Channel 8 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 8 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL8_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL8` reader - Select secure attribute"]
pub struct CHANNEL8_R(crate::FieldReader<bool, CHANNEL8_A>);
impl CHANNEL8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL8_A {
        match self.bits {
            true => CHANNEL8_A::SECURE,
            false => CHANNEL8_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL8_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL8_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL8_R {
    type Target = crate::FieldReader<bool, CHANNEL8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL8` writer - Select secure attribute"]
pub struct CHANNEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 8 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL8_A::SECURE)
    }
    #[doc = "Channel 8 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL8_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL9_A {
    #[doc = "1: Channel 9 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 9 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL9_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL9` reader - Select secure attribute"]
pub struct CHANNEL9_R(crate::FieldReader<bool, CHANNEL9_A>);
impl CHANNEL9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL9_A {
        match self.bits {
            true => CHANNEL9_A::SECURE,
            false => CHANNEL9_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL9_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL9_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL9_R {
    type Target = crate::FieldReader<bool, CHANNEL9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL9` writer - Select secure attribute"]
pub struct CHANNEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 9 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL9_A::SECURE)
    }
    #[doc = "Channel 9 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL9_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL10_A {
    #[doc = "1: Channel 10 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 10 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL10_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL10` reader - Select secure attribute"]
pub struct CHANNEL10_R(crate::FieldReader<bool, CHANNEL10_A>);
impl CHANNEL10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL10_A {
        match self.bits {
            true => CHANNEL10_A::SECURE,
            false => CHANNEL10_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL10_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL10_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL10_R {
    type Target = crate::FieldReader<bool, CHANNEL10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL10` writer - Select secure attribute"]
pub struct CHANNEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 10 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL10_A::SECURE)
    }
    #[doc = "Channel 10 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL10_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL11_A {
    #[doc = "1: Channel 11 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 11 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL11_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL11` reader - Select secure attribute"]
pub struct CHANNEL11_R(crate::FieldReader<bool, CHANNEL11_A>);
impl CHANNEL11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL11_A {
        match self.bits {
            true => CHANNEL11_A::SECURE,
            false => CHANNEL11_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL11_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL11_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL11_R {
    type Target = crate::FieldReader<bool, CHANNEL11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL11` writer - Select secure attribute"]
pub struct CHANNEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 11 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL11_A::SECURE)
    }
    #[doc = "Channel 11 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL11_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL12_A {
    #[doc = "1: Channel 12 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 12 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL12_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL12` reader - Select secure attribute"]
pub struct CHANNEL12_R(crate::FieldReader<bool, CHANNEL12_A>);
impl CHANNEL12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL12_A {
        match self.bits {
            true => CHANNEL12_A::SECURE,
            false => CHANNEL12_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL12_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL12_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL12_R {
    type Target = crate::FieldReader<bool, CHANNEL12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL12` writer - Select secure attribute"]
pub struct CHANNEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 12 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL12_A::SECURE)
    }
    #[doc = "Channel 12 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL12_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL13_A {
    #[doc = "1: Channel 13 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 13 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL13_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL13` reader - Select secure attribute"]
pub struct CHANNEL13_R(crate::FieldReader<bool, CHANNEL13_A>);
impl CHANNEL13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL13_A {
        match self.bits {
            true => CHANNEL13_A::SECURE,
            false => CHANNEL13_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL13_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL13_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL13_R {
    type Target = crate::FieldReader<bool, CHANNEL13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL13` writer - Select secure attribute"]
pub struct CHANNEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 13 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL13_A::SECURE)
    }
    #[doc = "Channel 13 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL13_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL14_A {
    #[doc = "1: Channel 14 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 14 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL14_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL14` reader - Select secure attribute"]
pub struct CHANNEL14_R(crate::FieldReader<bool, CHANNEL14_A>);
impl CHANNEL14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL14_A {
        match self.bits {
            true => CHANNEL14_A::SECURE,
            false => CHANNEL14_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL14_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL14_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL14_R {
    type Target = crate::FieldReader<bool, CHANNEL14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL14` writer - Select secure attribute"]
pub struct CHANNEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 14 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL14_A::SECURE)
    }
    #[doc = "Channel 14 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL14_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL15_A {
    #[doc = "1: Channel 15 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 15 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL15_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL15` reader - Select secure attribute"]
pub struct CHANNEL15_R(crate::FieldReader<bool, CHANNEL15_A>);
impl CHANNEL15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL15_A {
        match self.bits {
            true => CHANNEL15_A::SECURE,
            false => CHANNEL15_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL15_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL15_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL15_R {
    type Target = crate::FieldReader<bool, CHANNEL15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL15` writer - Select secure attribute"]
pub struct CHANNEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 15 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL15_A::SECURE)
    }
    #[doc = "Channel 15 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL15_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL16_A {
    #[doc = "1: Channel 16 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 16 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL16_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL16` reader - Select secure attribute"]
pub struct CHANNEL16_R(crate::FieldReader<bool, CHANNEL16_A>);
impl CHANNEL16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL16_A {
        match self.bits {
            true => CHANNEL16_A::SECURE,
            false => CHANNEL16_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL16_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL16_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL16_R {
    type Target = crate::FieldReader<bool, CHANNEL16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL16` writer - Select secure attribute"]
pub struct CHANNEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 16 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL16_A::SECURE)
    }
    #[doc = "Channel 16 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL16_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL17_A {
    #[doc = "1: Channel 17 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 17 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL17_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL17` reader - Select secure attribute"]
pub struct CHANNEL17_R(crate::FieldReader<bool, CHANNEL17_A>);
impl CHANNEL17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL17_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL17_A {
        match self.bits {
            true => CHANNEL17_A::SECURE,
            false => CHANNEL17_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL17_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL17_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL17_R {
    type Target = crate::FieldReader<bool, CHANNEL17_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL17` writer - Select secure attribute"]
pub struct CHANNEL17_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 17 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL17_A::SECURE)
    }
    #[doc = "Channel 17 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL17_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL18_A {
    #[doc = "1: Channel 18 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 18 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL18_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL18` reader - Select secure attribute"]
pub struct CHANNEL18_R(crate::FieldReader<bool, CHANNEL18_A>);
impl CHANNEL18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL18_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL18_A {
        match self.bits {
            true => CHANNEL18_A::SECURE,
            false => CHANNEL18_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL18_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL18_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL18_R {
    type Target = crate::FieldReader<bool, CHANNEL18_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL18` writer - Select secure attribute"]
pub struct CHANNEL18_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 18 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL18_A::SECURE)
    }
    #[doc = "Channel 18 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL18_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL19_A {
    #[doc = "1: Channel 19 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 19 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL19_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL19` reader - Select secure attribute"]
pub struct CHANNEL19_R(crate::FieldReader<bool, CHANNEL19_A>);
impl CHANNEL19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL19_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL19_A {
        match self.bits {
            true => CHANNEL19_A::SECURE,
            false => CHANNEL19_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL19_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL19_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL19_R {
    type Target = crate::FieldReader<bool, CHANNEL19_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL19` writer - Select secure attribute"]
pub struct CHANNEL19_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 19 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL19_A::SECURE)
    }
    #[doc = "Channel 19 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL19_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL20_A {
    #[doc = "1: Channel 20 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 20 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL20_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL20` reader - Select secure attribute"]
pub struct CHANNEL20_R(crate::FieldReader<bool, CHANNEL20_A>);
impl CHANNEL20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL20_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL20_A {
        match self.bits {
            true => CHANNEL20_A::SECURE,
            false => CHANNEL20_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL20_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL20_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL20_R {
    type Target = crate::FieldReader<bool, CHANNEL20_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL20` writer - Select secure attribute"]
pub struct CHANNEL20_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 20 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL20_A::SECURE)
    }
    #[doc = "Channel 20 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL20_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL21_A {
    #[doc = "1: Channel 21 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 21 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL21_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL21` reader - Select secure attribute"]
pub struct CHANNEL21_R(crate::FieldReader<bool, CHANNEL21_A>);
impl CHANNEL21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL21_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL21_A {
        match self.bits {
            true => CHANNEL21_A::SECURE,
            false => CHANNEL21_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL21_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL21_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL21_R {
    type Target = crate::FieldReader<bool, CHANNEL21_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL21` writer - Select secure attribute"]
pub struct CHANNEL21_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 21 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL21_A::SECURE)
    }
    #[doc = "Channel 21 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL21_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL22_A {
    #[doc = "1: Channel 22 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 22 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL22_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL22` reader - Select secure attribute"]
pub struct CHANNEL22_R(crate::FieldReader<bool, CHANNEL22_A>);
impl CHANNEL22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL22_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL22_A {
        match self.bits {
            true => CHANNEL22_A::SECURE,
            false => CHANNEL22_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL22_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL22_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL22_R {
    type Target = crate::FieldReader<bool, CHANNEL22_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL22` writer - Select secure attribute"]
pub struct CHANNEL22_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 22 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL22_A::SECURE)
    }
    #[doc = "Channel 22 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL22_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL23_A {
    #[doc = "1: Channel 23 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 23 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL23_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL23` reader - Select secure attribute"]
pub struct CHANNEL23_R(crate::FieldReader<bool, CHANNEL23_A>);
impl CHANNEL23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL23_A {
        match self.bits {
            true => CHANNEL23_A::SECURE,
            false => CHANNEL23_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL23_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL23_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL23_R {
    type Target = crate::FieldReader<bool, CHANNEL23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL23` writer - Select secure attribute"]
pub struct CHANNEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 23 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL23_A::SECURE)
    }
    #[doc = "Channel 23 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL23_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL24_A {
    #[doc = "1: Channel 24 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 24 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL24_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL24` reader - Select secure attribute"]
pub struct CHANNEL24_R(crate::FieldReader<bool, CHANNEL24_A>);
impl CHANNEL24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL24_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL24_A {
        match self.bits {
            true => CHANNEL24_A::SECURE,
            false => CHANNEL24_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL24_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL24_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL24_R {
    type Target = crate::FieldReader<bool, CHANNEL24_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL24` writer - Select secure attribute"]
pub struct CHANNEL24_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 24 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL24_A::SECURE)
    }
    #[doc = "Channel 24 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL24_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL25_A {
    #[doc = "1: Channel 25 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 25 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL25_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL25` reader - Select secure attribute"]
pub struct CHANNEL25_R(crate::FieldReader<bool, CHANNEL25_A>);
impl CHANNEL25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL25_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL25_A {
        match self.bits {
            true => CHANNEL25_A::SECURE,
            false => CHANNEL25_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL25_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL25_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL25_R {
    type Target = crate::FieldReader<bool, CHANNEL25_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL25` writer - Select secure attribute"]
pub struct CHANNEL25_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL25_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 25 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL25_A::SECURE)
    }
    #[doc = "Channel 25 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL25_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL26_A {
    #[doc = "1: Channel 26 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 26 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL26_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL26` reader - Select secure attribute"]
pub struct CHANNEL26_R(crate::FieldReader<bool, CHANNEL26_A>);
impl CHANNEL26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL26_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL26_A {
        match self.bits {
            true => CHANNEL26_A::SECURE,
            false => CHANNEL26_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL26_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL26_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL26_R {
    type Target = crate::FieldReader<bool, CHANNEL26_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL26` writer - Select secure attribute"]
pub struct CHANNEL26_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL26_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 26 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL26_A::SECURE)
    }
    #[doc = "Channel 26 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL26_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL27_A {
    #[doc = "1: Channel 27 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 27 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL27_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL27` reader - Select secure attribute"]
pub struct CHANNEL27_R(crate::FieldReader<bool, CHANNEL27_A>);
impl CHANNEL27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL27_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL27_A {
        match self.bits {
            true => CHANNEL27_A::SECURE,
            false => CHANNEL27_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL27_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL27_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL27_R {
    type Target = crate::FieldReader<bool, CHANNEL27_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL27` writer - Select secure attribute"]
pub struct CHANNEL27_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL27_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 27 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL27_A::SECURE)
    }
    #[doc = "Channel 27 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL27_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL28_A {
    #[doc = "1: Channel 28 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 28 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL28_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL28` reader - Select secure attribute"]
pub struct CHANNEL28_R(crate::FieldReader<bool, CHANNEL28_A>);
impl CHANNEL28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL28_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL28_A {
        match self.bits {
            true => CHANNEL28_A::SECURE,
            false => CHANNEL28_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL28_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL28_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL28_R {
    type Target = crate::FieldReader<bool, CHANNEL28_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL28` writer - Select secure attribute"]
pub struct CHANNEL28_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL28_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 28 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL28_A::SECURE)
    }
    #[doc = "Channel 28 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL28_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL29_A {
    #[doc = "1: Channel 29 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 29 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL29_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL29` reader - Select secure attribute"]
pub struct CHANNEL29_R(crate::FieldReader<bool, CHANNEL29_A>);
impl CHANNEL29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL29_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL29_A {
        match self.bits {
            true => CHANNEL29_A::SECURE,
            false => CHANNEL29_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL29_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL29_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL29_R {
    type Target = crate::FieldReader<bool, CHANNEL29_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL29` writer - Select secure attribute"]
pub struct CHANNEL29_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL29_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 29 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL29_A::SECURE)
    }
    #[doc = "Channel 29 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL29_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL30_A {
    #[doc = "1: Channel 30 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 30 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL30_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL30` reader - Select secure attribute"]
pub struct CHANNEL30_R(crate::FieldReader<bool, CHANNEL30_A>);
impl CHANNEL30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL30_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL30_A {
        match self.bits {
            true => CHANNEL30_A::SECURE,
            false => CHANNEL30_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL30_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL30_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL30_R {
    type Target = crate::FieldReader<bool, CHANNEL30_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL30` writer - Select secure attribute"]
pub struct CHANNEL30_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL30_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 30 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL30_A::SECURE)
    }
    #[doc = "Channel 30 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL30_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL31_A {
    #[doc = "1: Channel 31 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel 31 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL31_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL31` reader - Select secure attribute"]
pub struct CHANNEL31_R(crate::FieldReader<bool, CHANNEL31_A>);
impl CHANNEL31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANNEL31_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHANNEL31_A {
        match self.bits {
            true => CHANNEL31_A::SECURE,
            false => CHANNEL31_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == CHANNEL31_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == CHANNEL31_A::NONSECURE
    }
}
impl core::ops::Deref for CHANNEL31_R {
    type Target = crate::FieldReader<bool, CHANNEL31_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL31` writer - Select secure attribute"]
pub struct CHANNEL31_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL31_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel 31 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL31_A::SECURE)
    }
    #[doc = "Channel 31 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL31_A::NONSECURE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select secure attribute"]
    #[inline(always)]
    pub fn channel0(&self) -> CHANNEL0_R {
        CHANNEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select secure attribute"]
    #[inline(always)]
    pub fn channel1(&self) -> CHANNEL1_R {
        CHANNEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select secure attribute"]
    #[inline(always)]
    pub fn channel2(&self) -> CHANNEL2_R {
        CHANNEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select secure attribute"]
    #[inline(always)]
    pub fn channel3(&self) -> CHANNEL3_R {
        CHANNEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Select secure attribute"]
    #[inline(always)]
    pub fn channel4(&self) -> CHANNEL4_R {
        CHANNEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Select secure attribute"]
    #[inline(always)]
    pub fn channel5(&self) -> CHANNEL5_R {
        CHANNEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Select secure attribute"]
    #[inline(always)]
    pub fn channel6(&self) -> CHANNEL6_R {
        CHANNEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Select secure attribute"]
    #[inline(always)]
    pub fn channel7(&self) -> CHANNEL7_R {
        CHANNEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Select secure attribute"]
    #[inline(always)]
    pub fn channel8(&self) -> CHANNEL8_R {
        CHANNEL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Select secure attribute"]
    #[inline(always)]
    pub fn channel9(&self) -> CHANNEL9_R {
        CHANNEL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Select secure attribute"]
    #[inline(always)]
    pub fn channel10(&self) -> CHANNEL10_R {
        CHANNEL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Select secure attribute"]
    #[inline(always)]
    pub fn channel11(&self) -> CHANNEL11_R {
        CHANNEL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Select secure attribute"]
    #[inline(always)]
    pub fn channel12(&self) -> CHANNEL12_R {
        CHANNEL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Select secure attribute"]
    #[inline(always)]
    pub fn channel13(&self) -> CHANNEL13_R {
        CHANNEL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Select secure attribute"]
    #[inline(always)]
    pub fn channel14(&self) -> CHANNEL14_R {
        CHANNEL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Select secure attribute"]
    #[inline(always)]
    pub fn channel15(&self) -> CHANNEL15_R {
        CHANNEL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Select secure attribute"]
    #[inline(always)]
    pub fn channel16(&self) -> CHANNEL16_R {
        CHANNEL16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Select secure attribute"]
    #[inline(always)]
    pub fn channel17(&self) -> CHANNEL17_R {
        CHANNEL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Select secure attribute"]
    #[inline(always)]
    pub fn channel18(&self) -> CHANNEL18_R {
        CHANNEL18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Select secure attribute"]
    #[inline(always)]
    pub fn channel19(&self) -> CHANNEL19_R {
        CHANNEL19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Select secure attribute"]
    #[inline(always)]
    pub fn channel20(&self) -> CHANNEL20_R {
        CHANNEL20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Select secure attribute"]
    #[inline(always)]
    pub fn channel21(&self) -> CHANNEL21_R {
        CHANNEL21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Select secure attribute"]
    #[inline(always)]
    pub fn channel22(&self) -> CHANNEL22_R {
        CHANNEL22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Select secure attribute"]
    #[inline(always)]
    pub fn channel23(&self) -> CHANNEL23_R {
        CHANNEL23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Select secure attribute"]
    #[inline(always)]
    pub fn channel24(&self) -> CHANNEL24_R {
        CHANNEL24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Select secure attribute"]
    #[inline(always)]
    pub fn channel25(&self) -> CHANNEL25_R {
        CHANNEL25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Select secure attribute"]
    #[inline(always)]
    pub fn channel26(&self) -> CHANNEL26_R {
        CHANNEL26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Select secure attribute"]
    #[inline(always)]
    pub fn channel27(&self) -> CHANNEL27_R {
        CHANNEL27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Select secure attribute"]
    #[inline(always)]
    pub fn channel28(&self) -> CHANNEL28_R {
        CHANNEL28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Select secure attribute"]
    #[inline(always)]
    pub fn channel29(&self) -> CHANNEL29_R {
        CHANNEL29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Select secure attribute"]
    #[inline(always)]
    pub fn channel30(&self) -> CHANNEL30_R {
        CHANNEL30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Select secure attribute"]
    #[inline(always)]
    pub fn channel31(&self) -> CHANNEL31_R {
        CHANNEL31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select secure attribute"]
    #[inline(always)]
    pub fn channel0(&mut self) -> CHANNEL0_W {
        CHANNEL0_W { w: self }
    }
    #[doc = "Bit 1 - Select secure attribute"]
    #[inline(always)]
    pub fn channel1(&mut self) -> CHANNEL1_W {
        CHANNEL1_W { w: self }
    }
    #[doc = "Bit 2 - Select secure attribute"]
    #[inline(always)]
    pub fn channel2(&mut self) -> CHANNEL2_W {
        CHANNEL2_W { w: self }
    }
    #[doc = "Bit 3 - Select secure attribute"]
    #[inline(always)]
    pub fn channel3(&mut self) -> CHANNEL3_W {
        CHANNEL3_W { w: self }
    }
    #[doc = "Bit 4 - Select secure attribute"]
    #[inline(always)]
    pub fn channel4(&mut self) -> CHANNEL4_W {
        CHANNEL4_W { w: self }
    }
    #[doc = "Bit 5 - Select secure attribute"]
    #[inline(always)]
    pub fn channel5(&mut self) -> CHANNEL5_W {
        CHANNEL5_W { w: self }
    }
    #[doc = "Bit 6 - Select secure attribute"]
    #[inline(always)]
    pub fn channel6(&mut self) -> CHANNEL6_W {
        CHANNEL6_W { w: self }
    }
    #[doc = "Bit 7 - Select secure attribute"]
    #[inline(always)]
    pub fn channel7(&mut self) -> CHANNEL7_W {
        CHANNEL7_W { w: self }
    }
    #[doc = "Bit 8 - Select secure attribute"]
    #[inline(always)]
    pub fn channel8(&mut self) -> CHANNEL8_W {
        CHANNEL8_W { w: self }
    }
    #[doc = "Bit 9 - Select secure attribute"]
    #[inline(always)]
    pub fn channel9(&mut self) -> CHANNEL9_W {
        CHANNEL9_W { w: self }
    }
    #[doc = "Bit 10 - Select secure attribute"]
    #[inline(always)]
    pub fn channel10(&mut self) -> CHANNEL10_W {
        CHANNEL10_W { w: self }
    }
    #[doc = "Bit 11 - Select secure attribute"]
    #[inline(always)]
    pub fn channel11(&mut self) -> CHANNEL11_W {
        CHANNEL11_W { w: self }
    }
    #[doc = "Bit 12 - Select secure attribute"]
    #[inline(always)]
    pub fn channel12(&mut self) -> CHANNEL12_W {
        CHANNEL12_W { w: self }
    }
    #[doc = "Bit 13 - Select secure attribute"]
    #[inline(always)]
    pub fn channel13(&mut self) -> CHANNEL13_W {
        CHANNEL13_W { w: self }
    }
    #[doc = "Bit 14 - Select secure attribute"]
    #[inline(always)]
    pub fn channel14(&mut self) -> CHANNEL14_W {
        CHANNEL14_W { w: self }
    }
    #[doc = "Bit 15 - Select secure attribute"]
    #[inline(always)]
    pub fn channel15(&mut self) -> CHANNEL15_W {
        CHANNEL15_W { w: self }
    }
    #[doc = "Bit 16 - Select secure attribute"]
    #[inline(always)]
    pub fn channel16(&mut self) -> CHANNEL16_W {
        CHANNEL16_W { w: self }
    }
    #[doc = "Bit 17 - Select secure attribute"]
    #[inline(always)]
    pub fn channel17(&mut self) -> CHANNEL17_W {
        CHANNEL17_W { w: self }
    }
    #[doc = "Bit 18 - Select secure attribute"]
    #[inline(always)]
    pub fn channel18(&mut self) -> CHANNEL18_W {
        CHANNEL18_W { w: self }
    }
    #[doc = "Bit 19 - Select secure attribute"]
    #[inline(always)]
    pub fn channel19(&mut self) -> CHANNEL19_W {
        CHANNEL19_W { w: self }
    }
    #[doc = "Bit 20 - Select secure attribute"]
    #[inline(always)]
    pub fn channel20(&mut self) -> CHANNEL20_W {
        CHANNEL20_W { w: self }
    }
    #[doc = "Bit 21 - Select secure attribute"]
    #[inline(always)]
    pub fn channel21(&mut self) -> CHANNEL21_W {
        CHANNEL21_W { w: self }
    }
    #[doc = "Bit 22 - Select secure attribute"]
    #[inline(always)]
    pub fn channel22(&mut self) -> CHANNEL22_W {
        CHANNEL22_W { w: self }
    }
    #[doc = "Bit 23 - Select secure attribute"]
    #[inline(always)]
    pub fn channel23(&mut self) -> CHANNEL23_W {
        CHANNEL23_W { w: self }
    }
    #[doc = "Bit 24 - Select secure attribute"]
    #[inline(always)]
    pub fn channel24(&mut self) -> CHANNEL24_W {
        CHANNEL24_W { w: self }
    }
    #[doc = "Bit 25 - Select secure attribute"]
    #[inline(always)]
    pub fn channel25(&mut self) -> CHANNEL25_W {
        CHANNEL25_W { w: self }
    }
    #[doc = "Bit 26 - Select secure attribute"]
    #[inline(always)]
    pub fn channel26(&mut self) -> CHANNEL26_W {
        CHANNEL26_W { w: self }
    }
    #[doc = "Bit 27 - Select secure attribute"]
    #[inline(always)]
    pub fn channel27(&mut self) -> CHANNEL27_W {
        CHANNEL27_W { w: self }
    }
    #[doc = "Bit 28 - Select secure attribute"]
    #[inline(always)]
    pub fn channel28(&mut self) -> CHANNEL28_W {
        CHANNEL28_W { w: self }
    }
    #[doc = "Bit 29 - Select secure attribute"]
    #[inline(always)]
    pub fn channel29(&mut self) -> CHANNEL29_W {
        CHANNEL29_W { w: self }
    }
    #[doc = "Bit 30 - Select secure attribute"]
    #[inline(always)]
    pub fn channel30(&mut self) -> CHANNEL30_W {
        CHANNEL30_W { w: self }
    }
    #[doc = "Bit 31 - Select secure attribute"]
    #[inline(always)]
    pub fn channel31(&mut self) -> CHANNEL31_W {
        CHANNEL31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
pub struct PERM_SPEC;
impl crate::RegisterSpec for PERM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perm::R](R) reader structure"]
impl crate::Readable for PERM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perm::W](W) writer structure"]
impl crate::Writable for PERM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERM to value 0xffff_ffff"]
impl crate::Resettable for PERM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
