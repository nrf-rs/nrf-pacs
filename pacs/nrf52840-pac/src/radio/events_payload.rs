#[doc = "Register `EVENTS_PAYLOAD` reader"]
pub struct R(crate::R<EVENTS_PAYLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_PAYLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_PAYLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_PAYLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_PAYLOAD` writer"]
pub struct W(crate::W<EVENTS_PAYLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_PAYLOAD_SPEC>;
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
impl From<crate::W<EVENTS_PAYLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_PAYLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_PAYLOAD` reader - "]
pub type EVENTS_PAYLOAD_R = crate::BitReader<bool>;
#[doc = "Field `EVENTS_PAYLOAD` writer - "]
pub type EVENTS_PAYLOAD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_PAYLOAD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_payload(&self) -> EVENTS_PAYLOAD_R {
        EVENTS_PAYLOAD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_payload(&mut self) -> EVENTS_PAYLOAD_W<0> {
        EVENTS_PAYLOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet payload sent or received\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_payload](index.html) module"]
pub struct EVENTS_PAYLOAD_SPEC;
impl crate::RegisterSpec for EVENTS_PAYLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_payload::R](R) reader structure"]
impl crate::Readable for EVENTS_PAYLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_payload::W](W) writer structure"]
impl crate::Writable for EVENTS_PAYLOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_PAYLOAD to value 0"]
impl crate::Resettable for EVENTS_PAYLOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
