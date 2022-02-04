#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable or disable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Enable or disable interrupt for event READY"]
pub struct READY_R(crate::FieldReader<bool, READY_A>);
impl READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::DISABLED,
            true => READY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == READY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == READY_A::ENABLED
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, READY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READY` writer - Enable or disable interrupt for event READY"]
pub struct READY_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event FIELDDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDDETECTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<FIELDDETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDDETECTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDDETECTED` reader - Enable or disable interrupt for event FIELDDETECTED"]
pub struct FIELDDETECTED_R(crate::FieldReader<bool, FIELDDETECTED_A>);
impl FIELDDETECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIELDDETECTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDDETECTED_A {
        match self.bits {
            false => FIELDDETECTED_A::DISABLED,
            true => FIELDDETECTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FIELDDETECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FIELDDETECTED_A::ENABLED
    }
}
impl core::ops::Deref for FIELDDETECTED_R {
    type Target = crate::FieldReader<bool, FIELDDETECTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIELDDETECTED` writer - Enable or disable interrupt for event FIELDDETECTED"]
pub struct FIELDDETECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELDDETECTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDDETECTED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIELDDETECTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIELDDETECTED_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event FIELDLOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDLOST_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<FIELDLOST_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDLOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDLOST` reader - Enable or disable interrupt for event FIELDLOST"]
pub struct FIELDLOST_R(crate::FieldReader<bool, FIELDLOST_A>);
impl FIELDLOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIELDLOST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDLOST_A {
        match self.bits {
            false => FIELDLOST_A::DISABLED,
            true => FIELDLOST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FIELDLOST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FIELDLOST_A::ENABLED
    }
}
impl core::ops::Deref for FIELDLOST_R {
    type Target = crate::FieldReader<bool, FIELDLOST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIELDLOST` writer - Enable or disable interrupt for event FIELDLOST"]
pub struct FIELDLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELDLOST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDLOST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIELDLOST_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIELDLOST_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TXFRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMESTART_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TXFRAMESTART_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRAMESTART` reader - Enable or disable interrupt for event TXFRAMESTART"]
pub struct TXFRAMESTART_R(crate::FieldReader<bool, TXFRAMESTART_A>);
impl TXFRAMESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFRAMESTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFRAMESTART_A {
        match self.bits {
            false => TXFRAMESTART_A::DISABLED,
            true => TXFRAMESTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXFRAMESTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXFRAMESTART_A::ENABLED
    }
}
impl core::ops::Deref for TXFRAMESTART_R {
    type Target = crate::FieldReader<bool, TXFRAMESTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFRAMESTART` writer - Enable or disable interrupt for event TXFRAMESTART"]
pub struct TXFRAMESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFRAMESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFRAMESTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFRAMESTART_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFRAMESTART_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TXFRAMEEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMEEND_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TXFRAMEEND_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMEEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRAMEEND` reader - Enable or disable interrupt for event TXFRAMEEND"]
pub struct TXFRAMEEND_R(crate::FieldReader<bool, TXFRAMEEND_A>);
impl TXFRAMEEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFRAMEEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFRAMEEND_A {
        match self.bits {
            false => TXFRAMEEND_A::DISABLED,
            true => TXFRAMEEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXFRAMEEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXFRAMEEND_A::ENABLED
    }
}
impl core::ops::Deref for TXFRAMEEND_R {
    type Target = crate::FieldReader<bool, TXFRAMEEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFRAMEEND` writer - Enable or disable interrupt for event TXFRAMEEND"]
pub struct TXFRAMEEND_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFRAMEEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFRAMEEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFRAMEEND_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFRAMEEND_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event RXFRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRAMESTART_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RXFRAMESTART_A> for bool {
    #[inline(always)]
    fn from(variant: RXFRAMESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRAMESTART` reader - Enable or disable interrupt for event RXFRAMESTART"]
pub struct RXFRAMESTART_R(crate::FieldReader<bool, RXFRAMESTART_A>);
impl RXFRAMESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFRAMESTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFRAMESTART_A {
        match self.bits {
            false => RXFRAMESTART_A::DISABLED,
            true => RXFRAMESTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXFRAMESTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXFRAMESTART_A::ENABLED
    }
}
impl core::ops::Deref for RXFRAMESTART_R {
    type Target = crate::FieldReader<bool, RXFRAMESTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFRAMESTART` writer - Enable or disable interrupt for event RXFRAMESTART"]
pub struct RXFRAMESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFRAMESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFRAMESTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXFRAMESTART_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXFRAMESTART_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event RXFRAMEEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRAMEEND_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RXFRAMEEND_A> for bool {
    #[inline(always)]
    fn from(variant: RXFRAMEEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRAMEEND` reader - Enable or disable interrupt for event RXFRAMEEND"]
pub struct RXFRAMEEND_R(crate::FieldReader<bool, RXFRAMEEND_A>);
impl RXFRAMEEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFRAMEEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFRAMEEND_A {
        match self.bits {
            false => RXFRAMEEND_A::DISABLED,
            true => RXFRAMEEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXFRAMEEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXFRAMEEND_A::ENABLED
    }
}
impl core::ops::Deref for RXFRAMEEND_R {
    type Target = crate::FieldReader<bool, RXFRAMEEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFRAMEEND` writer - Enable or disable interrupt for event RXFRAMEEND"]
pub struct RXFRAMEEND_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFRAMEEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFRAMEEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXFRAMEEND_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXFRAMEEND_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Enable or disable interrupt for event ERROR"]
pub struct ERROR_R(crate::FieldReader<bool, ERROR_A>);
impl ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::DISABLED,
            true => ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ERROR_A::ENABLED
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR` writer - Enable or disable interrupt for event ERROR"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERROR_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERROR_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event RXERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERROR_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RXERROR_A> for bool {
    #[inline(always)]
    fn from(variant: RXERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXERROR` reader - Enable or disable interrupt for event RXERROR"]
pub struct RXERROR_R(crate::FieldReader<bool, RXERROR_A>);
impl RXERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXERROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXERROR_A {
        match self.bits {
            false => RXERROR_A::DISABLED,
            true => RXERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXERROR_A::ENABLED
    }
}
impl core::ops::Deref for RXERROR_R {
    type Target = crate::FieldReader<bool, RXERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXERROR` writer - Enable or disable interrupt for event RXERROR"]
pub struct RXERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXERROR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXERROR_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXERROR_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event ENDRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` reader - Enable or disable interrupt for event ENDRX"]
pub struct ENDRX_R(crate::FieldReader<bool, ENDRX_A>);
impl ENDRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDRX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_A {
        match self.bits {
            false => ENDRX_A::DISABLED,
            true => ENDRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDRX_A::ENABLED
    }
}
impl core::ops::Deref for ENDRX_R {
    type Target = crate::FieldReader<bool, ENDRX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDRX` writer - Enable or disable interrupt for event ENDRX"]
pub struct ENDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDRX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDRX_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDRX_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event ENDTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTX_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDTX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDTX` reader - Enable or disable interrupt for event ENDTX"]
pub struct ENDTX_R(crate::FieldReader<bool, ENDTX_A>);
impl ENDTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDTX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDTX_A {
        match self.bits {
            false => ENDTX_A::DISABLED,
            true => ENDTX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDTX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDTX_A::ENABLED
    }
}
impl core::ops::Deref for ENDTX_R {
    type Target = crate::FieldReader<bool, ENDTX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDTX` writer - Enable or disable interrupt for event ENDTX"]
pub struct ENDTX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDTX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDTX_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDTX_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event AUTOCOLRESSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOCOLRESSTARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<AUTOCOLRESSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOCOLRESSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCOLRESSTARTED` reader - Enable or disable interrupt for event AUTOCOLRESSTARTED"]
pub struct AUTOCOLRESSTARTED_R(crate::FieldReader<bool, AUTOCOLRESSTARTED_A>);
impl AUTOCOLRESSTARTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOCOLRESSTARTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOCOLRESSTARTED_A {
        match self.bits {
            false => AUTOCOLRESSTARTED_A::DISABLED,
            true => AUTOCOLRESSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AUTOCOLRESSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AUTOCOLRESSTARTED_A::ENABLED
    }
}
impl core::ops::Deref for AUTOCOLRESSTARTED_R {
    type Target = crate::FieldReader<bool, AUTOCOLRESSTARTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOCOLRESSTARTED` writer - Enable or disable interrupt for event AUTOCOLRESSTARTED"]
pub struct AUTOCOLRESSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCOLRESSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOCOLRESSTARTED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOCOLRESSTARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOCOLRESSTARTED_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event COLLISION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLLISION_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<COLLISION_A> for bool {
    #[inline(always)]
    fn from(variant: COLLISION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLLISION` reader - Enable or disable interrupt for event COLLISION"]
pub struct COLLISION_R(crate::FieldReader<bool, COLLISION_A>);
impl COLLISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLLISION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COLLISION_A {
        match self.bits {
            false => COLLISION_A::DISABLED,
            true => COLLISION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COLLISION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COLLISION_A::ENABLED
    }
}
impl core::ops::Deref for COLLISION_R {
    type Target = crate::FieldReader<bool, COLLISION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLISION` writer - Enable or disable interrupt for event COLLISION"]
pub struct COLLISION_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLISION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COLLISION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COLLISION_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COLLISION_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event SELECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SELECTED_A> for bool {
    #[inline(always)]
    fn from(variant: SELECTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECTED` reader - Enable or disable interrupt for event SELECTED"]
pub struct SELECTED_R(crate::FieldReader<bool, SELECTED_A>);
impl SELECTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SELECTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELECTED_A {
        match self.bits {
            false => SELECTED_A::DISABLED,
            true => SELECTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SELECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SELECTED_A::ENABLED
    }
}
impl core::ops::Deref for SELECTED_R {
    type Target = crate::FieldReader<bool, SELECTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELECTED` writer - Enable or disable interrupt for event SELECTED"]
pub struct SELECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> SELECTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELECTED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SELECTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SELECTED_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<STARTED_A> for bool {
    #[inline(always)]
    fn from(variant: STARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` reader - Enable or disable interrupt for event STARTED"]
pub struct STARTED_R(crate::FieldReader<bool, STARTED_A>);
impl STARTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STARTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTED_A {
        match self.bits {
            false => STARTED_A::DISABLED,
            true => STARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == STARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == STARTED_A::ENABLED
    }
}
impl core::ops::Deref for STARTED_R {
    type Target = crate::FieldReader<bool, STARTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTED` writer - Enable or disable interrupt for event STARTED"]
pub struct STARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STARTED_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event FIELDDETECTED"]
    #[inline(always)]
    pub fn fielddetected(&self) -> FIELDDETECTED_R {
        FIELDDETECTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event FIELDLOST"]
    #[inline(always)]
    pub fn fieldlost(&self) -> FIELDLOST_R {
        FIELDLOST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event TXFRAMESTART"]
    #[inline(always)]
    pub fn txframestart(&self) -> TXFRAMESTART_R {
        TXFRAMESTART_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event TXFRAMEEND"]
    #[inline(always)]
    pub fn txframeend(&self) -> TXFRAMEEND_R {
        TXFRAMEEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event RXFRAMESTART"]
    #[inline(always)]
    pub fn rxframestart(&self) -> RXFRAMESTART_R {
        RXFRAMESTART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event RXFRAMEEND"]
    #[inline(always)]
    pub fn rxframeend(&self) -> RXFRAMEEND_R {
        RXFRAMEEND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event RXERROR"]
    #[inline(always)]
    pub fn rxerror(&self) -> RXERROR_R {
        RXERROR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event ENDTX"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event AUTOCOLRESSTARTED"]
    #[inline(always)]
    pub fn autocolresstarted(&self) -> AUTOCOLRESSTARTED_R {
        AUTOCOLRESSTARTED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable or disable interrupt for event COLLISION"]
    #[inline(always)]
    pub fn collision(&self) -> COLLISION_R {
        COLLISION_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for event SELECTED"]
    #[inline(always)]
    pub fn selected(&self) -> SELECTED_R {
        SELECTED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&self) -> STARTED_R {
        STARTED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event FIELDDETECTED"]
    #[inline(always)]
    pub fn fielddetected(&mut self) -> FIELDDETECTED_W {
        FIELDDETECTED_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event FIELDLOST"]
    #[inline(always)]
    pub fn fieldlost(&mut self) -> FIELDLOST_W {
        FIELDLOST_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event TXFRAMESTART"]
    #[inline(always)]
    pub fn txframestart(&mut self) -> TXFRAMESTART_W {
        TXFRAMESTART_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event TXFRAMEEND"]
    #[inline(always)]
    pub fn txframeend(&mut self) -> TXFRAMEEND_W {
        TXFRAMEEND_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event RXFRAMESTART"]
    #[inline(always)]
    pub fn rxframestart(&mut self) -> RXFRAMESTART_W {
        RXFRAMESTART_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event RXFRAMEEND"]
    #[inline(always)]
    pub fn rxframeend(&mut self) -> RXFRAMEEND_W {
        RXFRAMEEND_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event RXERROR"]
    #[inline(always)]
    pub fn rxerror(&mut self) -> RXERROR_W {
        RXERROR_W { w: self }
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W {
        ENDRX_W { w: self }
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event ENDTX"]
    #[inline(always)]
    pub fn endtx(&mut self) -> ENDTX_W {
        ENDTX_W { w: self }
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event AUTOCOLRESSTARTED"]
    #[inline(always)]
    pub fn autocolresstarted(&mut self) -> AUTOCOLRESSTARTED_W {
        AUTOCOLRESSTARTED_W { w: self }
    }
    #[doc = "Bit 18 - Enable or disable interrupt for event COLLISION"]
    #[inline(always)]
    pub fn collision(&mut self) -> COLLISION_W {
        COLLISION_W { w: self }
    }
    #[doc = "Bit 19 - Enable or disable interrupt for event SELECTED"]
    #[inline(always)]
    pub fn selected(&mut self) -> SELECTED_W {
        SELECTED_W { w: self }
    }
    #[doc = "Bit 20 - Enable or disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&mut self) -> STARTED_W {
        STARTED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
