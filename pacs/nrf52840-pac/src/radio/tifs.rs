#[doc = "Register `TIFS` reader"]
pub type R = crate::R<TifsSpec>;
#[doc = "Register `TIFS` writer"]
pub type W = crate::W<TifsSpec>;
#[doc = "Field `TIFS` reader - Interframe spacing in us"]
pub type TifsR = crate::FieldReader<u16>;
#[doc = "Field `TIFS` writer - Interframe spacing in us"]
pub type TifsW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Interframe spacing in us"]
    #[inline(always)]
    pub fn tifs(&self) -> TifsR {
        TifsR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interframe spacing in us"]
    #[inline(always)]
    pub fn tifs(&mut self) -> TifsW<'_, TifsSpec> {
        TifsW::new(self, 0)
    }
}
#[doc = "Interframe spacing in us\n\nYou can [`read`](crate::Reg::read) this register and get [`tifs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TifsSpec;
impl crate::RegisterSpec for TifsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tifs::R`](R) reader structure"]
impl crate::Readable for TifsSpec {}
#[doc = "`write(|w| ..)` method takes [`tifs::W`](W) writer structure"]
impl crate::Writable for TifsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIFS to value 0"]
impl crate::Resettable for TifsSpec {}
