#[doc = "Register `LEDPRE` reader"]
pub type R = crate::R<LedpreSpec>;
#[doc = "Register `LEDPRE` writer"]
pub type W = crate::W<LedpreSpec>;
#[doc = "Field `LEDPRE` reader - Period in us the LED is switched on prior to sampling"]
pub type LedpreR = crate::FieldReader<u16>;
#[doc = "Field `LEDPRE` writer - Period in us the LED is switched on prior to sampling"]
pub type LedpreW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Period in us the LED is switched on prior to sampling"]
    #[inline(always)]
    pub fn ledpre(&self) -> LedpreR {
        LedpreR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Period in us the LED is switched on prior to sampling"]
    #[inline(always)]
    pub fn ledpre(&mut self) -> LedpreW<'_, LedpreSpec> {
        LedpreW::new(self, 0)
    }
}
#[doc = "Time period the LED is switched ON prior to sampling\n\nYou can [`read`](crate::Reg::read) this register and get [`ledpre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledpre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LedpreSpec;
impl crate::RegisterSpec for LedpreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledpre::R`](R) reader structure"]
impl crate::Readable for LedpreSpec {}
#[doc = "`write(|w| ..)` method takes [`ledpre::W`](W) writer structure"]
impl crate::Writable for LedpreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEDPRE to value 0x10"]
impl crate::Resettable for LedpreSpec {
    const RESET_VALUE: u32 = 0x10;
}
