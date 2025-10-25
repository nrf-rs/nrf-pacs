#[doc = "Register `NRFHW[%s]` reader"]
pub type R = crate::R<NrfhwSpec>;
#[doc = "Register `NRFHW[%s]` writer"]
pub type W = crate::W<NrfhwSpec>;
#[doc = "Field `NRFHW` reader - Reserved for Nordic hardware design"]
pub type NrfhwR = crate::FieldReader<u32>;
#[doc = "Field `NRFHW` writer - Reserved for Nordic hardware design"]
pub type NrfhwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved for Nordic hardware design"]
    #[inline(always)]
    pub fn nrfhw(&self) -> NrfhwR {
        NrfhwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for Nordic hardware design"]
    #[inline(always)]
    pub fn nrfhw(&mut self) -> NrfhwW<'_, NrfhwSpec> {
        NrfhwW::new(self, 0)
    }
}
#[doc = "Description collection: Reserved for Nordic hardware design\n\nYou can [`read`](crate::Reg::read) this register and get [`nrfhw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrfhw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NrfhwSpec;
impl crate::RegisterSpec for NrfhwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nrfhw::R`](R) reader structure"]
impl crate::Readable for NrfhwSpec {}
#[doc = "`write(|w| ..)` method takes [`nrfhw::W`](W) writer structure"]
impl crate::Writable for NrfhwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NRFHW[%s] to value 0xffff_ffff"]
impl crate::Resettable for NrfhwSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
