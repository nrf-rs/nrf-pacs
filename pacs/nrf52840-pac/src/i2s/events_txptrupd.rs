#[doc = "Register `EVENTS_TXPTRUPD` reader"]
pub struct R(crate::R<EVENTS_TXPTRUPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_TXPTRUPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_TXPTRUPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_TXPTRUPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_TXPTRUPD` writer"]
pub struct W(crate::W<EVENTS_TXPTRUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_TXPTRUPD_SPEC>;
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
impl From<crate::W<EVENTS_TXPTRUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_TXPTRUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_TXPTRUPD` reader - "]
pub type EVENTS_TXPTRUPD_R = crate::BitReader<bool>;
#[doc = "Field `EVENTS_TXPTRUPD` writer - "]
pub type EVENTS_TXPTRUPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_TXPTRUPD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_txptrupd(&self) -> EVENTS_TXPTRUPD_R {
        EVENTS_TXPTRUPD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_txptrupd(&mut self) -> EVENTS_TXPTRUPD_W<0> {
        EVENTS_TXPTRUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txptrupd](index.html) module"]
pub struct EVENTS_TXPTRUPD_SPEC;
impl crate::RegisterSpec for EVENTS_TXPTRUPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_txptrupd::R](R) reader structure"]
impl crate::Readable for EVENTS_TXPTRUPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_txptrupd::W](W) writer structure"]
impl crate::Writable for EVENTS_TXPTRUPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_TXPTRUPD to value 0"]
impl crate::Resettable for EVENTS_TXPTRUPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
