#[doc = "Register `RECEIVE_CNF[%s]` reader"]
pub struct R(crate::R<RECEIVE_CNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_CNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_CNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_CNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECEIVE_CNF[%s]` writer"]
pub struct W(crate::W<RECEIVE_CNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEIVE_CNF_SPEC>;
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
impl From<crate::W<RECEIVE_CNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEIVE_CNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable subscription to channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN0_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN0` reader - Enable subscription to channel 0."]
pub struct CHEN0_R(crate::FieldReader<bool, CHEN0_A>);
impl CHEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN0_A {
        match self.bits {
            false => CHEN0_A::DISABLE,
            true => CHEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN0_A::ENABLE
    }
}
impl core::ops::Deref for CHEN0_R {
    type Target = crate::FieldReader<bool, CHEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN0` writer - Enable subscription to channel 0."]
pub struct CHEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN0_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN0_A::ENABLE)
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
#[doc = "Enable subscription to channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN1_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN1` reader - Enable subscription to channel 1."]
pub struct CHEN1_R(crate::FieldReader<bool, CHEN1_A>);
impl CHEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN1_A {
        match self.bits {
            false => CHEN1_A::DISABLE,
            true => CHEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN1_A::ENABLE
    }
}
impl core::ops::Deref for CHEN1_R {
    type Target = crate::FieldReader<bool, CHEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN1` writer - Enable subscription to channel 1."]
pub struct CHEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN1_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN1_A::ENABLE)
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
#[doc = "Enable subscription to channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN2_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN2` reader - Enable subscription to channel 2."]
pub struct CHEN2_R(crate::FieldReader<bool, CHEN2_A>);
impl CHEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN2_A {
        match self.bits {
            false => CHEN2_A::DISABLE,
            true => CHEN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN2_A::ENABLE
    }
}
impl core::ops::Deref for CHEN2_R {
    type Target = crate::FieldReader<bool, CHEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN2` writer - Enable subscription to channel 2."]
pub struct CHEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN2_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN2_A::ENABLE)
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
#[doc = "Enable subscription to channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN3_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN3` reader - Enable subscription to channel 3."]
pub struct CHEN3_R(crate::FieldReader<bool, CHEN3_A>);
impl CHEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN3_A {
        match self.bits {
            false => CHEN3_A::DISABLE,
            true => CHEN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN3_A::ENABLE
    }
}
impl core::ops::Deref for CHEN3_R {
    type Target = crate::FieldReader<bool, CHEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN3` writer - Enable subscription to channel 3."]
pub struct CHEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN3_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN3_A::ENABLE)
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
#[doc = "Enable subscription to channel 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN4_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN4` reader - Enable subscription to channel 4."]
pub struct CHEN4_R(crate::FieldReader<bool, CHEN4_A>);
impl CHEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN4_A {
        match self.bits {
            false => CHEN4_A::DISABLE,
            true => CHEN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN4_A::ENABLE
    }
}
impl core::ops::Deref for CHEN4_R {
    type Target = crate::FieldReader<bool, CHEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN4` writer - Enable subscription to channel 4."]
pub struct CHEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN4_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN4_A::ENABLE)
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
#[doc = "Enable subscription to channel 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN5_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN5` reader - Enable subscription to channel 5."]
pub struct CHEN5_R(crate::FieldReader<bool, CHEN5_A>);
impl CHEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN5_A {
        match self.bits {
            false => CHEN5_A::DISABLE,
            true => CHEN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN5_A::ENABLE
    }
}
impl core::ops::Deref for CHEN5_R {
    type Target = crate::FieldReader<bool, CHEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN5` writer - Enable subscription to channel 5."]
pub struct CHEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN5_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN5_A::ENABLE)
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
#[doc = "Enable subscription to channel 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN6_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN6_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN6` reader - Enable subscription to channel 6."]
pub struct CHEN6_R(crate::FieldReader<bool, CHEN6_A>);
impl CHEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN6_A {
        match self.bits {
            false => CHEN6_A::DISABLE,
            true => CHEN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN6_A::ENABLE
    }
}
impl core::ops::Deref for CHEN6_R {
    type Target = crate::FieldReader<bool, CHEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN6` writer - Enable subscription to channel 6."]
pub struct CHEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN6_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN6_A::ENABLE)
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
#[doc = "Enable subscription to channel 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN7_A {
    #[doc = "0: Disable events."]
    DISABLE = 0,
    #[doc = "1: Enable events."]
    ENABLE = 1,
}
impl From<CHEN7_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN7` reader - Enable subscription to channel 7."]
pub struct CHEN7_R(crate::FieldReader<bool, CHEN7_A>);
impl CHEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN7_A {
        match self.bits {
            false => CHEN7_A::DISABLE,
            true => CHEN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN7_A::ENABLE
    }
}
impl core::ops::Deref for CHEN7_R {
    type Target = crate::FieldReader<bool, CHEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN7` writer - Enable subscription to channel 7."]
pub struct CHEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN7_A::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN7_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Enable subscription to channel 0."]
    #[inline(always)]
    pub fn chen0(&self) -> CHEN0_R {
        CHEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable subscription to channel 1."]
    #[inline(always)]
    pub fn chen1(&self) -> CHEN1_R {
        CHEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable subscription to channel 2."]
    #[inline(always)]
    pub fn chen2(&self) -> CHEN2_R {
        CHEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable subscription to channel 3."]
    #[inline(always)]
    pub fn chen3(&self) -> CHEN3_R {
        CHEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable subscription to channel 4."]
    #[inline(always)]
    pub fn chen4(&self) -> CHEN4_R {
        CHEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable subscription to channel 5."]
    #[inline(always)]
    pub fn chen5(&self) -> CHEN5_R {
        CHEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable subscription to channel 6."]
    #[inline(always)]
    pub fn chen6(&self) -> CHEN6_R {
        CHEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable subscription to channel 7."]
    #[inline(always)]
    pub fn chen7(&self) -> CHEN7_R {
        CHEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable subscription to channel 0."]
    #[inline(always)]
    pub fn chen0(&mut self) -> CHEN0_W {
        CHEN0_W { w: self }
    }
    #[doc = "Bit 1 - Enable subscription to channel 1."]
    #[inline(always)]
    pub fn chen1(&mut self) -> CHEN1_W {
        CHEN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable subscription to channel 2."]
    #[inline(always)]
    pub fn chen2(&mut self) -> CHEN2_W {
        CHEN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable subscription to channel 3."]
    #[inline(always)]
    pub fn chen3(&mut self) -> CHEN3_W {
        CHEN3_W { w: self }
    }
    #[doc = "Bit 4 - Enable subscription to channel 4."]
    #[inline(always)]
    pub fn chen4(&mut self) -> CHEN4_W {
        CHEN4_W { w: self }
    }
    #[doc = "Bit 5 - Enable subscription to channel 5."]
    #[inline(always)]
    pub fn chen5(&mut self) -> CHEN5_W {
        CHEN5_W { w: self }
    }
    #[doc = "Bit 6 - Enable subscription to channel 6."]
    #[inline(always)]
    pub fn chen6(&mut self) -> CHEN6_W {
        CHEN6_W { w: self }
    }
    #[doc = "Bit 7 - Enable subscription to channel 7."]
    #[inline(always)]
    pub fn chen7(&mut self) -> CHEN7_W {
        CHEN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_cnf](index.html) module"]
pub struct RECEIVE_CNF_SPEC;
impl crate::RegisterSpec for RECEIVE_CNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_cnf::R](R) reader structure"]
impl crate::Readable for RECEIVE_CNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receive_cnf::W](W) writer structure"]
impl crate::Writable for RECEIVE_CNF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RECEIVE_CNF[%s]
to value 0"]
impl crate::Resettable for RECEIVE_CNF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
