#[doc = "Register `POWERSET` writer"]
pub struct W(crate::W<POWERSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWERSET_SPEC>;
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
impl From<crate::W<POWERSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWERSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Keep RAM section S0 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S0POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S0POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0POWER` writer - Keep RAM section S0 of RAMn on or off in System ON mode"]
pub struct S0POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S0POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S0POWER_AW::ON)
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
#[doc = "Keep RAM section S1 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S1POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S1POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1POWER` writer - Keep RAM section S1 of RAMn on or off in System ON mode"]
pub struct S1POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S1POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S1POWER_AW::ON)
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
#[doc = "Keep RAM section S2 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S2POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S2POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2POWER` writer - Keep RAM section S2 of RAMn on or off in System ON mode"]
pub struct S2POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S2POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S2POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S2POWER_AW::ON)
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
#[doc = "Keep RAM section S3 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S3POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S3POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3POWER` writer - Keep RAM section S3 of RAMn on or off in System ON mode"]
pub struct S3POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S3POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S3POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S3POWER_AW::ON)
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
#[doc = "Keep RAM section S4 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S4POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S4POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4POWER` writer - Keep RAM section S4 of RAMn on or off in System ON mode"]
pub struct S4POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S4POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S4POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S4POWER_AW::ON)
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
#[doc = "Keep RAM section S5 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S5POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S5POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5POWER` writer - Keep RAM section S5 of RAMn on or off in System ON mode"]
pub struct S5POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S5POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S5POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S5POWER_AW::ON)
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
#[doc = "Keep RAM section S6 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S6POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S6POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6POWER` writer - Keep RAM section S6 of RAMn on or off in System ON mode"]
pub struct S6POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S6POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S6POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S6POWER_AW::ON)
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
#[doc = "Keep RAM section S7 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S7POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S7POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7POWER` writer - Keep RAM section S7 of RAMn on or off in System ON mode"]
pub struct S7POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S7POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S7POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S7POWER_AW::ON)
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
#[doc = "Keep RAM section S8 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S8POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S8POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8POWER` writer - Keep RAM section S8 of RAMn on or off in System ON mode"]
pub struct S8POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S8POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S8POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S8POWER_AW::ON)
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
#[doc = "Keep RAM section S9 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S9POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S9POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S9POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S9POWER` writer - Keep RAM section S9 of RAMn on or off in System ON mode"]
pub struct S9POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S9POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S9POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S9POWER_AW::ON)
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
#[doc = "Keep RAM section S10 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S10POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S10POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S10POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S10POWER` writer - Keep RAM section S10 of RAMn on or off in System ON mode"]
pub struct S10POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S10POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S10POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S10POWER_AW::ON)
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
#[doc = "Keep RAM section S11 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S11POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S11POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S11POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S11POWER` writer - Keep RAM section S11 of RAMn on or off in System ON mode"]
pub struct S11POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S11POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S11POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S11POWER_AW::ON)
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
#[doc = "Keep RAM section S12 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S12POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S12POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S12POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S12POWER` writer - Keep RAM section S12 of RAMn on or off in System ON mode"]
pub struct S12POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S12POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S12POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S12POWER_AW::ON)
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
#[doc = "Keep RAM section S13 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S13POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S13POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S13POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S13POWER` writer - Keep RAM section S13 of RAMn on or off in System ON mode"]
pub struct S13POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S13POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S13POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S13POWER_AW::ON)
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
#[doc = "Keep RAM section S14 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S14POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S14POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S14POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S14POWER` writer - Keep RAM section S14 of RAMn on or off in System ON mode"]
pub struct S14POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S14POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S14POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S14POWER_AW::ON)
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
#[doc = "Keep RAM section S15 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S15POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S15POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S15POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S15POWER` writer - Keep RAM section S15 of RAMn on or off in System ON mode"]
pub struct S15POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S15POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S15POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S15POWER_AW::ON)
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
#[doc = "Keep retention on RAM section S0 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S0RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S0RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0RETENTION` writer - Keep retention on RAM section S0 when RAM section is switched off"]
pub struct S0RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S0RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S0RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S1 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S1RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S1RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1RETENTION` writer - Keep retention on RAM section S1 when RAM section is switched off"]
pub struct S1RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S1RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S1RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S2 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S2RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S2RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2RETENTION` writer - Keep retention on RAM section S2 when RAM section is switched off"]
pub struct S2RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S2RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S2RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S2RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S3 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S3RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S3RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3RETENTION` writer - Keep retention on RAM section S3 when RAM section is switched off"]
pub struct S3RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S3RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S3RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S3RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S4 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S4RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S4RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S4RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4RETENTION` writer - Keep retention on RAM section S4 when RAM section is switched off"]
pub struct S4RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S4RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S4RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S4RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S5 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S5RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S5RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S5RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5RETENTION` writer - Keep retention on RAM section S5 when RAM section is switched off"]
pub struct S5RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S5RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S5RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S5RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S6 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S6RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S6RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S6RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6RETENTION` writer - Keep retention on RAM section S6 when RAM section is switched off"]
pub struct S6RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S6RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S6RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S6RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S7 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S7RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S7RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S7RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7RETENTION` writer - Keep retention on RAM section S7 when RAM section is switched off"]
pub struct S7RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S7RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S7RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S7RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S8 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S8RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S8RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S8RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8RETENTION` writer - Keep retention on RAM section S8 when RAM section is switched off"]
pub struct S8RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S8RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S8RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S8RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S9 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S9RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S9RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S9RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S9RETENTION` writer - Keep retention on RAM section S9 when RAM section is switched off"]
pub struct S9RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S9RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S9RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S9RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S10 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S10RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S10RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S10RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S10RETENTION` writer - Keep retention on RAM section S10 when RAM section is switched off"]
pub struct S10RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S10RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S10RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S10RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S11 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S11RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S11RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S11RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S11RETENTION` writer - Keep retention on RAM section S11 when RAM section is switched off"]
pub struct S11RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S11RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S11RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S11RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S12 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S12RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S12RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S12RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S12RETENTION` writer - Keep retention on RAM section S12 when RAM section is switched off"]
pub struct S12RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S12RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S12RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S12RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S13 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S13RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S13RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S13RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S13RETENTION` writer - Keep retention on RAM section S13 when RAM section is switched off"]
pub struct S13RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S13RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S13RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S13RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S14 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S14RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S14RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S14RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S14RETENTION` writer - Keep retention on RAM section S14 when RAM section is switched off"]
pub struct S14RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S14RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S14RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S14RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S15 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S15RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S15RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S15RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S15RETENTION` writer - Keep retention on RAM section S15 when RAM section is switched off"]
pub struct S15RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S15RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S15RETENTION_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S15RETENTION_AW::ON)
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
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s0power(&mut self) -> S0POWER_W {
        S0POWER_W { w: self }
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s1power(&mut self) -> S1POWER_W {
        S1POWER_W { w: self }
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s2power(&mut self) -> S2POWER_W {
        S2POWER_W { w: self }
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s3power(&mut self) -> S3POWER_W {
        S3POWER_W { w: self }
    }
    #[doc = "Bit 4 - Keep RAM section S4 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s4power(&mut self) -> S4POWER_W {
        S4POWER_W { w: self }
    }
    #[doc = "Bit 5 - Keep RAM section S5 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s5power(&mut self) -> S5POWER_W {
        S5POWER_W { w: self }
    }
    #[doc = "Bit 6 - Keep RAM section S6 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s6power(&mut self) -> S6POWER_W {
        S6POWER_W { w: self }
    }
    #[doc = "Bit 7 - Keep RAM section S7 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s7power(&mut self) -> S7POWER_W {
        S7POWER_W { w: self }
    }
    #[doc = "Bit 8 - Keep RAM section S8 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s8power(&mut self) -> S8POWER_W {
        S8POWER_W { w: self }
    }
    #[doc = "Bit 9 - Keep RAM section S9 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s9power(&mut self) -> S9POWER_W {
        S9POWER_W { w: self }
    }
    #[doc = "Bit 10 - Keep RAM section S10 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s10power(&mut self) -> S10POWER_W {
        S10POWER_W { w: self }
    }
    #[doc = "Bit 11 - Keep RAM section S11 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s11power(&mut self) -> S11POWER_W {
        S11POWER_W { w: self }
    }
    #[doc = "Bit 12 - Keep RAM section S12 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s12power(&mut self) -> S12POWER_W {
        S12POWER_W { w: self }
    }
    #[doc = "Bit 13 - Keep RAM section S13 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s13power(&mut self) -> S13POWER_W {
        S13POWER_W { w: self }
    }
    #[doc = "Bit 14 - Keep RAM section S14 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s14power(&mut self) -> S14POWER_W {
        S14POWER_W { w: self }
    }
    #[doc = "Bit 15 - Keep RAM section S15 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn s15power(&mut self) -> S15POWER_W {
        S15POWER_W { w: self }
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is switched off"]
    #[inline(always)]
    pub fn s0retention(&mut self) -> S0RETENTION_W {
        S0RETENTION_W { w: self }
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is switched off"]
    #[inline(always)]
    pub fn s1retention(&mut self) -> S1RETENTION_W {
        S1RETENTION_W { w: self }
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 when RAM section is switched off"]
    #[inline(always)]
    pub fn s2retention(&mut self) -> S2RETENTION_W {
        S2RETENTION_W { w: self }
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 when RAM section is switched off"]
    #[inline(always)]
    pub fn s3retention(&mut self) -> S3RETENTION_W {
        S3RETENTION_W { w: self }
    }
    #[doc = "Bit 20 - Keep retention on RAM section S4 when RAM section is switched off"]
    #[inline(always)]
    pub fn s4retention(&mut self) -> S4RETENTION_W {
        S4RETENTION_W { w: self }
    }
    #[doc = "Bit 21 - Keep retention on RAM section S5 when RAM section is switched off"]
    #[inline(always)]
    pub fn s5retention(&mut self) -> S5RETENTION_W {
        S5RETENTION_W { w: self }
    }
    #[doc = "Bit 22 - Keep retention on RAM section S6 when RAM section is switched off"]
    #[inline(always)]
    pub fn s6retention(&mut self) -> S6RETENTION_W {
        S6RETENTION_W { w: self }
    }
    #[doc = "Bit 23 - Keep retention on RAM section S7 when RAM section is switched off"]
    #[inline(always)]
    pub fn s7retention(&mut self) -> S7RETENTION_W {
        S7RETENTION_W { w: self }
    }
    #[doc = "Bit 24 - Keep retention on RAM section S8 when RAM section is switched off"]
    #[inline(always)]
    pub fn s8retention(&mut self) -> S8RETENTION_W {
        S8RETENTION_W { w: self }
    }
    #[doc = "Bit 25 - Keep retention on RAM section S9 when RAM section is switched off"]
    #[inline(always)]
    pub fn s9retention(&mut self) -> S9RETENTION_W {
        S9RETENTION_W { w: self }
    }
    #[doc = "Bit 26 - Keep retention on RAM section S10 when RAM section is switched off"]
    #[inline(always)]
    pub fn s10retention(&mut self) -> S10RETENTION_W {
        S10RETENTION_W { w: self }
    }
    #[doc = "Bit 27 - Keep retention on RAM section S11 when RAM section is switched off"]
    #[inline(always)]
    pub fn s11retention(&mut self) -> S11RETENTION_W {
        S11RETENTION_W { w: self }
    }
    #[doc = "Bit 28 - Keep retention on RAM section S12 when RAM section is switched off"]
    #[inline(always)]
    pub fn s12retention(&mut self) -> S12RETENTION_W {
        S12RETENTION_W { w: self }
    }
    #[doc = "Bit 29 - Keep retention on RAM section S13 when RAM section is switched off"]
    #[inline(always)]
    pub fn s13retention(&mut self) -> S13RETENTION_W {
        S13RETENTION_W { w: self }
    }
    #[doc = "Bit 30 - Keep retention on RAM section S14 when RAM section is switched off"]
    #[inline(always)]
    pub fn s14retention(&mut self) -> S14RETENTION_W {
        S14RETENTION_W { w: self }
    }
    #[doc = "Bit 31 - Keep retention on RAM section S15 when RAM section is switched off"]
    #[inline(always)]
    pub fn s15retention(&mut self) -> S15RETENTION_W {
        S15RETENTION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: RAMn power control set register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerset](index.html) module"]
pub struct POWERSET_SPEC;
impl crate::RegisterSpec for POWERSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [powerset::W](W) writer structure"]
impl crate::Writable for POWERSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWERSET to value 0xffff"]
impl crate::Resettable for POWERSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
