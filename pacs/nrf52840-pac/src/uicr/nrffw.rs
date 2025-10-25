#[doc = "Register `NRFFW[%s]` reader"]
pub type R = crate::R<NrffwSpec>;
#[doc = "Register `NRFFW[%s]` writer"]
pub type W = crate::W<NrffwSpec>;
#[doc = "Field `NRFFW` reader - Reserved for Nordic firmware design"]
pub type NrffwR = crate::FieldReader<u32>;
#[doc = "Field `NRFFW` writer - Reserved for Nordic firmware design"]
pub type NrffwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw(&self) -> NrffwR {
        NrffwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw(&mut self) -> NrffwW<'_, NrffwSpec> {
        NrffwW::new(self, 0)
    }
}
#[doc = "Description collection: Reserved for Nordic firmware design\n\nYou can [`read`](crate::Reg::read) this register and get [`nrffw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrffw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NrffwSpec;
impl crate::RegisterSpec for NrffwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nrffw::R`](R) reader structure"]
impl crate::Readable for NrffwSpec {}
#[doc = "`write(|w| ..)` method takes [`nrffw::W`](W) writer structure"]
impl crate::Writable for NrffwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NRFFW[%s] to value 0xffff_ffff"]
impl crate::Resettable for NrffwSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
