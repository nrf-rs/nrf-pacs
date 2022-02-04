#[doc = "Register `HOST_IOT_KDR0` reader"]
pub struct R(crate::R<HOST_IOT_KDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_IOT_KDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_IOT_KDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_IOT_KDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_IOT_KDR0` writer"]
pub struct W(crate::W<HOST_IOT_KDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_IOT_KDR0_SPEC>;
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
impl From<crate::W<HOST_IOT_KDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_IOT_KDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_IOT_KDR0` reader - Write: K_DR bits 31:0 Read: 0x00000000 when 128-bit K_DR key value is not yet retained in the CRYPTOCELL AO power domain Read: 0x00000001 when 128-bit K_DR key value is successfully retained in the CRYPTOCELL AO power domain"]
pub struct HOST_IOT_KDR0_R(crate::FieldReader<u32, u32>);
impl HOST_IOT_KDR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HOST_IOT_KDR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_IOT_KDR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_IOT_KDR0` writer - Write: K_DR bits 31:0 Read: 0x00000000 when 128-bit K_DR key value is not yet retained in the CRYPTOCELL AO power domain Read: 0x00000001 when 128-bit K_DR key value is successfully retained in the CRYPTOCELL AO power domain"]
pub struct HOST_IOT_KDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_IOT_KDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write: K_DR bits 31:0 Read: 0x00000000 when 128-bit K_DR key value is not yet retained in the CRYPTOCELL AO power domain Read: 0x00000001 when 128-bit K_DR key value is successfully retained in the CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn host_iot_kdr0(&self) -> HOST_IOT_KDR0_R {
        HOST_IOT_KDR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write: K_DR bits 31:0 Read: 0x00000000 when 128-bit K_DR key value is not yet retained in the CRYPTOCELL AO power domain Read: 0x00000001 when 128-bit K_DR key value is successfully retained in the CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn host_iot_kdr0(&mut self) -> HOST_IOT_KDR0_W {
        HOST_IOT_KDR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_iot_kdr0](index.html) module"]
pub struct HOST_IOT_KDR0_SPEC;
impl crate::RegisterSpec for HOST_IOT_KDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_iot_kdr0::R](R) reader structure"]
impl crate::Readable for HOST_IOT_KDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_iot_kdr0::W](W) writer structure"]
impl crate::Writable for HOST_IOT_KDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_IOT_KDR0 to value 0"]
impl crate::Resettable for HOST_IOT_KDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
