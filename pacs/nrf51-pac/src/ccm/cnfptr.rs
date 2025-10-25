#[doc = "Register `CNFPTR` reader"]
pub type R = crate::R<CnfptrSpec>;
#[doc = "Register `CNFPTR` writer"]
pub type W = crate::W<CnfptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pointer to a data structure holding AES key and NONCE vector.\n\nYou can [`read`](crate::Reg::read) this register and get [`cnfptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnfptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CnfptrSpec;
impl crate::RegisterSpec for CnfptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnfptr::R`](R) reader structure"]
impl crate::Readable for CnfptrSpec {}
#[doc = "`write(|w| ..)` method takes [`cnfptr::W`](W) writer structure"]
impl crate::Writable for CnfptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNFPTR to value 0"]
impl crate::Resettable for CnfptrSpec {}
