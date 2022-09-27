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
pub type APPULSE_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTIAPPPULSE_SPEC, APPULSE_0_AW, O>;
impl<'a, const O: u8> APPULSE_0_W<'a, O> {
    #[doc = "Generates an event pulse on channel 0."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_0_AW::GENERATE)
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
pub type APPULSE_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTIAPPPULSE_SPEC, APPULSE_1_AW, O>;
impl<'a, const O: u8> APPULSE_1_W<'a, O> {
    #[doc = "Generates an event pulse on channel 1."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_1_AW::GENERATE)
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
pub type APPULSE_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTIAPPPULSE_SPEC, APPULSE_2_AW, O>;
impl<'a, const O: u8> APPULSE_2_W<'a, O> {
    #[doc = "Generates an event pulse on channel 2."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_2_AW::GENERATE)
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
pub type APPULSE_3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTIAPPPULSE_SPEC, APPULSE_3_AW, O>;
impl<'a, const O: u8> APPULSE_3_W<'a, O> {
    #[doc = "Generates an event pulse on channel 3."]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_3_AW::GENERATE)
    }
}
impl W {
    #[doc = "Bit 0 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_0(&mut self) -> APPULSE_0_W<0> {
        APPULSE_0_W::new(self)
    }
    #[doc = "Bit 1 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_1(&mut self) -> APPULSE_1_W<1> {
        APPULSE_1_W::new(self)
    }
    #[doc = "Bit 2 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_2(&mut self) -> APPULSE_2_W<2> {
        APPULSE_2_W::new(self)
    }
    #[doc = "Bit 3 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_3(&mut self) -> APPULSE_3_W<3> {
        APPULSE_3_W::new(self)
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
