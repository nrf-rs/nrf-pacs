#[doc = "Register `XIPOFFSET` reader"]
pub type R = crate::R<XipoffsetSpec>;
#[doc = "Register `XIPOFFSET` writer"]
pub type W = crate::W<XipoffsetSpec>;
#[doc = "Field `XIPOFFSET` reader - Address offset into the external memory for Execute in Place operation. Value must be a multiple of 4."]
pub type XipoffsetR = crate::FieldReader<u32>;
#[doc = "Field `XIPOFFSET` writer - Address offset into the external memory for Execute in Place operation. Value must be a multiple of 4."]
pub type XipoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address offset into the external memory for Execute in Place operation. Value must be a multiple of 4."]
    #[inline(always)]
    pub fn xipoffset(&self) -> XipoffsetR {
        XipoffsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address offset into the external memory for Execute in Place operation. Value must be a multiple of 4."]
    #[inline(always)]
    pub fn xipoffset(&mut self) -> XipoffsetW<'_, XipoffsetSpec> {
        XipoffsetW::new(self, 0)
    }
}
#[doc = "Address offset into the external memory for Execute in Place operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`xipoffset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xipoffset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipoffsetSpec;
impl crate::RegisterSpec for XipoffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xipoffset::R`](R) reader structure"]
impl crate::Readable for XipoffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`xipoffset::W`](W) writer structure"]
impl crate::Writable for XipoffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XIPOFFSET to value 0"]
impl crate::Resettable for XipoffsetSpec {}
