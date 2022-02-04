#[doc = "Register `DACNF` reader"]
pub struct R(crate::R<DACNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACNF` writer"]
pub struct W(crate::W<DACNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACNF_SPEC>;
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
impl From<crate::W<DACNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable or disable device address matching using device address 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA0_A> for bool {
    #[inline(always)]
    fn from(variant: ENA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA0` reader - Enable or disable device address matching using device address 0"]
pub struct ENA0_R(crate::FieldReader<bool, ENA0_A>);
impl ENA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA0_A {
        match self.bits {
            false => ENA0_A::DISABLED,
            true => ENA0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENA0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENA0_A::ENABLED
    }
}
impl core::ops::Deref for ENA0_R {
    type Target = crate::FieldReader<bool, ENA0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA0` writer - Enable or disable device address matching using device address 0"]
pub struct ENA0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA0_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA1_A> for bool {
    #[inline(always)]
    fn from(variant: ENA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA1` reader - Enable or disable device address matching using device address 1"]
pub struct ENA1_R(crate::FieldReader<bool, ENA1_A>);
impl ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA1_A {
        match self.bits {
            false => ENA1_A::DISABLED,
            true => ENA1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENA1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENA1_A::ENABLED
    }
}
impl core::ops::Deref for ENA1_R {
    type Target = crate::FieldReader<bool, ENA1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA1` writer - Enable or disable device address matching using device address 1"]
pub struct ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA1_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA2_A> for bool {
    #[inline(always)]
    fn from(variant: ENA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA2` reader - Enable or disable device address matching using device address 2"]
pub struct ENA2_R(crate::FieldReader<bool, ENA2_A>);
impl ENA2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA2_A {
        match self.bits {
            false => ENA2_A::DISABLED,
            true => ENA2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENA2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENA2_A::ENABLED
    }
}
impl core::ops::Deref for ENA2_R {
    type Target = crate::FieldReader<bool, ENA2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA2` writer - Enable or disable device address matching using device address 2"]
pub struct ENA2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA2_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA2_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA3_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA3_A> for bool {
    #[inline(always)]
    fn from(variant: ENA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA3` reader - Enable or disable device address matching using device address 3"]
pub struct ENA3_R(crate::FieldReader<bool, ENA3_A>);
impl ENA3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA3_A {
        match self.bits {
            false => ENA3_A::DISABLED,
            true => ENA3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENA3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENA3_A::ENABLED
    }
}
impl core::ops::Deref for ENA3_R {
    type Target = crate::FieldReader<bool, ENA3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA3` writer - Enable or disable device address matching using device address 3"]
pub struct ENA3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA3_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA3_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA4_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA4_A> for bool {
    #[inline(always)]
    fn from(variant: ENA4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA4` reader - Enable or disable device address matching using device address 4"]
pub struct ENA4_R(crate::FieldReader<bool, ENA4_A>);
impl ENA4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA4_A {
        match self.bits {
            false => ENA4_A::DISABLED,
            true => ENA4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENA4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENA4_A::ENABLED
    }
}
impl core::ops::Deref for ENA4_R {
    type Target = crate::FieldReader<bool, ENA4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA4` writer - Enable or disable device address matching using device address 4"]
pub struct ENA4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA4_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA4_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA5_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA5_A> for bool {
    #[inline(always)]
    fn from(variant: ENA5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA5` reader - Enable or disable device address matching using device address 5"]
pub struct ENA5_R(crate::FieldReader<bool, ENA5_A>);
impl ENA5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA5_A {
        match self.bits {
            false => ENA5_A::DISABLED,
            true => ENA5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENA5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENA5_A::ENABLED
    }
}
impl core::ops::Deref for ENA5_R {
    type Target = crate::FieldReader<bool, ENA5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA5` writer - Enable or disable device address matching using device address 5"]
pub struct ENA5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA5_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA5_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA6_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA6_A> for bool {
    #[inline(always)]
    fn from(variant: ENA6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA6` reader - Enable or disable device address matching using device address 6"]
pub struct ENA6_R(crate::FieldReader<bool, ENA6_A>);
impl ENA6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA6_A {
        match self.bits {
            false => ENA6_A::DISABLED,
            true => ENA6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENA6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENA6_A::ENABLED
    }
}
impl core::ops::Deref for ENA6_R {
    type Target = crate::FieldReader<bool, ENA6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA6` writer - Enable or disable device address matching using device address 6"]
pub struct ENA6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA6_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA6_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA7_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA7_A> for bool {
    #[inline(always)]
    fn from(variant: ENA7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA7` reader - Enable or disable device address matching using device address 7"]
pub struct ENA7_R(crate::FieldReader<bool, ENA7_A>);
impl ENA7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA7_A {
        match self.bits {
            false => ENA7_A::DISABLED,
            true => ENA7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENA7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENA7_A::ENABLED
    }
}
impl core::ops::Deref for ENA7_R {
    type Target = crate::FieldReader<bool, ENA7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA7` writer - Enable or disable device address matching using device address 7"]
pub struct ENA7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA7_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA7_A::ENABLED)
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
#[doc = "Field `TXADD0` reader - TxAdd for device address 0"]
pub struct TXADD0_R(crate::FieldReader<bool, bool>);
impl TXADD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXADD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXADD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXADD0` writer - TxAdd for device address 0"]
pub struct TXADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD0_W<'a> {
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
#[doc = "Field `TXADD1` reader - TxAdd for device address 1"]
pub struct TXADD1_R(crate::FieldReader<bool, bool>);
impl TXADD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXADD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXADD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXADD1` writer - TxAdd for device address 1"]
pub struct TXADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD1_W<'a> {
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
#[doc = "Field `TXADD2` reader - TxAdd for device address 2"]
pub struct TXADD2_R(crate::FieldReader<bool, bool>);
impl TXADD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXADD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXADD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXADD2` writer - TxAdd for device address 2"]
pub struct TXADD2_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD2_W<'a> {
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
#[doc = "Field `TXADD3` reader - TxAdd for device address 3"]
pub struct TXADD3_R(crate::FieldReader<bool, bool>);
impl TXADD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXADD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXADD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXADD3` writer - TxAdd for device address 3"]
pub struct TXADD3_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD3_W<'a> {
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
#[doc = "Field `TXADD4` reader - TxAdd for device address 4"]
pub struct TXADD4_R(crate::FieldReader<bool, bool>);
impl TXADD4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXADD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXADD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXADD4` writer - TxAdd for device address 4"]
pub struct TXADD4_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD4_W<'a> {
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
#[doc = "Field `TXADD5` reader - TxAdd for device address 5"]
pub struct TXADD5_R(crate::FieldReader<bool, bool>);
impl TXADD5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXADD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXADD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXADD5` writer - TxAdd for device address 5"]
pub struct TXADD5_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD5_W<'a> {
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
#[doc = "Field `TXADD6` reader - TxAdd for device address 6"]
pub struct TXADD6_R(crate::FieldReader<bool, bool>);
impl TXADD6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXADD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXADD6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXADD6` writer - TxAdd for device address 6"]
pub struct TXADD6_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD6_W<'a> {
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
#[doc = "Field `TXADD7` reader - TxAdd for device address 7"]
pub struct TXADD7_R(crate::FieldReader<bool, bool>);
impl TXADD7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXADD7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXADD7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXADD7` writer - TxAdd for device address 7"]
pub struct TXADD7_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD7_W<'a> {
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
    #[doc = "Bit 0 - Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub fn ena0(&self) -> ENA0_R {
        ENA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub fn ena1(&self) -> ENA1_R {
        ENA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub fn ena2(&self) -> ENA2_R {
        ENA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub fn ena3(&self) -> ENA3_R {
        ENA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub fn ena4(&self) -> ENA4_R {
        ENA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub fn ena5(&self) -> ENA5_R {
        ENA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub fn ena6(&self) -> ENA6_R {
        ENA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub fn ena7(&self) -> ENA7_R {
        ENA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TxAdd for device address 0"]
    #[inline(always)]
    pub fn txadd0(&self) -> TXADD0_R {
        TXADD0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TxAdd for device address 1"]
    #[inline(always)]
    pub fn txadd1(&self) -> TXADD1_R {
        TXADD1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TxAdd for device address 2"]
    #[inline(always)]
    pub fn txadd2(&self) -> TXADD2_R {
        TXADD2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TxAdd for device address 3"]
    #[inline(always)]
    pub fn txadd3(&self) -> TXADD3_R {
        TXADD3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TxAdd for device address 4"]
    #[inline(always)]
    pub fn txadd4(&self) -> TXADD4_R {
        TXADD4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TxAdd for device address 5"]
    #[inline(always)]
    pub fn txadd5(&self) -> TXADD5_R {
        TXADD5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TxAdd for device address 6"]
    #[inline(always)]
    pub fn txadd6(&self) -> TXADD6_R {
        TXADD6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TxAdd for device address 7"]
    #[inline(always)]
    pub fn txadd7(&self) -> TXADD7_R {
        TXADD7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub fn ena0(&mut self) -> ENA0_W {
        ENA0_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub fn ena1(&mut self) -> ENA1_W {
        ENA1_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub fn ena2(&mut self) -> ENA2_W {
        ENA2_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub fn ena3(&mut self) -> ENA3_W {
        ENA3_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub fn ena4(&mut self) -> ENA4_W {
        ENA4_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub fn ena5(&mut self) -> ENA5_W {
        ENA5_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub fn ena6(&mut self) -> ENA6_W {
        ENA6_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub fn ena7(&mut self) -> ENA7_W {
        ENA7_W { w: self }
    }
    #[doc = "Bit 8 - TxAdd for device address 0"]
    #[inline(always)]
    pub fn txadd0(&mut self) -> TXADD0_W {
        TXADD0_W { w: self }
    }
    #[doc = "Bit 9 - TxAdd for device address 1"]
    #[inline(always)]
    pub fn txadd1(&mut self) -> TXADD1_W {
        TXADD1_W { w: self }
    }
    #[doc = "Bit 10 - TxAdd for device address 2"]
    #[inline(always)]
    pub fn txadd2(&mut self) -> TXADD2_W {
        TXADD2_W { w: self }
    }
    #[doc = "Bit 11 - TxAdd for device address 3"]
    #[inline(always)]
    pub fn txadd3(&mut self) -> TXADD3_W {
        TXADD3_W { w: self }
    }
    #[doc = "Bit 12 - TxAdd for device address 4"]
    #[inline(always)]
    pub fn txadd4(&mut self) -> TXADD4_W {
        TXADD4_W { w: self }
    }
    #[doc = "Bit 13 - TxAdd for device address 5"]
    #[inline(always)]
    pub fn txadd5(&mut self) -> TXADD5_W {
        TXADD5_W { w: self }
    }
    #[doc = "Bit 14 - TxAdd for device address 6"]
    #[inline(always)]
    pub fn txadd6(&mut self) -> TXADD6_W {
        TXADD6_W { w: self }
    }
    #[doc = "Bit 15 - TxAdd for device address 7"]
    #[inline(always)]
    pub fn txadd7(&mut self) -> TXADD7_W {
        TXADD7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device address match configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacnf](index.html) module"]
pub struct DACNF_SPEC;
impl crate::RegisterSpec for DACNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacnf::R](R) reader structure"]
impl crate::Readable for DACNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacnf::W](W) writer structure"]
impl crate::Writable for DACNF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACNF to value 0"]
impl crate::Resettable for DACNF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
