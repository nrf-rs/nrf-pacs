#[doc = "Register `NRFMDK[%s]` reader"]
pub type R = crate::R<NrfmdkSpec>;
#[doc = "Register `NRFMDK[%s]` writer"]
pub type W = crate::W<NrfmdkSpec>;
#[doc = "Field `NRFMDK` reader - Reserved for Nordic MDK"]
pub type NrfmdkR = crate::FieldReader<u32>;
#[doc = "Field `NRFMDK` writer - Reserved for Nordic MDK"]
pub type NrfmdkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved for Nordic MDK"]
    #[inline(always)]
    pub fn nrfmdk(&self) -> NrfmdkR {
        NrfmdkR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for Nordic MDK"]
    #[inline(always)]
    pub fn nrfmdk(&mut self) -> NrfmdkW<'_, NrfmdkSpec> {
        NrfmdkW::new(self, 0)
    }
}
#[doc = "Description collection: Reserved for Nordic MDK\n\nYou can [`read`](crate::Reg::read) this register and get [`nrfmdk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrfmdk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NrfmdkSpec;
impl crate::RegisterSpec for NrfmdkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nrfmdk::R`](R) reader structure"]
impl crate::Readable for NrfmdkSpec {}
#[doc = "`write(|w| ..)` method takes [`nrfmdk::W`](W) writer structure"]
impl crate::Writable for NrfmdkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NRFMDK[%s] to value 0xffff_ffff"]
impl crate::Resettable for NrfmdkSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
