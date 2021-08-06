#[doc = "Register `NRF_1MBIT[%s]` reader"]
pub struct R(crate::R<NRF_1MBIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRF_1MBIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRF_1MBIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRF_1MBIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRF_1MBIT[%s]` writer"]
pub struct W(crate::W<NRF_1MBIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRF_1MBIT_SPEC>;
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
impl From<crate::W<NRF_1MBIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRF_1MBIT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrf_1mbit](index.html) module"]
pub struct NRF_1MBIT_SPEC;
impl crate::RegisterSpec for NRF_1MBIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrf_1mbit::R](R) reader structure"]
impl crate::Readable for NRF_1MBIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrf_1mbit::W](W) writer structure"]
impl crate::Writable for NRF_1MBIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NRF_1MBIT[%s]
to value 0xffff_ffff"]
impl crate::Resettable for NRF_1MBIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
