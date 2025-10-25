#[doc = "Register `AMOUNT` reader"]
pub type R = crate::R<AmountSpec>;
#[doc = "Field `RXDATABITS` reader - Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing)."]
pub type RxdatabitsR = crate::FieldReader;
#[doc = "Field `RXDATABYTES` reader - Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)"]
pub type RxdatabytesR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing)."]
    #[inline(always)]
    pub fn rxdatabits(&self) -> RxdatabitsR {
        RxdatabitsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:11 - Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)"]
    #[inline(always)]
    pub fn rxdatabytes(&self) -> RxdatabytesR {
        RxdatabytesR::new(((self.bits >> 3) & 0x01ff) as u16)
    }
}
#[doc = "Size of last incoming frame\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmountSpec;
impl crate::RegisterSpec for AmountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amount::R`](R) reader structure"]
impl crate::Readable for AmountSpec {}
#[doc = "`reset()` method sets AMOUNT to value 0"]
impl crate::Resettable for AmountSpec {}
