#[doc = "Register `RXDELAY` reader"]
pub type R = crate::R<RxdelaySpec>;
#[doc = "Register `RXDELAY` writer"]
pub type W = crate::W<RxdelaySpec>;
#[doc = "Field `RXDELAY` reader - Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
pub type RxdelayR = crate::FieldReader;
#[doc = "Field `RXDELAY` writer - Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
pub type RxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&self) -> RxdelayR {
        RxdelayR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&mut self) -> RxdelayW<'_, RxdelaySpec> {
        RxdelayW::new(self, 0)
    }
}
#[doc = "Sample delay for input serial data on MISO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdelay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdelay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdelaySpec;
impl crate::RegisterSpec for RxdelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdelay::R`](R) reader structure"]
impl crate::Readable for RxdelaySpec {}
#[doc = "`write(|w| ..)` method takes [`rxdelay::W`](W) writer structure"]
impl crate::Writable for RxdelaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXDELAY to value 0x02"]
impl crate::Resettable for RxdelaySpec {
    const RESET_VALUE: u32 = 0x02;
}
