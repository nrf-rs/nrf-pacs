#[doc = "Register `MHRMATCHCONF` reader"]
pub type R = crate::R<MhrmatchconfSpec>;
#[doc = "Register `MHRMATCHCONF` writer"]
pub type W = crate::W<MhrmatchconfSpec>;
#[doc = "Field `MHRMATCHCONF` reader - Search pattern configuration"]
pub type MhrmatchconfR = crate::FieldReader<u32>;
#[doc = "Field `MHRMATCHCONF` writer - Search pattern configuration"]
pub type MhrmatchconfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Search pattern configuration"]
    #[inline(always)]
    pub fn mhrmatchconf(&self) -> MhrmatchconfR {
        MhrmatchconfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Search pattern configuration"]
    #[inline(always)]
    pub fn mhrmatchconf(&mut self) -> MhrmatchconfW<'_, MhrmatchconfSpec> {
        MhrmatchconfW::new(self, 0)
    }
}
#[doc = "Search pattern configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mhrmatchconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mhrmatchconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MhrmatchconfSpec;
impl crate::RegisterSpec for MhrmatchconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhrmatchconf::R`](R) reader structure"]
impl crate::Readable for MhrmatchconfSpec {}
#[doc = "`write(|w| ..)` method takes [`mhrmatchconf::W`](W) writer structure"]
impl crate::Writable for MhrmatchconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MHRMATCHCONF to value 0"]
impl crate::Resettable for MhrmatchconfSpec {}
