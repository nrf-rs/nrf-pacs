#[doc = "Register `CTIV` reader"]
pub type R = crate::R<CtivSpec>;
#[doc = "Register `CTIV` writer"]
pub type W = crate::W<CtivSpec>;
#[doc = "Field `CTIV` reader - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
pub type CtivR = crate::FieldReader;
#[doc = "Field `CTIV` writer - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
pub type CtivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
    #[inline(always)]
    pub fn ctiv(&self) -> CtivR {
        CtivR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
    #[inline(always)]
    pub fn ctiv(&mut self) -> CtivW<'_, CtivSpec> {
        CtivW::new(self, 0)
    }
}
#[doc = "Calibration timer interval\n\nYou can [`read`](crate::Reg::read) this register and get [`ctiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtivSpec;
impl crate::RegisterSpec for CtivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctiv::R`](R) reader structure"]
impl crate::Readable for CtivSpec {}
#[doc = "`write(|w| ..)` method takes [`ctiv::W`](W) writer structure"]
impl crate::Writable for CtivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIV to value 0"]
impl crate::Resettable for CtivSpec {}
