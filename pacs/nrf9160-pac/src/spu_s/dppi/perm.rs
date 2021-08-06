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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL0_A {
    #[doc = "1: Channel0 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel0 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL0_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL0` reader - Select secure attribute."]
pub struct CHANNEL0_R(crate::FieldReader<bool, CHANNEL0_A>);
impl CHANNEL0_R {
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
#[doc = "Field `CHANNEL0` writer - Select secure attribute."]
pub struct CHANNEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel0 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL0_A::SECURE)
    }
    #[doc = "Channel0 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL1_A {
    #[doc = "1: Channel1 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel1 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL1_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL1` reader - Select secure attribute."]
pub struct CHANNEL1_R(crate::FieldReader<bool, CHANNEL1_A>);
impl CHANNEL1_R {
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
#[doc = "Field `CHANNEL1` writer - Select secure attribute."]
pub struct CHANNEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel1 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL1_A::SECURE)
    }
    #[doc = "Channel1 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL2_A {
    #[doc = "1: Channel2 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel2 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL2_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL2` reader - Select secure attribute."]
pub struct CHANNEL2_R(crate::FieldReader<bool, CHANNEL2_A>);
impl CHANNEL2_R {
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
#[doc = "Field `CHANNEL2` writer - Select secure attribute."]
pub struct CHANNEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel2 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL2_A::SECURE)
    }
    #[doc = "Channel2 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL3_A {
    #[doc = "1: Channel3 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel3 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL3_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL3` reader - Select secure attribute."]
pub struct CHANNEL3_R(crate::FieldReader<bool, CHANNEL3_A>);
impl CHANNEL3_R {
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
#[doc = "Field `CHANNEL3` writer - Select secure attribute."]
pub struct CHANNEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel3 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL3_A::SECURE)
    }
    #[doc = "Channel3 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL4_A {
    #[doc = "1: Channel4 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel4 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL4_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL4` reader - Select secure attribute."]
pub struct CHANNEL4_R(crate::FieldReader<bool, CHANNEL4_A>);
impl CHANNEL4_R {
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
#[doc = "Field `CHANNEL4` writer - Select secure attribute."]
pub struct CHANNEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel4 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL4_A::SECURE)
    }
    #[doc = "Channel4 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL5_A {
    #[doc = "1: Channel5 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel5 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL5_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL5` reader - Select secure attribute."]
pub struct CHANNEL5_R(crate::FieldReader<bool, CHANNEL5_A>);
impl CHANNEL5_R {
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
#[doc = "Field `CHANNEL5` writer - Select secure attribute."]
pub struct CHANNEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel5 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL5_A::SECURE)
    }
    #[doc = "Channel5 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL6_A {
    #[doc = "1: Channel6 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel6 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL6_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL6` reader - Select secure attribute."]
pub struct CHANNEL6_R(crate::FieldReader<bool, CHANNEL6_A>);
impl CHANNEL6_R {
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
#[doc = "Field `CHANNEL6` writer - Select secure attribute."]
pub struct CHANNEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel6 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL6_A::SECURE)
    }
    #[doc = "Channel6 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL7_A {
    #[doc = "1: Channel7 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel7 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL7_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL7` reader - Select secure attribute."]
pub struct CHANNEL7_R(crate::FieldReader<bool, CHANNEL7_A>);
impl CHANNEL7_R {
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
#[doc = "Field `CHANNEL7` writer - Select secure attribute."]
pub struct CHANNEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel7 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL7_A::SECURE)
    }
    #[doc = "Channel7 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL8_A {
    #[doc = "1: Channel8 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel8 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL8_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL8` reader - Select secure attribute."]
pub struct CHANNEL8_R(crate::FieldReader<bool, CHANNEL8_A>);
impl CHANNEL8_R {
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
#[doc = "Field `CHANNEL8` writer - Select secure attribute."]
pub struct CHANNEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel8 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL8_A::SECURE)
    }
    #[doc = "Channel8 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL9_A {
    #[doc = "1: Channel9 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel9 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL9_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL9` reader - Select secure attribute."]
pub struct CHANNEL9_R(crate::FieldReader<bool, CHANNEL9_A>);
impl CHANNEL9_R {
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
#[doc = "Field `CHANNEL9` writer - Select secure attribute."]
pub struct CHANNEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel9 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL9_A::SECURE)
    }
    #[doc = "Channel9 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL10_A {
    #[doc = "1: Channel10 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel10 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL10_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL10` reader - Select secure attribute."]
pub struct CHANNEL10_R(crate::FieldReader<bool, CHANNEL10_A>);
impl CHANNEL10_R {
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
#[doc = "Field `CHANNEL10` writer - Select secure attribute."]
pub struct CHANNEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel10 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL10_A::SECURE)
    }
    #[doc = "Channel10 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL11_A {
    #[doc = "1: Channel11 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel11 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL11_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL11` reader - Select secure attribute."]
pub struct CHANNEL11_R(crate::FieldReader<bool, CHANNEL11_A>);
impl CHANNEL11_R {
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
#[doc = "Field `CHANNEL11` writer - Select secure attribute."]
pub struct CHANNEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel11 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL11_A::SECURE)
    }
    #[doc = "Channel11 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL12_A {
    #[doc = "1: Channel12 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel12 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL12_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL12` reader - Select secure attribute."]
pub struct CHANNEL12_R(crate::FieldReader<bool, CHANNEL12_A>);
impl CHANNEL12_R {
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
#[doc = "Field `CHANNEL12` writer - Select secure attribute."]
pub struct CHANNEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel12 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL12_A::SECURE)
    }
    #[doc = "Channel12 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL13_A {
    #[doc = "1: Channel13 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel13 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL13_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL13` reader - Select secure attribute."]
pub struct CHANNEL13_R(crate::FieldReader<bool, CHANNEL13_A>);
impl CHANNEL13_R {
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
#[doc = "Field `CHANNEL13` writer - Select secure attribute."]
pub struct CHANNEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel13 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL13_A::SECURE)
    }
    #[doc = "Channel13 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL14_A {
    #[doc = "1: Channel14 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel14 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL14_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL14` reader - Select secure attribute."]
pub struct CHANNEL14_R(crate::FieldReader<bool, CHANNEL14_A>);
impl CHANNEL14_R {
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
#[doc = "Field `CHANNEL14` writer - Select secure attribute."]
pub struct CHANNEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel14 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL14_A::SECURE)
    }
    #[doc = "Channel14 has its non-secure attribute set"]
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
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL15_A {
    #[doc = "1: Channel15 has its secure attribute set"]
    SECURE = 1,
    #[doc = "0: Channel15 has its non-secure attribute set"]
    NONSECURE = 0,
}
impl From<CHANNEL15_A> for bool {
    #[inline(always)]
    fn from(variant: CHANNEL15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL15` reader - Select secure attribute."]
pub struct CHANNEL15_R(crate::FieldReader<bool, CHANNEL15_A>);
impl CHANNEL15_R {
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
#[doc = "Field `CHANNEL15` writer - Select secure attribute."]
pub struct CHANNEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel15 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL15_A::SECURE)
    }
    #[doc = "Channel15 has its non-secure attribute set"]
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
impl R {
    #[doc = "Bit 0 - Select secure attribute."]
    #[inline(always)]
    pub fn channel0(&self) -> CHANNEL0_R {
        CHANNEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select secure attribute."]
    #[inline(always)]
    pub fn channel1(&self) -> CHANNEL1_R {
        CHANNEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select secure attribute."]
    #[inline(always)]
    pub fn channel2(&self) -> CHANNEL2_R {
        CHANNEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select secure attribute."]
    #[inline(always)]
    pub fn channel3(&self) -> CHANNEL3_R {
        CHANNEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Select secure attribute."]
    #[inline(always)]
    pub fn channel4(&self) -> CHANNEL4_R {
        CHANNEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Select secure attribute."]
    #[inline(always)]
    pub fn channel5(&self) -> CHANNEL5_R {
        CHANNEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Select secure attribute."]
    #[inline(always)]
    pub fn channel6(&self) -> CHANNEL6_R {
        CHANNEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Select secure attribute."]
    #[inline(always)]
    pub fn channel7(&self) -> CHANNEL7_R {
        CHANNEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Select secure attribute."]
    #[inline(always)]
    pub fn channel8(&self) -> CHANNEL8_R {
        CHANNEL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Select secure attribute."]
    #[inline(always)]
    pub fn channel9(&self) -> CHANNEL9_R {
        CHANNEL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Select secure attribute."]
    #[inline(always)]
    pub fn channel10(&self) -> CHANNEL10_R {
        CHANNEL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Select secure attribute."]
    #[inline(always)]
    pub fn channel11(&self) -> CHANNEL11_R {
        CHANNEL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Select secure attribute."]
    #[inline(always)]
    pub fn channel12(&self) -> CHANNEL12_R {
        CHANNEL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Select secure attribute."]
    #[inline(always)]
    pub fn channel13(&self) -> CHANNEL13_R {
        CHANNEL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Select secure attribute."]
    #[inline(always)]
    pub fn channel14(&self) -> CHANNEL14_R {
        CHANNEL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Select secure attribute."]
    #[inline(always)]
    pub fn channel15(&self) -> CHANNEL15_R {
        CHANNEL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select secure attribute."]
    #[inline(always)]
    pub fn channel0(&mut self) -> CHANNEL0_W {
        CHANNEL0_W { w: self }
    }
    #[doc = "Bit 1 - Select secure attribute."]
    #[inline(always)]
    pub fn channel1(&mut self) -> CHANNEL1_W {
        CHANNEL1_W { w: self }
    }
    #[doc = "Bit 2 - Select secure attribute."]
    #[inline(always)]
    pub fn channel2(&mut self) -> CHANNEL2_W {
        CHANNEL2_W { w: self }
    }
    #[doc = "Bit 3 - Select secure attribute."]
    #[inline(always)]
    pub fn channel3(&mut self) -> CHANNEL3_W {
        CHANNEL3_W { w: self }
    }
    #[doc = "Bit 4 - Select secure attribute."]
    #[inline(always)]
    pub fn channel4(&mut self) -> CHANNEL4_W {
        CHANNEL4_W { w: self }
    }
    #[doc = "Bit 5 - Select secure attribute."]
    #[inline(always)]
    pub fn channel5(&mut self) -> CHANNEL5_W {
        CHANNEL5_W { w: self }
    }
    #[doc = "Bit 6 - Select secure attribute."]
    #[inline(always)]
    pub fn channel6(&mut self) -> CHANNEL6_W {
        CHANNEL6_W { w: self }
    }
    #[doc = "Bit 7 - Select secure attribute."]
    #[inline(always)]
    pub fn channel7(&mut self) -> CHANNEL7_W {
        CHANNEL7_W { w: self }
    }
    #[doc = "Bit 8 - Select secure attribute."]
    #[inline(always)]
    pub fn channel8(&mut self) -> CHANNEL8_W {
        CHANNEL8_W { w: self }
    }
    #[doc = "Bit 9 - Select secure attribute."]
    #[inline(always)]
    pub fn channel9(&mut self) -> CHANNEL9_W {
        CHANNEL9_W { w: self }
    }
    #[doc = "Bit 10 - Select secure attribute."]
    #[inline(always)]
    pub fn channel10(&mut self) -> CHANNEL10_W {
        CHANNEL10_W { w: self }
    }
    #[doc = "Bit 11 - Select secure attribute."]
    #[inline(always)]
    pub fn channel11(&mut self) -> CHANNEL11_W {
        CHANNEL11_W { w: self }
    }
    #[doc = "Bit 12 - Select secure attribute."]
    #[inline(always)]
    pub fn channel12(&mut self) -> CHANNEL12_W {
        CHANNEL12_W { w: self }
    }
    #[doc = "Bit 13 - Select secure attribute."]
    #[inline(always)]
    pub fn channel13(&mut self) -> CHANNEL13_W {
        CHANNEL13_W { w: self }
    }
    #[doc = "Bit 14 - Select secure attribute."]
    #[inline(always)]
    pub fn channel14(&mut self) -> CHANNEL14_W {
        CHANNEL14_W { w: self }
    }
    #[doc = "Bit 15 - Select secure attribute."]
    #[inline(always)]
    pub fn channel15(&mut self) -> CHANNEL15_W {
        CHANNEL15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
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
#[doc = "`reset()` method sets PERM to value 0xffff"]
impl crate::Resettable for PERM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
