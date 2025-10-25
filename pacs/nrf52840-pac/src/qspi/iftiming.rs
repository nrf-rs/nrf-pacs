#[doc = "Register `IFTIMING` reader"]
pub type R = crate::R<IftimingSpec>;
#[doc = "Register `IFTIMING` writer"]
pub type W = crate::W<IftimingSpec>;
#[doc = "Field `RXDELAY` reader - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
pub type RxdelayR = crate::FieldReader;
#[doc = "Field `RXDELAY` writer - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
pub type RxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 8:10 - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&self) -> RxdelayR {
        RxdelayR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&mut self) -> RxdelayW<'_, IftimingSpec> {
        RxdelayW::new(self, 8)
    }
}
#[doc = "SPI interface timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`iftiming::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iftiming::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IftimingSpec;
impl crate::RegisterSpec for IftimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iftiming::R`](R) reader structure"]
impl crate::Readable for IftimingSpec {}
#[doc = "`write(|w| ..)` method takes [`iftiming::W`](W) writer structure"]
impl crate::Writable for IftimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFTIMING to value 0x0200"]
impl crate::Resettable for IftimingSpec {
    const RESET_VALUE: u32 = 0x0200;
}
