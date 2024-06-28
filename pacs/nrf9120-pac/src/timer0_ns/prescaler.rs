#[doc = "Register `PRESCALER` reader"]
pub type R = crate::R<PrescalerSpec>;
#[doc = "Register `PRESCALER` writer"]
pub type W = crate::W<PrescalerSpec>;
#[doc = "Field `PRESCALER` reader - Prescaler value"]
pub type PrescalerR = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - Prescaler value"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Prescaler value"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler value"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<PrescalerSpec> {
        PrescalerW::new(self, 0)
    }
}
#[doc = "Timer prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prescaler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prescaler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrescalerSpec;
impl crate::RegisterSpec for PrescalerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prescaler::R`](R) reader structure"]
impl crate::Readable for PrescalerSpec {}
#[doc = "`write(|w| ..)` method takes [`prescaler::W`](W) writer structure"]
impl crate::Writable for PrescalerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESCALER to value 0x04"]
impl crate::Resettable for PrescalerSpec {
    const RESET_VALUE: u32 = 0x04;
}
