#[doc = "Register `AMOUNT` reader"]
pub type R = crate::R<AmountSpec>;
#[doc = "Register `AMOUNT` writer"]
pub type W = crate::W<AmountSpec>;
#[doc = "Field `TXDATABITS` reader - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
pub type TxdatabitsR = crate::FieldReader;
#[doc = "Field `TXDATABITS` writer - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
pub type TxdatabitsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXDATABYTES` reader - Number of complete bytes that shall be included in the frame, excluding CRC, parity, and framing."]
pub type TxdatabytesR = crate::FieldReader<u16>;
#[doc = "Field `TXDATABYTES` writer - Number of complete bytes that shall be included in the frame, excluding CRC, parity, and framing."]
pub type TxdatabytesW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:2 - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
    #[inline(always)]
    pub fn txdatabits(&self) -> TxdatabitsR {
        TxdatabitsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:11 - Number of complete bytes that shall be included in the frame, excluding CRC, parity, and framing."]
    #[inline(always)]
    pub fn txdatabytes(&self) -> TxdatabytesR {
        TxdatabytesR::new(((self.bits >> 3) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
    #[inline(always)]
    pub fn txdatabits(&mut self) -> TxdatabitsW<'_, AmountSpec> {
        TxdatabitsW::new(self, 0)
    }
    #[doc = "Bits 3:11 - Number of complete bytes that shall be included in the frame, excluding CRC, parity, and framing."]
    #[inline(always)]
    pub fn txdatabytes(&mut self) -> TxdatabytesW<'_, AmountSpec> {
        TxdatabytesW::new(self, 3)
    }
}
#[doc = "Size of outgoing frame\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmountSpec;
impl crate::RegisterSpec for AmountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amount::R`](R) reader structure"]
impl crate::Readable for AmountSpec {}
#[doc = "`write(|w| ..)` method takes [`amount::W`](W) writer structure"]
impl crate::Writable for AmountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AMOUNT to value 0"]
impl crate::Resettable for AmountSpec {}
