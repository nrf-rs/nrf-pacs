#[doc = "Register `CONTEXT_ID` reader"]
pub type R = crate::R<ContextIdSpec>;
#[doc = "Register `CONTEXT_ID` writer"]
pub type W = crate::W<ContextIdSpec>;
#[doc = "Field `CONTEXT_ID` reader - Context ID"]
pub type ContextIdR = crate::FieldReader;
#[doc = "Field `CONTEXT_ID` writer - Context ID"]
pub type ContextIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Context ID"]
    #[inline(always)]
    pub fn context_id(&self) -> ContextIdR {
        ContextIdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Context ID"]
    #[inline(always)]
    pub fn context_id(&mut self) -> ContextIdW<'_, ContextIdSpec> {
        ContextIdW::new(self, 0)
    }
}
#[doc = "A general-purpose read/write register.\n\nYou can [`read`](crate::Reg::read) this register and get [`context_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`context_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ContextIdSpec;
impl crate::RegisterSpec for ContextIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`context_id::R`](R) reader structure"]
impl crate::Readable for ContextIdSpec {}
#[doc = "`write(|w| ..)` method takes [`context_id::W`](W) writer structure"]
impl crate::Writable for ContextIdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTEXT_ID to value 0"]
impl crate::Resettable for ContextIdSpec {}
