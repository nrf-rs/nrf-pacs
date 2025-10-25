#[doc = "Register `GPREGRET[%s]` reader"]
pub type R = crate::R<GpregretSpec>;
#[doc = "Register `GPREGRET[%s]` writer"]
pub type W = crate::W<GpregretSpec>;
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
    pub fn gpregret(&mut self) -> GpregretW<'_, GpregretSpec> {
        GpregretW::new(self, 0)
    }
}
#[doc = "Description collection: General purpose retention register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpregret::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpregret::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpregretSpec;
impl crate::RegisterSpec for GpregretSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpregret::R`](R) reader structure"]
impl crate::Readable for GpregretSpec {}
#[doc = "`write(|w| ..)` method takes [`gpregret::W`](W) writer structure"]
impl crate::Writable for GpregretSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPREGRET[%s] to value 0"]
impl crate::Resettable for GpregretSpec {}
