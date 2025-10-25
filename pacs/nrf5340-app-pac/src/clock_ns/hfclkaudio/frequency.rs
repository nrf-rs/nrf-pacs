#[doc = "Register `FREQUENCY` reader"]
pub type R = crate::R<FrequencySpec>;
#[doc = "Register `FREQUENCY` writer"]
pub type W = crate::W<FrequencySpec>;
#[doc = "Field `FREQUENCY` reader - Frequency 0: 10.666 MHz 65535: 13.333 MHz"]
pub type FrequencyR = crate::FieldReader<u16>;
#[doc = "Field `FREQUENCY` writer - Frequency 0: 10.666 MHz 65535: 13.333 MHz"]
pub type FrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frequency 0: 10.666 MHz 65535: 13.333 MHz"]
    #[inline(always)]
    pub fn frequency(&self) -> FrequencyR {
        FrequencyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frequency 0: 10.666 MHz 65535: 13.333 MHz"]
    #[inline(always)]
    pub fn frequency(&mut self) -> FrequencyW<'_, FrequencySpec> {
        FrequencyW::new(self, 0)
    }
}
#[doc = "Audio PLL frequency in 11.176 MHz - 11.402 MHz or 12.165 MHz - 12.411 MHz frequency bands\n\nYou can [`read`](crate::Reg::read) this register and get [`frequency::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frequency::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrequencySpec;
impl crate::RegisterSpec for FrequencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frequency::R`](R) reader structure"]
impl crate::Readable for FrequencySpec {}
#[doc = "`write(|w| ..)` method takes [`frequency::W`](W) writer structure"]
impl crate::Writable for FrequencySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FREQUENCY to value 0x9bae"]
impl crate::Resettable for FrequencySpec {
    const RESET_VALUE: u32 = 0x9bae;
}
