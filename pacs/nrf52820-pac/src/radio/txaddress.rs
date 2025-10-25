#[doc = "Register `TXADDRESS` reader"]
pub type R = crate::R<TxaddressSpec>;
#[doc = "Register `TXADDRESS` writer"]
pub type W = crate::W<TxaddressSpec>;
#[doc = "Field `TXADDRESS` reader - Transmit address select"]
pub type TxaddressR = crate::FieldReader;
#[doc = "Field `TXADDRESS` writer - Transmit address select"]
pub type TxaddressW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Transmit address select"]
    #[inline(always)]
    pub fn txaddress(&self) -> TxaddressR {
        TxaddressR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit address select"]
    #[inline(always)]
    pub fn txaddress(&mut self) -> TxaddressW<'_, TxaddressSpec> {
        TxaddressW::new(self, 0)
    }
}
#[doc = "Transmit address select\n\nYou can [`read`](crate::Reg::read) this register and get [`txaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxaddressSpec;
impl crate::RegisterSpec for TxaddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txaddress::R`](R) reader structure"]
impl crate::Readable for TxaddressSpec {}
#[doc = "`write(|w| ..)` method takes [`txaddress::W`](W) writer structure"]
impl crate::Writable for TxaddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXADDRESS to value 0"]
impl crate::Resettable for TxaddressSpec {}
