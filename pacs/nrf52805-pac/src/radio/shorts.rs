#[doc = "Register `SHORTS` reader"]
pub struct R(crate::R<SHORTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORTS` writer"]
pub struct W(crate::W<SHORTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORTS_SPEC>;
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
impl From<crate::W<SHORTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shortcut between event READY and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READY_START_A> for bool {
    #[inline(always)]
    fn from(variant: READY_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY_START` reader - Shortcut between event READY and task START"]
pub struct READY_START_R(crate::FieldReader<bool, READY_START_A>);
impl READY_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        READY_START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_START_A {
        match self.bits {
            false => READY_START_A::DISABLED,
            true => READY_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == READY_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == READY_START_A::ENABLED
    }
}
impl core::ops::Deref for READY_START_R {
    type Target = crate::FieldReader<bool, READY_START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READY_START` writer - Shortcut between event READY and task START"]
pub struct READY_START_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_START_A::ENABLED)
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
#[doc = "Shortcut between event END and task DISABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_DISABLE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<END_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: END_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END_DISABLE` reader - Shortcut between event END and task DISABLE"]
pub struct END_DISABLE_R(crate::FieldReader<bool, END_DISABLE_A>);
impl END_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        END_DISABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_DISABLE_A {
        match self.bits {
            false => END_DISABLE_A::DISABLED,
            true => END_DISABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == END_DISABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == END_DISABLE_A::ENABLED
    }
}
impl core::ops::Deref for END_DISABLE_R {
    type Target = crate::FieldReader<bool, END_DISABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_DISABLE` writer - Shortcut between event END and task DISABLE"]
pub struct END_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> END_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_DISABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_DISABLE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_DISABLE_A::ENABLED)
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
#[doc = "Shortcut between event DISABLED and task TXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_TXEN_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_TXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED_TXEN` reader - Shortcut between event DISABLED and task TXEN"]
pub struct DISABLED_TXEN_R(crate::FieldReader<bool, DISABLED_TXEN_A>);
impl DISABLED_TXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISABLED_TXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_TXEN_A {
        match self.bits {
            false => DISABLED_TXEN_A::DISABLED,
            true => DISABLED_TXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DISABLED_TXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DISABLED_TXEN_A::ENABLED
    }
}
impl core::ops::Deref for DISABLED_TXEN_R {
    type Target = crate::FieldReader<bool, DISABLED_TXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLED_TXEN` writer - Shortcut between event DISABLED and task TXEN"]
pub struct DISABLED_TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_TXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_TXEN_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_TXEN_A::ENABLED)
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
#[doc = "Shortcut between event DISABLED and task RXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_RXEN_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_RXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED_RXEN` reader - Shortcut between event DISABLED and task RXEN"]
pub struct DISABLED_RXEN_R(crate::FieldReader<bool, DISABLED_RXEN_A>);
impl DISABLED_RXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISABLED_RXEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_RXEN_A {
        match self.bits {
            false => DISABLED_RXEN_A::DISABLED,
            true => DISABLED_RXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DISABLED_RXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DISABLED_RXEN_A::ENABLED
    }
}
impl core::ops::Deref for DISABLED_RXEN_R {
    type Target = crate::FieldReader<bool, DISABLED_RXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLED_RXEN` writer - Shortcut between event DISABLED and task RXEN"]
pub struct DISABLED_RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_RXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_RXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_RXEN_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_RXEN_A::ENABLED)
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
#[doc = "Shortcut between event ADDRESS and task RSSISTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_RSSISTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ADDRESS_RSSISTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_RSSISTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS_RSSISTART` reader - Shortcut between event ADDRESS and task RSSISTART"]
pub struct ADDRESS_RSSISTART_R(crate::FieldReader<bool, ADDRESS_RSSISTART_A>);
impl ADDRESS_RSSISTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRESS_RSSISTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_RSSISTART_A {
        match self.bits {
            false => ADDRESS_RSSISTART_A::DISABLED,
            true => ADDRESS_RSSISTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADDRESS_RSSISTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADDRESS_RSSISTART_A::ENABLED
    }
}
impl core::ops::Deref for ADDRESS_RSSISTART_R {
    type Target = crate::FieldReader<bool, ADDRESS_RSSISTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESS_RSSISTART` writer - Shortcut between event ADDRESS and task RSSISTART"]
pub struct ADDRESS_RSSISTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_RSSISTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_RSSISTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS_RSSISTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS_RSSISTART_A::ENABLED)
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
#[doc = "Shortcut between event END and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<END_START_A> for bool {
    #[inline(always)]
    fn from(variant: END_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END_START` reader - Shortcut between event END and task START"]
pub struct END_START_R(crate::FieldReader<bool, END_START_A>);
impl END_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        END_START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_START_A {
        match self.bits {
            false => END_START_A::DISABLED,
            true => END_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == END_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == END_START_A::ENABLED
    }
}
impl core::ops::Deref for END_START_R {
    type Target = crate::FieldReader<bool, END_START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_START` writer - Shortcut between event END and task START"]
pub struct END_START_W<'a> {
    w: &'a mut W,
}
impl<'a> END_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_START_A::ENABLED)
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
#[doc = "Shortcut between event ADDRESS and task BCSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_BCSTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ADDRESS_BCSTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_BCSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS_BCSTART` reader - Shortcut between event ADDRESS and task BCSTART"]
pub struct ADDRESS_BCSTART_R(crate::FieldReader<bool, ADDRESS_BCSTART_A>);
impl ADDRESS_BCSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRESS_BCSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_BCSTART_A {
        match self.bits {
            false => ADDRESS_BCSTART_A::DISABLED,
            true => ADDRESS_BCSTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADDRESS_BCSTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADDRESS_BCSTART_A::ENABLED
    }
}
impl core::ops::Deref for ADDRESS_BCSTART_R {
    type Target = crate::FieldReader<bool, ADDRESS_BCSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESS_BCSTART` writer - Shortcut between event ADDRESS and task BCSTART"]
pub struct ADDRESS_BCSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_BCSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_BCSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS_BCSTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS_BCSTART_A::ENABLED)
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
#[doc = "Shortcut between event DISABLED and task RSSISTOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_RSSISTOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_RSSISTOP_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_RSSISTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED_RSSISTOP` reader - Shortcut between event DISABLED and task RSSISTOP"]
pub struct DISABLED_RSSISTOP_R(crate::FieldReader<bool, DISABLED_RSSISTOP_A>);
impl DISABLED_RSSISTOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISABLED_RSSISTOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_RSSISTOP_A {
        match self.bits {
            false => DISABLED_RSSISTOP_A::DISABLED,
            true => DISABLED_RSSISTOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DISABLED_RSSISTOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DISABLED_RSSISTOP_A::ENABLED
    }
}
impl core::ops::Deref for DISABLED_RSSISTOP_R {
    type Target = crate::FieldReader<bool, DISABLED_RSSISTOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLED_RSSISTOP` writer - Shortcut between event DISABLED and task RSSISTOP"]
pub struct DISABLED_RSSISTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_RSSISTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_RSSISTOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_RSSISTOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_RSSISTOP_A::ENABLED)
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
#[doc = "Shortcut between event TXREADY and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXREADY_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<TXREADY_START_A> for bool {
    #[inline(always)]
    fn from(variant: TXREADY_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXREADY_START` reader - Shortcut between event TXREADY and task START"]
pub struct TXREADY_START_R(crate::FieldReader<bool, TXREADY_START_A>);
impl TXREADY_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXREADY_START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXREADY_START_A {
        match self.bits {
            false => TXREADY_START_A::DISABLED,
            true => TXREADY_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXREADY_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXREADY_START_A::ENABLED
    }
}
impl core::ops::Deref for TXREADY_START_R {
    type Target = crate::FieldReader<bool, TXREADY_START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXREADY_START` writer - Shortcut between event TXREADY and task START"]
pub struct TXREADY_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TXREADY_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXREADY_START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXREADY_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXREADY_START_A::ENABLED)
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
#[doc = "Shortcut between event RXREADY and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXREADY_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<RXREADY_START_A> for bool {
    #[inline(always)]
    fn from(variant: RXREADY_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXREADY_START` reader - Shortcut between event RXREADY and task START"]
pub struct RXREADY_START_R(crate::FieldReader<bool, RXREADY_START_A>);
impl RXREADY_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXREADY_START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXREADY_START_A {
        match self.bits {
            false => RXREADY_START_A::DISABLED,
            true => RXREADY_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXREADY_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXREADY_START_A::ENABLED
    }
}
impl core::ops::Deref for RXREADY_START_R {
    type Target = crate::FieldReader<bool, RXREADY_START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXREADY_START` writer - Shortcut between event RXREADY and task START"]
pub struct RXREADY_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RXREADY_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXREADY_START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXREADY_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXREADY_START_A::ENABLED)
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
#[doc = "Shortcut between event PHYEND and task DISABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYEND_DISABLE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<PHYEND_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PHYEND_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYEND_DISABLE` reader - Shortcut between event PHYEND and task DISABLE"]
pub struct PHYEND_DISABLE_R(crate::FieldReader<bool, PHYEND_DISABLE_A>);
impl PHYEND_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYEND_DISABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYEND_DISABLE_A {
        match self.bits {
            false => PHYEND_DISABLE_A::DISABLED,
            true => PHYEND_DISABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PHYEND_DISABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PHYEND_DISABLE_A::ENABLED
    }
}
impl core::ops::Deref for PHYEND_DISABLE_R {
    type Target = crate::FieldReader<bool, PHYEND_DISABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYEND_DISABLE` writer - Shortcut between event PHYEND and task DISABLE"]
pub struct PHYEND_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYEND_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHYEND_DISABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PHYEND_DISABLE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PHYEND_DISABLE_A::ENABLED)
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
#[doc = "Shortcut between event PHYEND and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYEND_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<PHYEND_START_A> for bool {
    #[inline(always)]
    fn from(variant: PHYEND_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYEND_START` reader - Shortcut between event PHYEND and task START"]
pub struct PHYEND_START_R(crate::FieldReader<bool, PHYEND_START_A>);
impl PHYEND_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYEND_START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYEND_START_A {
        match self.bits {
            false => PHYEND_START_A::DISABLED,
            true => PHYEND_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PHYEND_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PHYEND_START_A::ENABLED
    }
}
impl core::ops::Deref for PHYEND_START_R {
    type Target = crate::FieldReader<bool, PHYEND_START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYEND_START` writer - Shortcut between event PHYEND and task START"]
pub struct PHYEND_START_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYEND_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHYEND_START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PHYEND_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PHYEND_START_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Shortcut between event READY and task START"]
    #[inline(always)]
    pub fn ready_start(&self) -> READY_START_R {
        READY_START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event END and task DISABLE"]
    #[inline(always)]
    pub fn end_disable(&self) -> END_DISABLE_R {
        END_DISABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event DISABLED and task TXEN"]
    #[inline(always)]
    pub fn disabled_txen(&self) -> DISABLED_TXEN_R {
        DISABLED_TXEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event DISABLED and task RXEN"]
    #[inline(always)]
    pub fn disabled_rxen(&self) -> DISABLED_RXEN_R {
        DISABLED_RXEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event ADDRESS and task RSSISTART"]
    #[inline(always)]
    pub fn address_rssistart(&self) -> ADDRESS_RSSISTART_R {
        ADDRESS_RSSISTART_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event END and task START"]
    #[inline(always)]
    pub fn end_start(&self) -> END_START_R {
        END_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event ADDRESS and task BCSTART"]
    #[inline(always)]
    pub fn address_bcstart(&self) -> ADDRESS_BCSTART_R {
        ADDRESS_BCSTART_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Shortcut between event DISABLED and task RSSISTOP"]
    #[inline(always)]
    pub fn disabled_rssistop(&self) -> DISABLED_RSSISTOP_R {
        DISABLED_RSSISTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Shortcut between event TXREADY and task START"]
    #[inline(always)]
    pub fn txready_start(&self) -> TXREADY_START_R {
        TXREADY_START_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Shortcut between event RXREADY and task START"]
    #[inline(always)]
    pub fn rxready_start(&self) -> RXREADY_START_R {
        RXREADY_START_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Shortcut between event PHYEND and task DISABLE"]
    #[inline(always)]
    pub fn phyend_disable(&self) -> PHYEND_DISABLE_R {
        PHYEND_DISABLE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Shortcut between event PHYEND and task START"]
    #[inline(always)]
    pub fn phyend_start(&self) -> PHYEND_START_R {
        PHYEND_START_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event READY and task START"]
    #[inline(always)]
    pub fn ready_start(&mut self) -> READY_START_W {
        READY_START_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between event END and task DISABLE"]
    #[inline(always)]
    pub fn end_disable(&mut self) -> END_DISABLE_W {
        END_DISABLE_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between event DISABLED and task TXEN"]
    #[inline(always)]
    pub fn disabled_txen(&mut self) -> DISABLED_TXEN_W {
        DISABLED_TXEN_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between event DISABLED and task RXEN"]
    #[inline(always)]
    pub fn disabled_rxen(&mut self) -> DISABLED_RXEN_W {
        DISABLED_RXEN_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between event ADDRESS and task RSSISTART"]
    #[inline(always)]
    pub fn address_rssistart(&mut self) -> ADDRESS_RSSISTART_W {
        ADDRESS_RSSISTART_W { w: self }
    }
    #[doc = "Bit 5 - Shortcut between event END and task START"]
    #[inline(always)]
    pub fn end_start(&mut self) -> END_START_W {
        END_START_W { w: self }
    }
    #[doc = "Bit 6 - Shortcut between event ADDRESS and task BCSTART"]
    #[inline(always)]
    pub fn address_bcstart(&mut self) -> ADDRESS_BCSTART_W {
        ADDRESS_BCSTART_W { w: self }
    }
    #[doc = "Bit 8 - Shortcut between event DISABLED and task RSSISTOP"]
    #[inline(always)]
    pub fn disabled_rssistop(&mut self) -> DISABLED_RSSISTOP_W {
        DISABLED_RSSISTOP_W { w: self }
    }
    #[doc = "Bit 18 - Shortcut between event TXREADY and task START"]
    #[inline(always)]
    pub fn txready_start(&mut self) -> TXREADY_START_W {
        TXREADY_START_W { w: self }
    }
    #[doc = "Bit 19 - Shortcut between event RXREADY and task START"]
    #[inline(always)]
    pub fn rxready_start(&mut self) -> RXREADY_START_W {
        RXREADY_START_W { w: self }
    }
    #[doc = "Bit 20 - Shortcut between event PHYEND and task DISABLE"]
    #[inline(always)]
    pub fn phyend_disable(&mut self) -> PHYEND_DISABLE_W {
        PHYEND_DISABLE_W { w: self }
    }
    #[doc = "Bit 21 - Shortcut between event PHYEND and task START"]
    #[inline(always)]
    pub fn phyend_start(&mut self) -> PHYEND_START_W {
        PHYEND_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
pub struct SHORTS_SPEC;
impl crate::RegisterSpec for SHORTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shorts::R](R) reader structure"]
impl crate::Readable for SHORTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shorts::W](W) writer structure"]
impl crate::Writable for SHORTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for SHORTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
