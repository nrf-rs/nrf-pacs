#[doc = "Register `EVENTS_RXPTRUPD` reader"]
pub struct R(crate::R<EVENTS_RXPTRUPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_RXPTRUPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_RXPTRUPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_RXPTRUPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_RXPTRUPD` writer"]
pub struct W(crate::W<EVENTS_RXPTRUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_RXPTRUPD_SPEC>;
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
impl From<crate::W<EVENTS_RXPTRUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_RXPTRUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_RXPTRUPD` reader - "]
pub type EVENTS_RXPTRUPD_R = crate::BitReader<bool>;
#[doc = "Field `EVENTS_RXPTRUPD` writer - "]
pub type EVENTS_RXPTRUPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_RXPTRUPD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_rxptrupd(&self) -> EVENTS_RXPTRUPD_R {
        EVENTS_RXPTRUPD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_rxptrupd(&mut self) -> EVENTS_RXPTRUPD_W<0> {
        EVENTS_RXPTRUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxptrupd](index.html) module"]
pub struct EVENTS_RXPTRUPD_SPEC;
impl crate::RegisterSpec for EVENTS_RXPTRUPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_rxptrupd::R](R) reader structure"]
impl crate::Readable for EVENTS_RXPTRUPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_rxptrupd::W](W) writer structure"]
impl crate::Writable for EVENTS_RXPTRUPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_RXPTRUPD to value 0"]
impl crate::Resettable for EVENTS_RXPTRUPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
