#[doc = "Register `TEP` reader"]
pub type R = crate::R<TepSpec>;
#[doc = "Register `TEP` writer"]
pub type W = crate::W<TepSpec>;
#[doc = "Field `TEP` reader - Pointer to task register. Accepts only addresses to registers from the Task group."]
pub type TepR = crate::FieldReader<u32>;
#[doc = "Field `TEP` writer - Pointer to task register. Accepts only addresses to registers from the Task group."]
pub type TepW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pointer to task register. Accepts only addresses to registers from the Task group."]
    #[inline(always)]
    pub fn tep(&self) -> TepR {
        TepR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to task register. Accepts only addresses to registers from the Task group."]
    #[inline(always)]
    pub fn tep(&mut self) -> TepW<'_, TepSpec> {
        TepW::new(self, 0)
    }
}
#[doc = "Description cluster\\[0\\]: Channel 0 task end-point\n\nYou can [`read`](crate::Reg::read) this register and get [`tep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TepSpec;
impl crate::RegisterSpec for TepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tep::R`](R) reader structure"]
impl crate::Readable for TepSpec {}
#[doc = "`write(|w| ..)` method takes [`tep::W`](W) writer structure"]
impl crate::Writable for TepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEP to value 0"]
impl crate::Resettable for TepSpec {}
