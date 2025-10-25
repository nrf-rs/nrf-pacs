#[doc = "Register `SCRATCHPTR` reader"]
pub type R = crate::R<ScratchptrSpec>;
#[doc = "Register `SCRATCHPTR` writer"]
pub type W = crate::W<ScratchptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pointer to a scratch data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`scratchptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratchptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScratchptrSpec;
impl crate::RegisterSpec for ScratchptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scratchptr::R`](R) reader structure"]
impl crate::Readable for ScratchptrSpec {}
#[doc = "`write(|w| ..)` method takes [`scratchptr::W`](W) writer structure"]
impl crate::Writable for ScratchptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCRATCHPTR to value 0"]
impl crate::Resettable for ScratchptrSpec {}
