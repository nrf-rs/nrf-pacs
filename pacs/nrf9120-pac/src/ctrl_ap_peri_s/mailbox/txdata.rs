#[doc = "Register `TXDATA` reader"]
pub type R = crate::R<TxdataSpec>;
#[doc = "Register `TXDATA` writer"]
pub type W = crate::W<TxdataSpec>;
#[doc = "Field `TXDATA` reader - Data sent to debugger"]
pub type TxdataR = crate::FieldReader<u32>;
#[doc = "Field `TXDATA` writer - Data sent to debugger"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data sent to debugger"]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data sent to debugger"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<TxdataSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "Data sent from the CPU to the debugger.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdataSpec;
impl crate::RegisterSpec for TxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata::R`](R) reader structure"]
impl crate::Readable for TxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TxdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TxdataSpec {
    const RESET_VALUE: u32 = 0;
}
