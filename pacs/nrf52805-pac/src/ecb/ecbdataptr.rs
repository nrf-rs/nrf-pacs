#[doc = "Register `ECBDATAPTR` reader"]
pub type R = crate::R<EcbdataptrSpec>;
#[doc = "Register `ECBDATAPTR` writer"]
pub type W = crate::W<EcbdataptrSpec>;
#[doc = "Field `ECBDATAPTR` reader - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
pub type EcbdataptrR = crate::FieldReader<u32>;
#[doc = "Field `ECBDATAPTR` writer - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
pub type EcbdataptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
    #[inline(always)]
    pub fn ecbdataptr(&self) -> EcbdataptrR {
        EcbdataptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
    #[inline(always)]
    pub fn ecbdataptr(&mut self) -> EcbdataptrW<'_, EcbdataptrSpec> {
        EcbdataptrW::new(self, 0)
    }
}
#[doc = "ECB block encrypt memory pointers\n\nYou can [`read`](crate::Reg::read) this register and get [`ecbdataptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecbdataptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcbdataptrSpec;
impl crate::RegisterSpec for EcbdataptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecbdataptr::R`](R) reader structure"]
impl crate::Readable for EcbdataptrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecbdataptr::W`](W) writer structure"]
impl crate::Writable for EcbdataptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECBDATAPTR to value 0"]
impl crate::Resettable for EcbdataptrSpec {}
