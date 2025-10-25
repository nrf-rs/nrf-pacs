#[doc = "Register `GPREGRET2` reader"]
pub type R = crate::R<Gpregret2Spec>;
#[doc = "Register `GPREGRET2` writer"]
pub type W = crate::W<Gpregret2Spec>;
#[doc = "Field `GPREGRET` reader - General purpose retention register"]
pub type GpregretR = crate::FieldReader;
#[doc = "Field `GPREGRET` writer - General purpose retention register"]
pub type GpregretW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - General purpose retention register"]
    #[inline(always)]
    pub fn gpregret(&self) -> GpregretR {
        GpregretR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - General purpose retention register"]
    #[inline(always)]
    pub fn gpregret(&mut self) -> GpregretW<'_, Gpregret2Spec> {
        GpregretW::new(self, 0)
    }
}
#[doc = "General purpose retention register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpregret2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpregret2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpregret2Spec;
impl crate::RegisterSpec for Gpregret2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpregret2::R`](R) reader structure"]
impl crate::Readable for Gpregret2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpregret2::W`](W) writer structure"]
impl crate::Writable for Gpregret2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPREGRET2 to value 0"]
impl crate::Resettable for Gpregret2Spec {}
