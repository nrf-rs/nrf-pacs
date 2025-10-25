#[doc = "Register `TH` reader"]
pub type R = crate::R<ThSpec>;
#[doc = "Register `TH` writer"]
pub type W = crate::W<ThSpec>;
#[doc = "Field `THDOWN` reader - VDOWN = (THDOWN+1)/64*VREF"]
pub type ThdownR = crate::FieldReader;
#[doc = "Field `THDOWN` writer - VDOWN = (THDOWN+1)/64*VREF"]
pub type ThdownW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `THUP` reader - VUP = (THUP+1)/64*VREF"]
pub type ThupR = crate::FieldReader;
#[doc = "Field `THUP` writer - VUP = (THUP+1)/64*VREF"]
pub type ThupW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - VDOWN = (THDOWN+1)/64*VREF"]
    #[inline(always)]
    pub fn thdown(&self) -> ThdownR {
        ThdownR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - VUP = (THUP+1)/64*VREF"]
    #[inline(always)]
    pub fn thup(&self) -> ThupR {
        ThupR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - VDOWN = (THDOWN+1)/64*VREF"]
    #[inline(always)]
    pub fn thdown(&mut self) -> ThdownW<'_, ThSpec> {
        ThdownW::new(self, 0)
    }
    #[doc = "Bits 8:13 - VUP = (THUP+1)/64*VREF"]
    #[inline(always)]
    pub fn thup(&mut self) -> ThupW<'_, ThSpec> {
        ThupW::new(self, 8)
    }
}
#[doc = "Threshold configuration for hysteresis unit\n\nYou can [`read`](crate::Reg::read) this register and get [`th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThSpec;
impl crate::RegisterSpec for ThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`th::R`](R) reader structure"]
impl crate::Readable for ThSpec {}
#[doc = "`write(|w| ..)` method takes [`th::W`](W) writer structure"]
impl crate::Writable for ThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TH to value 0"]
impl crate::Resettable for ThSpec {}
