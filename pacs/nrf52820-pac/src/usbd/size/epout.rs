#[doc = "Register `EPOUT[%s]` reader"]
pub type R = crate::R<EpoutSpec>;
#[doc = "Register `EPOUT[%s]` writer"]
pub type W = crate::W<EpoutSpec>;
#[doc = "Field `SIZE` reader - Number of bytes received last in the data stage of this OUT endpoint"]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - Number of bytes received last in the data stage of this OUT endpoint"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, EpoutSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "Description collection: Number of bytes received last in the data stage of this OUT endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`epout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpoutSpec;
impl crate::RegisterSpec for EpoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epout::R`](R) reader structure"]
impl crate::Readable for EpoutSpec {}
#[doc = "`write(|w| ..)` method takes [`epout::W`](W) writer structure"]
impl crate::Writable for EpoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPOUT[%s] to value 0"]
impl crate::Resettable for EpoutSpec {}
