#[doc = "Register `DFECTRL2` reader"]
pub type R = crate::R<Dfectrl2Spec>;
#[doc = "Register `DFECTRL2` writer"]
pub type W = crate::W<Dfectrl2Spec>;
#[doc = "Field `TSWITCHOFFSET` reader - Signed value offset after the end of the CRC before starting switching in number of 16M cycles"]
pub type TswitchoffsetR = crate::FieldReader<u16>;
#[doc = "Field `TSWITCHOFFSET` writer - Signed value offset after the end of the CRC before starting switching in number of 16M cycles"]
pub type TswitchoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `TSAMPLEOFFSET` reader - Signed value offset before starting sampling in number of 16M cycles relative to the beginning of the REFERENCE state - 12 us after switching start"]
pub type TsampleoffsetR = crate::FieldReader<u16>;
#[doc = "Field `TSAMPLEOFFSET` writer - Signed value offset before starting sampling in number of 16M cycles relative to the beginning of the REFERENCE state - 12 us after switching start"]
pub type TsampleoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:12 - Signed value offset after the end of the CRC before starting switching in number of 16M cycles"]
    #[inline(always)]
    pub fn tswitchoffset(&self) -> TswitchoffsetR {
        TswitchoffsetR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:27 - Signed value offset before starting sampling in number of 16M cycles relative to the beginning of the REFERENCE state - 12 us after switching start"]
    #[inline(always)]
    pub fn tsampleoffset(&self) -> TsampleoffsetR {
        TsampleoffsetR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Signed value offset after the end of the CRC before starting switching in number of 16M cycles"]
    #[inline(always)]
    pub fn tswitchoffset(&mut self) -> TswitchoffsetW<'_, Dfectrl2Spec> {
        TswitchoffsetW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Signed value offset before starting sampling in number of 16M cycles relative to the beginning of the REFERENCE state - 12 us after switching start"]
    #[inline(always)]
    pub fn tsampleoffset(&mut self) -> TsampleoffsetW<'_, Dfectrl2Spec> {
        TsampleoffsetW::new(self, 16)
    }
}
#[doc = "Start offset for Direction finding\n\nYou can [`read`](crate::Reg::read) this register and get [`dfectrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfectrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dfectrl2Spec;
impl crate::RegisterSpec for Dfectrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfectrl2::R`](R) reader structure"]
impl crate::Readable for Dfectrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`dfectrl2::W`](W) writer structure"]
impl crate::Writable for Dfectrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFECTRL2 to value 0"]
impl crate::Resettable for Dfectrl2Spec {}
