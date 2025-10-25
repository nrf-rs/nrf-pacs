#[doc = "Register `TRCCNTVR[%s]` reader"]
pub type R = crate::R<TrccntvrSpec>;
#[doc = "Register `TRCCNTVR[%s]` writer"]
pub type W = crate::W<TrccntvrSpec>;
#[doc = "Field `VALUE` reader - Contains the count value of counter n."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - Contains the count value of counter n."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Contains the count value of counter n."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Contains the count value of counter n."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, TrccntvrSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: This sets or returns the value of counter n. The count value is only stable when TRCSTATR.PMSTABLE == 1. If software uses counter n then it must write to this register to set the initial counter value. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trccntvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccntvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrccntvrSpec;
impl crate::RegisterSpec for TrccntvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trccntvr::R`](R) reader structure"]
impl crate::Readable for TrccntvrSpec {}
#[doc = "`write(|w| ..)` method takes [`trccntvr::W`](W) writer structure"]
impl crate::Writable for TrccntvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCCNTVR[%s] to value 0"]
impl crate::Resettable for TrccntvrSpec {}
