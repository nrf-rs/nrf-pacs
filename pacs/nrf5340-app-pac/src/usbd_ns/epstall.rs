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
pub type EP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPSTALL_SPEC, u8, u8, 3, O>;
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
pub type IO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPSTALL_SPEC, IO_AW, O>;
impl<'a, const O: u8> IO_W<'a, O> {
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
}
#[doc = "Stall selected endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_AW {
    #[doc = "0: Don't stall selected endpoint"]
    UN_STALL = 0,
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
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPSTALL_SPEC, STALL_AW, O>;
impl<'a, const O: u8> STALL_W<'a, O> {
    #[doc = "Don't stall selected endpoint"]
    #[inline(always)]
    pub fn un_stall(self) -> &'a mut W {
        self.variant(STALL_AW::UN_STALL)
    }
    #[doc = "Stall selected endpoint"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(STALL_AW::STALL)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select endpoint number"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W<0> {
        EP_W::new(self)
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn io(&mut self) -> IO_W<7> {
        IO_W::new(self)
    }
    #[doc = "Bit 8 - Stall selected endpoint"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<8> {
        STALL_W::new(self)
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
