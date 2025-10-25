#[doc = "Register `HOST_IOT_KDR2` writer"]
pub type W = crate::W<HostIotKdr2Spec>;
#[doc = "Field `HOST_IOT_KDR2` writer - K_DR bits 95:64"]
pub type HostIotKdr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - K_DR bits 95:64"]
    #[inline(always)]
    pub fn host_iot_kdr2(&mut self) -> HostIotKdr2W<'_, HostIotKdr2Spec> {
        HostIotKdr2W::new(self, 0)
    }
}
#[doc = "This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_kdr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostIotKdr2Spec;
impl crate::RegisterSpec for HostIotKdr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`host_iot_kdr2::W`](W) writer structure"]
impl crate::Writable for HostIotKdr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_IOT_KDR2 to value 0"]
impl crate::Resettable for HostIotKdr2Spec {}
