#[doc = "Register `HOST_IOT_KDR1` writer"]
pub type W = crate::W<HostIotKdr1Spec>;
#[doc = "Field `HOST_IOT_KDR1` writer - K_DR bits 63:32"]
pub type HostIotKdr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - K_DR bits 63:32"]
    #[inline(always)]
    pub fn host_iot_kdr1(&mut self) -> HostIotKdr1W<'_, HostIotKdr1Spec> {
        HostIotKdr1W::new(self, 0)
    }
}
#[doc = "This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_kdr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostIotKdr1Spec;
impl crate::RegisterSpec for HostIotKdr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`host_iot_kdr1::W`](W) writer structure"]
impl crate::Writable for HostIotKdr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_IOT_KDR1 to value 0"]
impl crate::Resettable for HostIotKdr1Spec {}
