#[doc = "Register `EDCNT` reader"]
pub type R = crate::R<EdcntSpec>;
#[doc = "Register `EDCNT` writer"]
pub type W = crate::W<EdcntSpec>;
#[doc = "Field `EDCNT` reader - IEEE 802.15.4 energy detect loop count"]
pub type EdcntR = crate::FieldReader<u32>;
#[doc = "Field `EDCNT` writer - IEEE 802.15.4 energy detect loop count"]
pub type EdcntW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub fn edcnt(&self) -> EdcntR {
        EdcntR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub fn edcnt(&mut self) -> EdcntW<'_, EdcntSpec> {
        EdcntW::new(self, 0)
    }
}
#[doc = "IEEE 802.15.4 energy detect loop count\n\nYou can [`read`](crate::Reg::read) this register and get [`edcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EdcntSpec;
impl crate::RegisterSpec for EdcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edcnt::R`](R) reader structure"]
impl crate::Readable for EdcntSpec {}
#[doc = "`write(|w| ..)` method takes [`edcnt::W`](W) writer structure"]
impl crate::Writable for EdcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EDCNT to value 0"]
impl crate::Resettable for EdcntSpec {}
