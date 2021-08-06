#[doc = "Register `CLOCKSTART` writer"]
pub struct W(crate::W<CLOCKSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCKSTART_SPEC>;
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
impl From<crate::W<CLOCKSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCKSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_AW {
    #[doc = "1: Start all trace and debug clocks."]
    START = 1,
}
impl From<START_AW> for bool {
    #[inline(always)]
    fn from(variant: START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - "]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start all trace and debug clocks."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_AW::START)
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start all trace and debug clocks.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clockstart](index.html) module"]
pub struct CLOCKSTART_SPEC;
impl crate::RegisterSpec for CLOCKSTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clockstart::W](W) writer structure"]
impl crate::Writable for CLOCKSTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCKSTART to value 0"]
impl crate::Resettable for CLOCKSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
