#[doc = "Register `TRCCNTRLDVR[%s]` reader"]
pub type R = crate::R<TrccntrldvrSpec>;
#[doc = "Register `TRCCNTRLDVR[%s]` writer"]
pub type W = crate::W<TrccntrldvrSpec>;
#[doc = "Field `VALUE` reader - Contains the reload value for counter n. When a reload event occurs for counter n then the trace unit copies the VALUEn field into counter n."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - Contains the reload value for counter n. When a reload event occurs for counter n then the trace unit copies the VALUEn field into counter n."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Contains the reload value for counter n. When a reload event occurs for counter n then the trace unit copies the VALUEn field into counter n."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Contains the reload value for counter n. When a reload event occurs for counter n then the trace unit copies the VALUEn field into counter n."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, TrccntrldvrSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: This sets or returns the reload count value for counter n. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trccntrldvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccntrldvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrccntrldvrSpec;
impl crate::RegisterSpec for TrccntrldvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trccntrldvr::R`](R) reader structure"]
impl crate::Readable for TrccntrldvrSpec {}
#[doc = "`write(|w| ..)` method takes [`trccntrldvr::W`](W) writer structure"]
impl crate::Writable for TrccntrldvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCCNTRLDVR[%s] to value 0"]
impl crate::Resettable for TrccntrldvrSpec {}
