#[doc = "Register `DEVICEADDR[%s]` reader"]
pub struct R(crate::R<DEVICEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVICEADDR[%s]` writer"]
pub struct W(crate::W<DEVICEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVICEADDR_SPEC>;
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
impl From<crate::W<DEVICEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVICEADDR_SPEC>) -> Self {
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
#[doc = "Device address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceaddr](index.html) module"]
pub struct DEVICEADDR_SPEC;
impl crate::RegisterSpec for DEVICEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deviceaddr::R](R) reader structure"]
impl crate::Readable for DEVICEADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deviceaddr::W](W) writer structure"]
impl crate::Writable for DEVICEADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVICEADDR[%s]
to value 0xffff_ffff"]
impl crate::Resettable for DEVICEADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
