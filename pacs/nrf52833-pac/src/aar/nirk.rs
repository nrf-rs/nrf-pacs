#[doc = "Register `NIRK` reader"]
pub type R = crate::R<NirkSpec>;
#[doc = "Register `NIRK` writer"]
pub type W = crate::W<NirkSpec>;
#[doc = "Field `NIRK` reader - Number of Identity Root Keys available in the IRK data structure"]
pub type NirkR = crate::FieldReader;
#[doc = "Field `NIRK` writer - Number of Identity Root Keys available in the IRK data structure"]
pub type NirkW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number of Identity Root Keys available in the IRK data structure"]
    #[inline(always)]
    pub fn nirk(&self) -> NirkR {
        NirkR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of Identity Root Keys available in the IRK data structure"]
    #[inline(always)]
    pub fn nirk(&mut self) -> NirkW<'_, NirkSpec> {
        NirkW::new(self, 0)
    }
}
#[doc = "Number of IRKs\n\nYou can [`read`](crate::Reg::read) this register and get [`nirk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nirk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NirkSpec;
impl crate::RegisterSpec for NirkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nirk::R`](R) reader structure"]
impl crate::Readable for NirkSpec {}
#[doc = "`write(|w| ..)` method takes [`nirk::W`](W) writer structure"]
impl crate::Writable for NirkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NIRK to value 0x01"]
impl crate::Resettable for NirkSpec {
    const RESET_VALUE: u32 = 0x01;
}
