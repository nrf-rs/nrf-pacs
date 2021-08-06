#[doc = "Register `CTIAPPPULSE` writer"]
pub struct W(crate::W<CTIAPPPULSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIAPPPULSE_SPEC>;
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
impl From<crate::W<CTIAPPPULSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIAPPPULSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPULSE_0_AW {
    #[doc = "1: Generates an event pulse on channel 0."]
    GENERATE = 1,
}
impl From<APPULSE_0_AW> for bool {
    #[inline(always)]
    fn from(variant: APPULSE_0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPULSE_0` writer - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
pub struct APPULSE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APPULSE_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPULSE_0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an event pulse on channel 0."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_0_AW::GENERATE)
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
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPULSE_1_AW {
    #[doc = "1: Generates an event pulse on channel 1."]
    GENERATE = 1,
}
impl From<APPULSE_1_AW> for bool {
    #[inline(always)]
    fn from(variant: APPULSE_1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPULSE_1` writer - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
pub struct APPULSE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APPULSE_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPULSE_1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an event pulse on channel 1."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_1_AW::GENERATE)
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
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPULSE_2_AW {
    #[doc = "1: Generates an event pulse on channel 2."]
    GENERATE = 1,
}
impl From<APPULSE_2_AW> for bool {
    #[inline(always)]
    fn from(variant: APPULSE_2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPULSE_2` writer - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
pub struct APPULSE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> APPULSE_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPULSE_2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an event pulse on channel 2."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_2_AW::GENERATE)
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
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPULSE_3_AW {
    #[doc = "1: Generates an event pulse on channel 3."]
    GENERATE = 1,
}
impl From<APPULSE_3_AW> for bool {
    #[inline(always)]
    fn from(variant: APPULSE_3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPULSE_3` writer - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
pub struct APPULSE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> APPULSE_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPULSE_3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an event pulse on channel 3."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_3_AW::GENERATE)
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
impl W {
    #[doc = "Bit 0 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_0(&mut self) -> APPULSE_0_W {
        APPULSE_0_W { w: self }
    }
    #[doc = "Bit 1 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_1(&mut self) -> APPULSE_1_W {
        APPULSE_1_W { w: self }
    }
    #[doc = "Bit 2 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_2(&mut self) -> APPULSE_2_W {
        APPULSE_2_W { w: self }
    }
    #[doc = "Bit 3 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_3(&mut self) -> APPULSE_3_W {
        APPULSE_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTI Application Pulse register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctiapppulse](index.html) module"]
pub struct CTIAPPPULSE_SPEC;
impl crate::RegisterSpec for CTIAPPPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ctiapppulse::W](W) writer structure"]
impl crate::Writable for CTIAPPPULSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIAPPPULSE to value 0"]
impl crate::Resettable for CTIAPPPULSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
