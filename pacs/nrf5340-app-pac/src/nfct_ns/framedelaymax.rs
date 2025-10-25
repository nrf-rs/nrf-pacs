#[doc = "Register `FRAMEDELAYMAX` reader"]
pub type R = crate::R<FramedelaymaxSpec>;
#[doc = "Register `FRAMEDELAYMAX` writer"]
pub type W = crate::W<FramedelaymaxSpec>;
#[doc = "Field `FRAMEDELAYMAX` reader - Maximum frame delay in number of 13.56 MHz clock cycles"]
pub type FramedelaymaxR = crate::FieldReader<u32>;
#[doc = "Field `FRAMEDELAYMAX` writer - Maximum frame delay in number of 13.56 MHz clock cycles"]
pub type FramedelaymaxW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Maximum frame delay in number of 13.56 MHz clock cycles"]
    #[inline(always)]
    pub fn framedelaymax(&self) -> FramedelaymaxR {
        FramedelaymaxR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Maximum frame delay in number of 13.56 MHz clock cycles"]
    #[inline(always)]
    pub fn framedelaymax(&mut self) -> FramedelaymaxW<'_, FramedelaymaxSpec> {
        FramedelaymaxW::new(self, 0)
    }
}
#[doc = "Maximum frame delay\n\nYou can [`read`](crate::Reg::read) this register and get [`framedelaymax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framedelaymax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FramedelaymaxSpec;
impl crate::RegisterSpec for FramedelaymaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framedelaymax::R`](R) reader structure"]
impl crate::Readable for FramedelaymaxSpec {}
#[doc = "`write(|w| ..)` method takes [`framedelaymax::W`](W) writer structure"]
impl crate::Writable for FramedelaymaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMEDELAYMAX to value 0x1000"]
impl crate::Resettable for FramedelaymaxSpec {
    const RESET_VALUE: u32 = 0x1000;
}
