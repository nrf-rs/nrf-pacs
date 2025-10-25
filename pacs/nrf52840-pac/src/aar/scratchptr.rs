#[doc = "Register `SCRATCHPTR` reader"]
pub type R = crate::R<ScratchptrSpec>;
#[doc = "Register `SCRATCHPTR` writer"]
pub type W = crate::W<ScratchptrSpec>;
#[doc = "Field `SCRATCHPTR` reader - Pointer to a scratch data area used for temporary storage during resolution. A space of minimum 3 bytes must be reserved."]
pub type ScratchptrR = crate::FieldReader<u32>;
#[doc = "Field `SCRATCHPTR` writer - Pointer to a scratch data area used for temporary storage during resolution. A space of minimum 3 bytes must be reserved."]
pub type ScratchptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pointer to a scratch data area used for temporary storage during resolution. A space of minimum 3 bytes must be reserved."]
    #[inline(always)]
    pub fn scratchptr(&self) -> ScratchptrR {
        ScratchptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to a scratch data area used for temporary storage during resolution. A space of minimum 3 bytes must be reserved."]
    #[inline(always)]
    pub fn scratchptr(&mut self) -> ScratchptrW<'_, ScratchptrSpec> {
        ScratchptrW::new(self, 0)
    }
}
#[doc = "Pointer to data area used for temporary storage\n\nYou can [`read`](crate::Reg::read) this register and get [`scratchptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratchptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
