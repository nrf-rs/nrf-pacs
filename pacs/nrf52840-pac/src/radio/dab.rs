#[doc = "Register `DAB[%s]` reader"]
pub type R = crate::R<DabSpec>;
#[doc = "Register `DAB[%s]` writer"]
pub type W = crate::W<DabSpec>;
#[doc = "Field `DAB` reader - Device address base segment n"]
pub type DabR = crate::FieldReader<u32>;
#[doc = "Field `DAB` writer - Device address base segment n"]
pub type DabW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device address base segment n"]
    #[inline(always)]
    pub fn dab(&self) -> DabR {
        DabR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device address base segment n"]
    #[inline(always)]
    pub fn dab(&mut self) -> DabW<'_, DabSpec> {
        DabW::new(self, 0)
    }
}
#[doc = "Description collection: Device address base segment n\n\nYou can [`read`](crate::Reg::read) this register and get [`dab::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dab::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DabSpec;
impl crate::RegisterSpec for DabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dab::R`](R) reader structure"]
impl crate::Readable for DabSpec {}
#[doc = "`write(|w| ..)` method takes [`dab::W`](W) writer structure"]
impl crate::Writable for DabSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAB[%s] to value 0"]
impl crate::Resettable for DabSpec {}
