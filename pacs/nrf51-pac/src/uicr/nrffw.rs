#[doc = "Register `NRFFW[%s]` reader"]
pub type R = crate::R<NrffwSpec>;
#[doc = "Register `NRFFW[%s]` writer"]
pub type W = crate::W<NrffwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reserved for Nordic firmware design.\n\nYou can [`read`](crate::Reg::read) this register and get [`nrffw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrffw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
