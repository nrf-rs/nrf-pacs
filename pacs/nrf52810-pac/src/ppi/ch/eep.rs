#[doc = "Register `EEP` reader"]
pub type R = crate::R<EepSpec>;
#[doc = "Register `EEP` writer"]
pub type W = crate::W<EepSpec>;
#[doc = "Field `EEP` reader - Pointer to event register. Accepts only addresses to registers from the Event group."]
pub type EepR = crate::FieldReader<u32>;
#[doc = "Field `EEP` writer - Pointer to event register. Accepts only addresses to registers from the Event group."]
pub type EepW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pointer to event register. Accepts only addresses to registers from the Event group."]
    #[inline(always)]
    pub fn eep(&self) -> EepR {
        EepR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to event register. Accepts only addresses to registers from the Event group."]
    #[inline(always)]
    pub fn eep(&mut self) -> EepW<'_, EepSpec> {
        EepW::new(self, 0)
    }
}
#[doc = "Description cluster: Channel n event end-point\n\nYou can [`read`](crate::Reg::read) this register and get [`eep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EepSpec;
impl crate::RegisterSpec for EepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eep::R`](R) reader structure"]
impl crate::Readable for EepSpec {}
#[doc = "`write(|w| ..)` method takes [`eep::W`](W) writer structure"]
impl crate::Writable for EepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEP to value 0"]
impl crate::Resettable for EepSpec {}
