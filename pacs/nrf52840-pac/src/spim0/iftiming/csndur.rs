#[doc = "Register `CSNDUR` reader"]
pub type R = crate::R<CsndurSpec>;
#[doc = "Register `CSNDUR` writer"]
pub type W = crate::W<CsndurSpec>;
#[doc = "Field `CSNDUR` reader - Minimum duration between edge of CSN and edge of SCK at the start and end of a transaction. If END-START shortcut is used, minimum duration CSN will stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
pub type CsndurR = crate::FieldReader;
#[doc = "Field `CSNDUR` writer - Minimum duration between edge of CSN and edge of SCK at the start and end of a transaction. If END-START shortcut is used, minimum duration CSN will stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
pub type CsndurW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Minimum duration between edge of CSN and edge of SCK at the start and end of a transaction. If END-START shortcut is used, minimum duration CSN will stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
    #[inline(always)]
    pub fn csndur(&self) -> CsndurR {
        CsndurR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum duration between edge of CSN and edge of SCK at the start and end of a transaction. If END-START shortcut is used, minimum duration CSN will stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
    #[inline(always)]
    pub fn csndur(&mut self) -> CsndurW<'_, CsndurSpec> {
        CsndurW::new(self, 0)
    }
}
#[doc = "Minimum duration between edge of CSN and edge of SCK at the start and the end of a transaction, and minimum duration CSN will stay high between transactions if END-START shortcut is used\n\nYou can [`read`](crate::Reg::read) this register and get [`csndur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csndur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsndurSpec;
impl crate::RegisterSpec for CsndurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csndur::R`](R) reader structure"]
impl crate::Readable for CsndurSpec {}
#[doc = "`write(|w| ..)` method takes [`csndur::W`](W) writer structure"]
impl crate::Writable for CsndurSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSNDUR to value 0x02"]
impl crate::Resettable for CsndurSpec {
    const RESET_VALUE: u32 = 0x02;
}
