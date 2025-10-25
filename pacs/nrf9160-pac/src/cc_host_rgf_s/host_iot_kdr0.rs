#[doc = "Register `HOST_IOT_KDR0` reader"]
pub type R = crate::R<HostIotKdr0Spec>;
#[doc = "Register `HOST_IOT_KDR0` writer"]
pub type W = crate::W<HostIotKdr0Spec>;
#[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained. Write: K_DR bits 31:0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum HostIotKdr0 {
    #[doc = "0: Read: 128 bits K_DR key value is not yet retained in the CRYPTOCELL AO power domain."]
    NotRetained = 0,
    #[doc = "1: Read: 128 bits K_DR key value is successfully retained in the CRYPTOCELL AO power domain."]
    Retained = 1,
}
impl From<HostIotKdr0> for u32 {
    #[inline(always)]
    fn from(variant: HostIotKdr0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HostIotKdr0 {
    type Ux = u32;
}
impl crate::IsEnum for HostIotKdr0 {}
#[doc = "Field `HOST_IOT_KDR0` reader - This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained. Write: K_DR bits 31:0."]
pub type HostIotKdr0R = crate::FieldReader<HostIotKdr0>;
impl HostIotKdr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HostIotKdr0> {
        match self.bits {
            0 => Some(HostIotKdr0::NotRetained),
            1 => Some(HostIotKdr0::Retained),
            _ => None,
        }
    }
    #[doc = "Read: 128 bits K_DR key value is not yet retained in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn is_not_retained(&self) -> bool {
        *self == HostIotKdr0::NotRetained
    }
    #[doc = "Read: 128 bits K_DR key value is successfully retained in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == HostIotKdr0::Retained
    }
}
#[doc = "Field `HOST_IOT_KDR0` writer - This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained. Write: K_DR bits 31:0."]
pub type HostIotKdr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, HostIotKdr0>;
impl<'a, REG> HostIotKdr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Read: 128 bits K_DR key value is not yet retained in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn not_retained(self) -> &'a mut crate::W<REG> {
        self.variant(HostIotKdr0::NotRetained)
    }
    #[doc = "Read: 128 bits K_DR key value is successfully retained in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut crate::W<REG> {
        self.variant(HostIotKdr0::Retained)
    }
}
impl R {
    #[doc = "Bits 0:31 - This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained. Write: K_DR bits 31:0."]
    #[inline(always)]
    pub fn host_iot_kdr0(&self) -> HostIotKdr0R {
        HostIotKdr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained. Write: K_DR bits 31:0."]
    #[inline(always)]
    pub fn host_iot_kdr0(&mut self) -> HostIotKdr0W<'_, HostIotKdr0Spec> {
        HostIotKdr0W::new(self, 0)
    }
}
#[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_iot_kdr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_kdr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostIotKdr0Spec;
impl crate::RegisterSpec for HostIotKdr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_iot_kdr0::R`](R) reader structure"]
impl crate::Readable for HostIotKdr0Spec {}
#[doc = "`write(|w| ..)` method takes [`host_iot_kdr0::W`](W) writer structure"]
impl crate::Writable for HostIotKdr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_IOT_KDR0 to value 0"]
impl crate::Resettable for HostIotKdr0Spec {}
