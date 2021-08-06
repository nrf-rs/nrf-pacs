#[doc = "Register `DEVICEID[%s]` reader"]
pub struct R(crate::R<DEVICEID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVICEID[%s]` writer"]
pub struct W(crate::W<DEVICEID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVICEID_SPEC>;
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
impl From<crate::W<DEVICEID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVICEID_SPEC>) -> Self {
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
#[doc = "Device identifier.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceid](index.html) module"]
pub struct DEVICEID_SPEC;
impl crate::RegisterSpec for DEVICEID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deviceid::R](R) reader structure"]
impl crate::Readable for DEVICEID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deviceid::W](W) writer structure"]
impl crate::Writable for DEVICEID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVICEID[%s]
to value 0xffff_ffff"]
impl crate::Resettable for DEVICEID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
