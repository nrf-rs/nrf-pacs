#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Disable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Disable interrupt on READY event."]
pub struct READY_R(crate::FieldReader<bool, READY_A>);
impl READY_R {
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
#[doc = "Disable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<READY_AW> for bool {
    #[inline(always)]
    fn from(variant: READY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Disable interrupt on READY event."]
pub struct READY_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(READY_AW::CLEAR)
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
#[doc = "Disable interrupt on ADDRESS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ADDRESS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS` reader - Disable interrupt on ADDRESS event."]
pub struct ADDRESS_R(crate::FieldReader<bool, ADDRESS_A>);
impl ADDRESS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRESS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_A {
        match self.bits {
            false => ADDRESS_A::DISABLED,
            true => ADDRESS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADDRESS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADDRESS_A::ENABLED
    }
}
impl core::ops::Deref for ADDRESS_R {
    type Target = crate::FieldReader<bool, ADDRESS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on ADDRESS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<ADDRESS_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS` writer - Disable interrupt on ADDRESS event."]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADDRESS_AW::CLEAR)
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
#[doc = "Disable interrupt on PAYLOAD event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAYLOAD_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<PAYLOAD_A> for bool {
    #[inline(always)]
    fn from(variant: PAYLOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAYLOAD` reader - Disable interrupt on PAYLOAD event."]
pub struct PAYLOAD_R(crate::FieldReader<bool, PAYLOAD_A>);
impl PAYLOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAYLOAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAYLOAD_A {
        match self.bits {
            false => PAYLOAD_A::DISABLED,
            true => PAYLOAD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PAYLOAD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PAYLOAD_A::ENABLED
    }
}
impl core::ops::Deref for PAYLOAD_R {
    type Target = crate::FieldReader<bool, PAYLOAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on PAYLOAD event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAYLOAD_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<PAYLOAD_AW> for bool {
    #[inline(always)]
    fn from(variant: PAYLOAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAYLOAD` writer - Disable interrupt on PAYLOAD event."]
pub struct PAYLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAYLOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAYLOAD_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PAYLOAD_AW::CLEAR)
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
#[doc = "Disable interrupt on END event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` reader - Disable interrupt on END event."]
pub struct END_R(crate::FieldReader<bool, END_A>);
impl END_R {
    pub(crate) fn new(bits: bool) -> Self {
        END_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_A {
        match self.bits {
            false => END_A::DISABLED,
            true => END_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == END_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == END_A::ENABLED
    }
}
impl core::ops::Deref for END_R {
    type Target = crate::FieldReader<bool, END_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on END event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Disable interrupt on END event."]
pub struct END_W<'a> {
    w: &'a mut W,
}
impl<'a> END_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(END_AW::CLEAR)
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
#[doc = "Disable interrupt on DISABLED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<DISABLED_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED` reader - Disable interrupt on DISABLED event."]
pub struct DISABLED_R(crate::FieldReader<bool, DISABLED_A>);
impl DISABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISABLED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_A {
        match self.bits {
            false => DISABLED_A::DISABLED,
            true => DISABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DISABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DISABLED_A::ENABLED
    }
}
impl core::ops::Deref for DISABLED_R {
    type Target = crate::FieldReader<bool, DISABLED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on DISABLED event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<DISABLED_AW> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED` writer - Disable interrupt on DISABLED event."]
pub struct DISABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DISABLED_AW::CLEAR)
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
#[doc = "Disable interrupt on DEVMATCH event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMATCH_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<DEVMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: DEVMATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMATCH` reader - Disable interrupt on DEVMATCH event."]
pub struct DEVMATCH_R(crate::FieldReader<bool, DEVMATCH_A>);
impl DEVMATCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEVMATCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVMATCH_A {
        match self.bits {
            false => DEVMATCH_A::DISABLED,
            true => DEVMATCH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DEVMATCH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DEVMATCH_A::ENABLED
    }
}
impl core::ops::Deref for DEVMATCH_R {
    type Target = crate::FieldReader<bool, DEVMATCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on DEVMATCH event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMATCH_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<DEVMATCH_AW> for bool {
    #[inline(always)]
    fn from(variant: DEVMATCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMATCH` writer - Disable interrupt on DEVMATCH event."]
pub struct DEVMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVMATCH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DEVMATCH_AW::CLEAR)
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
#[doc = "Disable interrupt on DEVMISS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMISS_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<DEVMISS_A> for bool {
    #[inline(always)]
    fn from(variant: DEVMISS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMISS` reader - Disable interrupt on DEVMISS event."]
pub struct DEVMISS_R(crate::FieldReader<bool, DEVMISS_A>);
impl DEVMISS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEVMISS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVMISS_A {
        match self.bits {
            false => DEVMISS_A::DISABLED,
            true => DEVMISS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DEVMISS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DEVMISS_A::ENABLED
    }
}
impl core::ops::Deref for DEVMISS_R {
    type Target = crate::FieldReader<bool, DEVMISS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on DEVMISS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMISS_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<DEVMISS_AW> for bool {
    #[inline(always)]
    fn from(variant: DEVMISS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMISS` writer - Disable interrupt on DEVMISS event."]
pub struct DEVMISS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVMISS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVMISS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DEVMISS_AW::CLEAR)
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
#[doc = "Disable interrupt on RSSIEND event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSIEND_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<RSSIEND_A> for bool {
    #[inline(always)]
    fn from(variant: RSSIEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSSIEND` reader - Disable interrupt on RSSIEND event."]
pub struct RSSIEND_R(crate::FieldReader<bool, RSSIEND_A>);
impl RSSIEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSSIEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSSIEND_A {
        match self.bits {
            false => RSSIEND_A::DISABLED,
            true => RSSIEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RSSIEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RSSIEND_A::ENABLED
    }
}
impl core::ops::Deref for RSSIEND_R {
    type Target = crate::FieldReader<bool, RSSIEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on RSSIEND event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSIEND_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<RSSIEND_AW> for bool {
    #[inline(always)]
    fn from(variant: RSSIEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSSIEND` writer - Disable interrupt on RSSIEND event."]
pub struct RSSIEND_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSIEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSIEND_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSSIEND_AW::CLEAR)
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
#[doc = "Disable interrupt on BCMATCH event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCMATCH_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<BCMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: BCMATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCMATCH` reader - Disable interrupt on BCMATCH event."]
pub struct BCMATCH_R(crate::FieldReader<bool, BCMATCH_A>);
impl BCMATCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCMATCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCMATCH_A {
        match self.bits {
            false => BCMATCH_A::DISABLED,
            true => BCMATCH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BCMATCH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BCMATCH_A::ENABLED
    }
}
impl core::ops::Deref for BCMATCH_R {
    type Target = crate::FieldReader<bool, BCMATCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on BCMATCH event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCMATCH_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<BCMATCH_AW> for bool {
    #[inline(always)]
    fn from(variant: BCMATCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCMATCH` writer - Disable interrupt on BCMATCH event."]
pub struct BCMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> BCMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCMATCH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCMATCH_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Disable interrupt on READY event."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on ADDRESS event."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable interrupt on PAYLOAD event."]
    #[inline(always)]
    pub fn payload(&self) -> PAYLOAD_R {
        PAYLOAD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable interrupt on END event."]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Disable interrupt on DISABLED event."]
    #[inline(always)]
    pub fn disabled(&self) -> DISABLED_R {
        DISABLED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable interrupt on DEVMATCH event."]
    #[inline(always)]
    pub fn devmatch(&self) -> DEVMATCH_R {
        DEVMATCH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable interrupt on DEVMISS event."]
    #[inline(always)]
    pub fn devmiss(&self) -> DEVMISS_R {
        DEVMISS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Disable interrupt on RSSIEND event."]
    #[inline(always)]
    pub fn rssiend(&self) -> RSSIEND_R {
        RSSIEND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Disable interrupt on BCMATCH event."]
    #[inline(always)]
    pub fn bcmatch(&self) -> BCMATCH_R {
        BCMATCH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on READY event."]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W { w: self }
    }
    #[doc = "Bit 1 - Disable interrupt on ADDRESS event."]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    #[doc = "Bit 2 - Disable interrupt on PAYLOAD event."]
    #[inline(always)]
    pub fn payload(&mut self) -> PAYLOAD_W {
        PAYLOAD_W { w: self }
    }
    #[doc = "Bit 3 - Disable interrupt on END event."]
    #[inline(always)]
    pub fn end(&mut self) -> END_W {
        END_W { w: self }
    }
    #[doc = "Bit 4 - Disable interrupt on DISABLED event."]
    #[inline(always)]
    pub fn disabled(&mut self) -> DISABLED_W {
        DISABLED_W { w: self }
    }
    #[doc = "Bit 5 - Disable interrupt on DEVMATCH event."]
    #[inline(always)]
    pub fn devmatch(&mut self) -> DEVMATCH_W {
        DEVMATCH_W { w: self }
    }
    #[doc = "Bit 6 - Disable interrupt on DEVMISS event."]
    #[inline(always)]
    pub fn devmiss(&mut self) -> DEVMISS_W {
        DEVMISS_W { w: self }
    }
    #[doc = "Bit 7 - Disable interrupt on RSSIEND event."]
    #[inline(always)]
    pub fn rssiend(&mut self) -> RSSIEND_W {
        RSSIEND_W { w: self }
    }
    #[doc = "Bit 10 - Disable interrupt on BCMATCH event."]
    #[inline(always)]
    pub fn bcmatch(&mut self) -> BCMATCH_W {
        BCMATCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable clear register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
