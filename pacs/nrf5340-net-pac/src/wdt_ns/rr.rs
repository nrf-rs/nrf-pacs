#[doc = "Register `RR[%s]` writer"]
pub struct W(crate::W<RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RR_SPEC>;
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
impl From<crate::W<RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reload request register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RR_AW {
    #[doc = "1850885685: Value to request a reload of the watchdog timer"]
    RELOAD = 1850885685,
}
impl From<RR_AW> for u32 {
    #[inline(always)]
    fn from(variant: RR_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `RR` writer - Reload request register"]
pub struct RR_W<'a> {
    w: &'a mut W,
}
impl<'a> RR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Value to request a reload of the watchdog timer"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(RR_AW::RELOAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Reload request register"]
    #[inline(always)]
    pub fn rr(&mut self) -> RR_W {
        RR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Reload request n\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rr](index.html) module"]
pub struct RR_SPEC;
impl crate::RegisterSpec for RR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rr::W](W) writer structure"]
impl crate::Writable for RR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RR[%s]
to value 0"]
impl crate::Resettable for RR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
