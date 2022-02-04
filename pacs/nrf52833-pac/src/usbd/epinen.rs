#[doc = "Register `EPINEN` reader"]
pub struct R(crate::R<EPINEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPINEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPINEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPINEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPINEN` writer"]
pub struct W(crate::W<EPINEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPINEN_SPEC>;
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
impl From<crate::W<EPINEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPINEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable IN endpoint 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0_A {
    #[doc = "0: Disable endpoint IN 0 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 0 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN0_A> for bool {
    #[inline(always)]
    fn from(variant: IN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN0` reader - Enable IN endpoint 0"]
pub struct IN0_R(crate::FieldReader<bool, IN0_A>);
impl IN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN0_A {
        match self.bits {
            false => IN0_A::DISABLE,
            true => IN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IN0_A::ENABLE
    }
}
impl core::ops::Deref for IN0_R {
    type Target = crate::FieldReader<bool, IN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN0` writer - Enable IN endpoint 0"]
pub struct IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable endpoint IN 0 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN0_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 0 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN0_A::ENABLE)
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
#[doc = "Enable IN endpoint 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1_A {
    #[doc = "0: Disable endpoint IN 1 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 1 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN1_A> for bool {
    #[inline(always)]
    fn from(variant: IN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN1` reader - Enable IN endpoint 1"]
pub struct IN1_R(crate::FieldReader<bool, IN1_A>);
impl IN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN1_A {
        match self.bits {
            false => IN1_A::DISABLE,
            true => IN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IN1_A::ENABLE
    }
}
impl core::ops::Deref for IN1_R {
    type Target = crate::FieldReader<bool, IN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN1` writer - Enable IN endpoint 1"]
pub struct IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable endpoint IN 1 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN1_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 1 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN1_A::ENABLE)
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
#[doc = "Enable IN endpoint 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2_A {
    #[doc = "0: Disable endpoint IN 2 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 2 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN2_A> for bool {
    #[inline(always)]
    fn from(variant: IN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN2` reader - Enable IN endpoint 2"]
pub struct IN2_R(crate::FieldReader<bool, IN2_A>);
impl IN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN2_A {
        match self.bits {
            false => IN2_A::DISABLE,
            true => IN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IN2_A::ENABLE
    }
}
impl core::ops::Deref for IN2_R {
    type Target = crate::FieldReader<bool, IN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN2` writer - Enable IN endpoint 2"]
pub struct IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable endpoint IN 2 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN2_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 2 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN2_A::ENABLE)
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
#[doc = "Enable IN endpoint 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3_A {
    #[doc = "0: Disable endpoint IN 3 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 3 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN3_A> for bool {
    #[inline(always)]
    fn from(variant: IN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN3` reader - Enable IN endpoint 3"]
pub struct IN3_R(crate::FieldReader<bool, IN3_A>);
impl IN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN3_A {
        match self.bits {
            false => IN3_A::DISABLE,
            true => IN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IN3_A::ENABLE
    }
}
impl core::ops::Deref for IN3_R {
    type Target = crate::FieldReader<bool, IN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN3` writer - Enable IN endpoint 3"]
pub struct IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable endpoint IN 3 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN3_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 3 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN3_A::ENABLE)
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
#[doc = "Enable IN endpoint 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN4_A {
    #[doc = "0: Disable endpoint IN 4 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 4 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN4_A> for bool {
    #[inline(always)]
    fn from(variant: IN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN4` reader - Enable IN endpoint 4"]
pub struct IN4_R(crate::FieldReader<bool, IN4_A>);
impl IN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN4_A {
        match self.bits {
            false => IN4_A::DISABLE,
            true => IN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IN4_A::ENABLE
    }
}
impl core::ops::Deref for IN4_R {
    type Target = crate::FieldReader<bool, IN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN4` writer - Enable IN endpoint 4"]
pub struct IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> IN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable endpoint IN 4 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN4_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 4 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN4_A::ENABLE)
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
#[doc = "Enable IN endpoint 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN5_A {
    #[doc = "0: Disable endpoint IN 5 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 5 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN5_A> for bool {
    #[inline(always)]
    fn from(variant: IN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN5` reader - Enable IN endpoint 5"]
pub struct IN5_R(crate::FieldReader<bool, IN5_A>);
impl IN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN5_A {
        match self.bits {
            false => IN5_A::DISABLE,
            true => IN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IN5_A::ENABLE
    }
}
impl core::ops::Deref for IN5_R {
    type Target = crate::FieldReader<bool, IN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN5` writer - Enable IN endpoint 5"]
pub struct IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> IN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable endpoint IN 5 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN5_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 5 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN5_A::ENABLE)
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
#[doc = "Enable IN endpoint 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN6_A {
    #[doc = "0: Disable endpoint IN 6 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 6 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN6_A> for bool {
    #[inline(always)]
    fn from(variant: IN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN6` reader - Enable IN endpoint 6"]
pub struct IN6_R(crate::FieldReader<bool, IN6_A>);
impl IN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN6_A {
        match self.bits {
            false => IN6_A::DISABLE,
            true => IN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IN6_A::ENABLE
    }
}
impl core::ops::Deref for IN6_R {
    type Target = crate::FieldReader<bool, IN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN6` writer - Enable IN endpoint 6"]
pub struct IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> IN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable endpoint IN 6 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN6_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 6 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN6_A::ENABLE)
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
#[doc = "Enable IN endpoint 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN7_A {
    #[doc = "0: Disable endpoint IN 7 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 7 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN7_A> for bool {
    #[inline(always)]
    fn from(variant: IN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN7` reader - Enable IN endpoint 7"]
pub struct IN7_R(crate::FieldReader<bool, IN7_A>);
impl IN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN7_A {
        match self.bits {
            false => IN7_A::DISABLE,
            true => IN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IN7_A::ENABLE
    }
}
impl core::ops::Deref for IN7_R {
    type Target = crate::FieldReader<bool, IN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN7` writer - Enable IN endpoint 7"]
pub struct IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> IN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable endpoint IN 7 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN7_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 7 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN7_A::ENABLE)
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
#[doc = "Enable ISO IN endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOIN_A {
    #[doc = "0: Disable ISO IN endpoint 8"]
    DISABLE = 0,
    #[doc = "1: Enable ISO IN endpoint 8"]
    ENABLE = 1,
}
impl From<ISOIN_A> for bool {
    #[inline(always)]
    fn from(variant: ISOIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISOIN` reader - Enable ISO IN endpoint"]
pub struct ISOIN_R(crate::FieldReader<bool, ISOIN_A>);
impl ISOIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISOIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOIN_A {
        match self.bits {
            false => ISOIN_A::DISABLE,
            true => ISOIN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ISOIN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ISOIN_A::ENABLE
    }
}
impl core::ops::Deref for ISOIN_R {
    type Target = crate::FieldReader<bool, ISOIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOIN` writer - Enable ISO IN endpoint"]
pub struct ISOIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable ISO IN endpoint 8"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ISOIN_A::DISABLE)
    }
    #[doc = "Enable ISO IN endpoint 8"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ISOIN_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Enable IN endpoint 0"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable IN endpoint 1"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable IN endpoint 2"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable IN endpoint 3"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable IN endpoint 4"]
    #[inline(always)]
    pub fn in4(&self) -> IN4_R {
        IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable IN endpoint 5"]
    #[inline(always)]
    pub fn in5(&self) -> IN5_R {
        IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable IN endpoint 6"]
    #[inline(always)]
    pub fn in6(&self) -> IN6_R {
        IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable IN endpoint 7"]
    #[inline(always)]
    pub fn in7(&self) -> IN7_R {
        IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable ISO IN endpoint"]
    #[inline(always)]
    pub fn isoin(&self) -> ISOIN_R {
        ISOIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IN endpoint 0"]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W {
        IN0_W { w: self }
    }
    #[doc = "Bit 1 - Enable IN endpoint 1"]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W {
        IN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable IN endpoint 2"]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W {
        IN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable IN endpoint 3"]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W {
        IN3_W { w: self }
    }
    #[doc = "Bit 4 - Enable IN endpoint 4"]
    #[inline(always)]
    pub fn in4(&mut self) -> IN4_W {
        IN4_W { w: self }
    }
    #[doc = "Bit 5 - Enable IN endpoint 5"]
    #[inline(always)]
    pub fn in5(&mut self) -> IN5_W {
        IN5_W { w: self }
    }
    #[doc = "Bit 6 - Enable IN endpoint 6"]
    #[inline(always)]
    pub fn in6(&mut self) -> IN6_W {
        IN6_W { w: self }
    }
    #[doc = "Bit 7 - Enable IN endpoint 7"]
    #[inline(always)]
    pub fn in7(&mut self) -> IN7_W {
        IN7_W { w: self }
    }
    #[doc = "Bit 8 - Enable ISO IN endpoint"]
    #[inline(always)]
    pub fn isoin(&mut self) -> ISOIN_W {
        ISOIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint IN enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epinen](index.html) module"]
pub struct EPINEN_SPEC;
impl crate::RegisterSpec for EPINEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epinen::R](R) reader structure"]
impl crate::Readable for EPINEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epinen::W](W) writer structure"]
impl crate::Writable for EPINEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPINEN to value 0x01"]
impl crate::Resettable for EPINEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
