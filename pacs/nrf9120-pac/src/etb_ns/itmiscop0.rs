#[doc = "Register `ITMISCOP0` writer"]
pub type W = crate::W<Itmiscop0Spec>;
#[doc = "Field `ACQCOMP` writer - Set the value of acqcomp."]
pub type AcqcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL` writer - Set the value of full output port."]
pub type FullW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set the value of acqcomp."]
    #[inline(always)]
    pub fn acqcomp(&mut self) -> AcqcompW<'_, Itmiscop0Spec> {
        AcqcompW::new(self, 0)
    }
    #[doc = "Bit 1 - Set the value of full output port."]
    #[inline(always)]
    pub fn full(&mut self) -> FullW<'_, Itmiscop0Spec> {
        FullW::new(self, 1)
    }
}
#[doc = "Integration Test Miscellaneous Output Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itmiscop0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itmiscop0Spec;
impl crate::RegisterSpec for Itmiscop0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`itmiscop0::W`](W) writer structure"]
impl crate::Writable for Itmiscop0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITMISCOP0 to value 0"]
impl crate::Resettable for Itmiscop0Spec {}
