#[doc = "Register `HOST_IOT_KDR1` writer"]
pub struct W(crate::W<HOST_IOT_KDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_IOT_KDR1_SPEC>;
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
impl From<crate::W<HOST_IOT_KDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_IOT_KDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_IOT_KDR1` writer - K_DR bits 63:32"]
pub struct HOST_IOT_KDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_IOT_KDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - K_DR bits 63:32"]
    #[inline(always)]
    pub fn host_iot_kdr1(&mut self) -> HOST_IOT_KDR1_W {
        HOST_IOT_KDR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_iot_kdr1](index.html) module"]
pub struct HOST_IOT_KDR1_SPEC;
impl crate::RegisterSpec for HOST_IOT_KDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [host_iot_kdr1::W](W) writer structure"]
impl crate::Writable for HOST_IOT_KDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_IOT_KDR1 to value 0"]
impl crate::Resettable for HOST_IOT_KDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
