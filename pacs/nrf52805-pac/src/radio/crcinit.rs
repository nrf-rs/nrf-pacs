#[doc = "Register `CRCINIT` reader"]
pub type R = crate::R<CrcinitSpec>;
#[doc = "Register `CRCINIT` writer"]
pub type W = crate::W<CrcinitSpec>;
#[doc = "Field `CRCINIT` reader - CRC initial value"]
pub type CrcinitR = crate::FieldReader<u32>;
#[doc = "Field `CRCINIT` writer - CRC initial value"]
pub type CrcinitW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - CRC initial value"]
    #[inline(always)]
    pub fn crcinit(&self) -> CrcinitR {
        CrcinitR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - CRC initial value"]
    #[inline(always)]
    pub fn crcinit(&mut self) -> CrcinitW<'_, CrcinitSpec> {
        CrcinitW::new(self, 0)
    }
}
#[doc = "CRC initial value\n\nYou can [`read`](crate::Reg::read) this register and get [`crcinit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcinit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcinitSpec;
impl crate::RegisterSpec for CrcinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcinit::R`](R) reader structure"]
impl crate::Readable for CrcinitSpec {}
#[doc = "`write(|w| ..)` method takes [`crcinit::W`](W) writer structure"]
impl crate::Writable for CrcinitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCINIT to value 0"]
impl crate::Resettable for CrcinitSpec {}
