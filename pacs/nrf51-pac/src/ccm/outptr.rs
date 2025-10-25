#[doc = "Register `OUTPTR` reader"]
pub type R = crate::R<OutptrSpec>;
#[doc = "Register `OUTPTR` writer"]
pub type W = crate::W<OutptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pointer to the output packet.\n\nYou can [`read`](crate::Reg::read) this register and get [`outptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutptrSpec;
impl crate::RegisterSpec for OutptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outptr::R`](R) reader structure"]
impl crate::Readable for OutptrSpec {}
#[doc = "`write(|w| ..)` method takes [`outptr::W`](W) writer structure"]
impl crate::Writable for OutptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTPTR to value 0"]
impl crate::Resettable for OutptrSpec {}
