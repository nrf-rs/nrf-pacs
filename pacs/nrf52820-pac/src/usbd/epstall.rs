#[doc = "Register `EPSTALL` writer"]
pub struct W(crate::W<EPSTALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPSTALL_SPEC>;
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
impl From<crate::W<EPSTALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPSTALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP` writer - Select endpoint number"]
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Selects IN or OUT endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_AW {
    #[doc = "0: Selects OUT endpoint"]
    OUT = 0,
    #[doc = "1: Selects IN endpoint"]
    IN = 1,
}
impl From<IO_AW> for bool {
    #[inline(always)]
    fn from(variant: IO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO` writer - Selects IN or OUT endpoint"]
pub struct IO_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selects OUT endpoint"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO_AW::OUT)
    }
    #[doc = "Selects IN endpoint"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO_AW::IN)
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
#[doc = "Stall selected endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_AW {
    #[doc = "0: Don't stall selected endpoint"]
    UNSTALL = 0,
    #[doc = "1: Stall selected endpoint"]
    STALL = 1,
}
impl From<STALL_AW> for bool {
    #[inline(always)]
    fn from(variant: STALL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL` writer - Stall selected endpoint"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Don't stall selected endpoint"]
    #[inline(always)]
    pub fn un_stall(self) -> &'a mut W {
        self.variant(STALL_AW::UNSTALL)
    }
    #[doc = "Stall selected endpoint"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(STALL_AW::STALL)
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
impl W {
    #[doc = "Bits 0:2 - Select endpoint number"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn io(&mut self) -> IO_W {
        IO_W { w: self }
    }
    #[doc = "Bit 8 - Stall selected endpoint"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STALL endpoints\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstall](index.html) module"]
pub struct EPSTALL_SPEC;
impl crate::RegisterSpec for EPSTALL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [epstall::W](W) writer structure"]
impl crate::Writable for EPSTALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPSTALL to value 0"]
impl crate::Resettable for EPSTALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
