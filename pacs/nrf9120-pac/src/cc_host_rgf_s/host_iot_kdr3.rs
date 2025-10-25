#[doc = "Register `HOST_IOT_KDR3` writer"]
pub type W = crate::W<HostIotKdr3Spec>;
#[doc = "Field `HOST_IOT_KDR3` writer - K_DR bits 127:96"]
pub type HostIotKdr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - K_DR bits 127:96"]
    #[inline(always)]
    pub fn host_iot_kdr3(&mut self) -> HostIotKdr3W<'_, HostIotKdr3Spec> {
        HostIotKdr3W::new(self, 0)
    }
}
#[doc = "This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_kdr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostIotKdr3Spec;
impl crate::RegisterSpec for HostIotKdr3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`host_iot_kdr3::W`](W) writer structure"]
impl crate::Writable for HostIotKdr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_IOT_KDR3 to value 0"]
impl crate::Resettable for HostIotKdr3Spec {}
