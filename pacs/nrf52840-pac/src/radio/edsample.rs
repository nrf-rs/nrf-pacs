#[doc = "Register `EDSAMPLE` reader"]
pub type R = crate::R<EdsampleSpec>;
#[doc = "Register `EDSAMPLE` writer"]
pub type W = crate::W<EdsampleSpec>;
#[doc = "Field `EDLVL` reader - IEEE 802.15.4 energy detect level"]
pub type EdlvlR = crate::FieldReader;
#[doc = "Field `EDLVL` writer - IEEE 802.15.4 energy detect level"]
pub type EdlvlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edlvl(&self) -> EdlvlR {
        EdlvlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edlvl(&mut self) -> EdlvlW<'_, EdsampleSpec> {
        EdlvlW::new(self, 0)
    }
}
#[doc = "IEEE 802.15.4 energy detect level\n\nYou can [`read`](crate::Reg::read) this register and get [`edsample::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edsample::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EdsampleSpec;
impl crate::RegisterSpec for EdsampleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edsample::R`](R) reader structure"]
impl crate::Readable for EdsampleSpec {}
#[doc = "`write(|w| ..)` method takes [`edsample::W`](W) writer structure"]
impl crate::Writable for EdsampleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EDSAMPLE to value 0"]
impl crate::Resettable for EdsampleSpec {}
