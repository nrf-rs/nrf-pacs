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
#[doc = "Enable subscription to IPC channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN0_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN0` reader - Enable subscription to IPC channel 0"]
pub struct CHEN0_R(crate::FieldReader<bool, CHEN0_A>);
impl CHEN0_R {
    #[inline(always)]
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
#[doc = "Field `CHEN0` writer - Enable subscription to IPC channel 0"]
pub struct CHEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN0_A::DISABLE)
    }
    #[doc = "Enable events"]
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
#[doc = "Enable subscription to IPC channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN1_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN1` reader - Enable subscription to IPC channel 1"]
pub struct CHEN1_R(crate::FieldReader<bool, CHEN1_A>);
impl CHEN1_R {
    #[inline(always)]
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
#[doc = "Field `CHEN1` writer - Enable subscription to IPC channel 1"]
pub struct CHEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN1_A::DISABLE)
    }
    #[doc = "Enable events"]
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
#[doc = "Enable subscription to IPC channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN2_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN2` reader - Enable subscription to IPC channel 2"]
pub struct CHEN2_R(crate::FieldReader<bool, CHEN2_A>);
impl CHEN2_R {
    #[inline(always)]
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
#[doc = "Field `CHEN2` writer - Enable subscription to IPC channel 2"]
pub struct CHEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN2_A::DISABLE)
    }
    #[doc = "Enable events"]
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
#[doc = "Enable subscription to IPC channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN3_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN3` reader - Enable subscription to IPC channel 3"]
pub struct CHEN3_R(crate::FieldReader<bool, CHEN3_A>);
impl CHEN3_R {
    #[inline(always)]
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
#[doc = "Field `CHEN3` writer - Enable subscription to IPC channel 3"]
pub struct CHEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN3_A::DISABLE)
    }
    #[doc = "Enable events"]
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
#[doc = "Enable subscription to IPC channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN4_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN4` reader - Enable subscription to IPC channel 4"]
pub struct CHEN4_R(crate::FieldReader<bool, CHEN4_A>);
impl CHEN4_R {
    #[inline(always)]
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
#[doc = "Field `CHEN4` writer - Enable subscription to IPC channel 4"]
pub struct CHEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN4_A::DISABLE)
    }
    #[doc = "Enable events"]
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
#[doc = "Enable subscription to IPC channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN5_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN5` reader - Enable subscription to IPC channel 5"]
pub struct CHEN5_R(crate::FieldReader<bool, CHEN5_A>);
impl CHEN5_R {
    #[inline(always)]
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
#[doc = "Field `CHEN5` writer - Enable subscription to IPC channel 5"]
pub struct CHEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN5_A::DISABLE)
    }
    #[doc = "Enable events"]
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
#[doc = "Enable subscription to IPC channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN6_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN6_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN6` reader - Enable subscription to IPC channel 6"]
pub struct CHEN6_R(crate::FieldReader<bool, CHEN6_A>);
impl CHEN6_R {
    #[inline(always)]
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
#[doc = "Field `CHEN6` writer - Enable subscription to IPC channel 6"]
pub struct CHEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN6_A::DISABLE)
    }
    #[doc = "Enable events"]
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
#[doc = "Enable subscription to IPC channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN7_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN7_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN7` reader - Enable subscription to IPC channel 7"]
pub struct CHEN7_R(crate::FieldReader<bool, CHEN7_A>);
impl CHEN7_R {
    #[inline(always)]
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
#[doc = "Field `CHEN7` writer - Enable subscription to IPC channel 7"]
pub struct CHEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN7_A::DISABLE)
    }
    #[doc = "Enable events"]
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
#[doc = "Enable subscription to IPC channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN8_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN8_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN8` reader - Enable subscription to IPC channel 8"]
pub struct CHEN8_R(crate::FieldReader<bool, CHEN8_A>);
impl CHEN8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHEN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN8_A {
        match self.bits {
            false => CHEN8_A::DISABLE,
            true => CHEN8_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN8_A::ENABLE
    }
}
impl core::ops::Deref for CHEN8_R {
    type Target = crate::FieldReader<bool, CHEN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN8` writer - Enable subscription to IPC channel 8"]
pub struct CHEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN8_A::DISABLE)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN8_A::ENABLE)
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
#[doc = "Enable subscription to IPC channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN9_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN9_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN9` reader - Enable subscription to IPC channel 9"]
pub struct CHEN9_R(crate::FieldReader<bool, CHEN9_A>);
impl CHEN9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHEN9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN9_A {
        match self.bits {
            false => CHEN9_A::DISABLE,
            true => CHEN9_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN9_A::ENABLE
    }
}
impl core::ops::Deref for CHEN9_R {
    type Target = crate::FieldReader<bool, CHEN9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN9` writer - Enable subscription to IPC channel 9"]
pub struct CHEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN9_A::DISABLE)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN9_A::ENABLE)
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
#[doc = "Enable subscription to IPC channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN10_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN10_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN10` reader - Enable subscription to IPC channel 10"]
pub struct CHEN10_R(crate::FieldReader<bool, CHEN10_A>);
impl CHEN10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHEN10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN10_A {
        match self.bits {
            false => CHEN10_A::DISABLE,
            true => CHEN10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN10_A::ENABLE
    }
}
impl core::ops::Deref for CHEN10_R {
    type Target = crate::FieldReader<bool, CHEN10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN10` writer - Enable subscription to IPC channel 10"]
pub struct CHEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN10_A::DISABLE)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN10_A::ENABLE)
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
#[doc = "Enable subscription to IPC channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN11_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN11_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN11` reader - Enable subscription to IPC channel 11"]
pub struct CHEN11_R(crate::FieldReader<bool, CHEN11_A>);
impl CHEN11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHEN11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN11_A {
        match self.bits {
            false => CHEN11_A::DISABLE,
            true => CHEN11_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN11_A::ENABLE
    }
}
impl core::ops::Deref for CHEN11_R {
    type Target = crate::FieldReader<bool, CHEN11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN11` writer - Enable subscription to IPC channel 11"]
pub struct CHEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN11_A::DISABLE)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN11_A::ENABLE)
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
#[doc = "Enable subscription to IPC channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN12_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN12_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN12` reader - Enable subscription to IPC channel 12"]
pub struct CHEN12_R(crate::FieldReader<bool, CHEN12_A>);
impl CHEN12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHEN12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN12_A {
        match self.bits {
            false => CHEN12_A::DISABLE,
            true => CHEN12_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN12_A::ENABLE
    }
}
impl core::ops::Deref for CHEN12_R {
    type Target = crate::FieldReader<bool, CHEN12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN12` writer - Enable subscription to IPC channel 12"]
pub struct CHEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN12_A::DISABLE)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN12_A::ENABLE)
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
#[doc = "Enable subscription to IPC channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN13_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN13_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN13` reader - Enable subscription to IPC channel 13"]
pub struct CHEN13_R(crate::FieldReader<bool, CHEN13_A>);
impl CHEN13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHEN13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN13_A {
        match self.bits {
            false => CHEN13_A::DISABLE,
            true => CHEN13_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN13_A::ENABLE
    }
}
impl core::ops::Deref for CHEN13_R {
    type Target = crate::FieldReader<bool, CHEN13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN13` writer - Enable subscription to IPC channel 13"]
pub struct CHEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN13_A::DISABLE)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN13_A::ENABLE)
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
#[doc = "Enable subscription to IPC channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN14_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN14_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN14` reader - Enable subscription to IPC channel 14"]
pub struct CHEN14_R(crate::FieldReader<bool, CHEN14_A>);
impl CHEN14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHEN14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN14_A {
        match self.bits {
            false => CHEN14_A::DISABLE,
            true => CHEN14_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN14_A::ENABLE
    }
}
impl core::ops::Deref for CHEN14_R {
    type Target = crate::FieldReader<bool, CHEN14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN14` writer - Enable subscription to IPC channel 14"]
pub struct CHEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN14_A::DISABLE)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN14_A::ENABLE)
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
#[doc = "Enable subscription to IPC channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN15_A {
    #[doc = "0: Disable events"]
    DISABLE = 0,
    #[doc = "1: Enable events"]
    ENABLE = 1,
}
impl From<CHEN15_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN15` reader - Enable subscription to IPC channel 15"]
pub struct CHEN15_R(crate::FieldReader<bool, CHEN15_A>);
impl CHEN15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHEN15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN15_A {
        match self.bits {
            false => CHEN15_A::DISABLE,
            true => CHEN15_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHEN15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHEN15_A::ENABLE
    }
}
impl core::ops::Deref for CHEN15_R {
    type Target = crate::FieldReader<bool, CHEN15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN15` writer - Enable subscription to IPC channel 15"]
pub struct CHEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable events"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN15_A::DISABLE)
    }
    #[doc = "Enable events"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN15_A::ENABLE)
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
    #[doc = "Bit 0 - Enable subscription to IPC channel 0"]
    #[inline(always)]
    pub fn chen0(&self) -> CHEN0_R {
        CHEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable subscription to IPC channel 1"]
    #[inline(always)]
    pub fn chen1(&self) -> CHEN1_R {
        CHEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable subscription to IPC channel 2"]
    #[inline(always)]
    pub fn chen2(&self) -> CHEN2_R {
        CHEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable subscription to IPC channel 3"]
    #[inline(always)]
    pub fn chen3(&self) -> CHEN3_R {
        CHEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable subscription to IPC channel 4"]
    #[inline(always)]
    pub fn chen4(&self) -> CHEN4_R {
        CHEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable subscription to IPC channel 5"]
    #[inline(always)]
    pub fn chen5(&self) -> CHEN5_R {
        CHEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable subscription to IPC channel 6"]
    #[inline(always)]
    pub fn chen6(&self) -> CHEN6_R {
        CHEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable subscription to IPC channel 7"]
    #[inline(always)]
    pub fn chen7(&self) -> CHEN7_R {
        CHEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable subscription to IPC channel 8"]
    #[inline(always)]
    pub fn chen8(&self) -> CHEN8_R {
        CHEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable subscription to IPC channel 9"]
    #[inline(always)]
    pub fn chen9(&self) -> CHEN9_R {
        CHEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable subscription to IPC channel 10"]
    #[inline(always)]
    pub fn chen10(&self) -> CHEN10_R {
        CHEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable subscription to IPC channel 11"]
    #[inline(always)]
    pub fn chen11(&self) -> CHEN11_R {
        CHEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable subscription to IPC channel 12"]
    #[inline(always)]
    pub fn chen12(&self) -> CHEN12_R {
        CHEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable subscription to IPC channel 13"]
    #[inline(always)]
    pub fn chen13(&self) -> CHEN13_R {
        CHEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable subscription to IPC channel 14"]
    #[inline(always)]
    pub fn chen14(&self) -> CHEN14_R {
        CHEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable subscription to IPC channel 15"]
    #[inline(always)]
    pub fn chen15(&self) -> CHEN15_R {
        CHEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable subscription to IPC channel 0"]
    #[inline(always)]
    pub fn chen0(&mut self) -> CHEN0_W {
        CHEN0_W { w: self }
    }
    #[doc = "Bit 1 - Enable subscription to IPC channel 1"]
    #[inline(always)]
    pub fn chen1(&mut self) -> CHEN1_W {
        CHEN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable subscription to IPC channel 2"]
    #[inline(always)]
    pub fn chen2(&mut self) -> CHEN2_W {
        CHEN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable subscription to IPC channel 3"]
    #[inline(always)]
    pub fn chen3(&mut self) -> CHEN3_W {
        CHEN3_W { w: self }
    }
    #[doc = "Bit 4 - Enable subscription to IPC channel 4"]
    #[inline(always)]
    pub fn chen4(&mut self) -> CHEN4_W {
        CHEN4_W { w: self }
    }
    #[doc = "Bit 5 - Enable subscription to IPC channel 5"]
    #[inline(always)]
    pub fn chen5(&mut self) -> CHEN5_W {
        CHEN5_W { w: self }
    }
    #[doc = "Bit 6 - Enable subscription to IPC channel 6"]
    #[inline(always)]
    pub fn chen6(&mut self) -> CHEN6_W {
        CHEN6_W { w: self }
    }
    #[doc = "Bit 7 - Enable subscription to IPC channel 7"]
    #[inline(always)]
    pub fn chen7(&mut self) -> CHEN7_W {
        CHEN7_W { w: self }
    }
    #[doc = "Bit 8 - Enable subscription to IPC channel 8"]
    #[inline(always)]
    pub fn chen8(&mut self) -> CHEN8_W {
        CHEN8_W { w: self }
    }
    #[doc = "Bit 9 - Enable subscription to IPC channel 9"]
    #[inline(always)]
    pub fn chen9(&mut self) -> CHEN9_W {
        CHEN9_W { w: self }
    }
    #[doc = "Bit 10 - Enable subscription to IPC channel 10"]
    #[inline(always)]
    pub fn chen10(&mut self) -> CHEN10_W {
        CHEN10_W { w: self }
    }
    #[doc = "Bit 11 - Enable subscription to IPC channel 11"]
    #[inline(always)]
    pub fn chen11(&mut self) -> CHEN11_W {
        CHEN11_W { w: self }
    }
    #[doc = "Bit 12 - Enable subscription to IPC channel 12"]
    #[inline(always)]
    pub fn chen12(&mut self) -> CHEN12_W {
        CHEN12_W { w: self }
    }
    #[doc = "Bit 13 - Enable subscription to IPC channel 13"]
    #[inline(always)]
    pub fn chen13(&mut self) -> CHEN13_W {
        CHEN13_W { w: self }
    }
    #[doc = "Bit 14 - Enable subscription to IPC channel 14"]
    #[inline(always)]
    pub fn chen14(&mut self) -> CHEN14_W {
        CHEN14_W { w: self }
    }
    #[doc = "Bit 15 - Enable subscription to IPC channel 15"]
    #[inline(always)]
    pub fn chen15(&mut self) -> CHEN15_W {
        CHEN15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_cnf](index.html) module"]
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
