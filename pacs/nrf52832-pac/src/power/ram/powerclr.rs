#[doc = "Register `POWERCLR` writer"]
pub struct W(crate::W<POWERCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWERCLR_SPEC>;
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
impl From<crate::W<POWERCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWERCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Keep RAM section S0 of RAM0 on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S0POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S0POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0POWER` writer - Keep RAM section S0 of RAM0 on or off in System ON mode"]
pub struct S0POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S0POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0POWER_AW::OFF)
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
#[doc = "Keep RAM section S1 of RAM0 on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S1POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S1POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1POWER` writer - Keep RAM section S1 of RAM0 on or off in System ON mode"]
pub struct S1POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S1POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1POWER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1POWER_AW::OFF)
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
#[doc = "Keep retention on RAM section S0 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
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
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0RETENTION_AW::OFF)
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
    #[doc = "1: Off"]
    OFF = 1,
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
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1RETENTION_AW::OFF)
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
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 of RAM0 on or off in System ON mode"]
    #[inline(always)]
    pub fn s0power(&mut self) -> S0POWER_W {
        S0POWER_W { w: self }
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAM0 on or off in System ON mode"]
    #[inline(always)]
    pub fn s1power(&mut self) -> S1POWER_W {
        S1POWER_W { w: self }
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster\\[0\\]: RAM0 power control clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerclr](index.html) module"]
pub struct POWERCLR_SPEC;
impl crate::RegisterSpec for POWERCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [powerclr::W](W) writer structure"]
impl crate::Writable for POWERCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWERCLR to value 0xffff"]
impl crate::Resettable for POWERCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
