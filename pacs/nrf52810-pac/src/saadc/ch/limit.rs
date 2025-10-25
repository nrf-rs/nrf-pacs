#[doc = "Register `LIMIT` reader"]
pub type R = crate::R<LimitSpec>;
#[doc = "Register `LIMIT` writer"]
pub type W = crate::W<LimitSpec>;
#[doc = "Field `LOW` reader - Low level limit"]
pub type LowR = crate::FieldReader<u16>;
#[doc = "Field `LOW` writer - Low level limit"]
pub type LowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HIGH` reader - High level limit"]
pub type HighR = crate::FieldReader<u16>;
#[doc = "Field `HIGH` writer - High level limit"]
pub type HighW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low level limit"]
    #[inline(always)]
    pub fn low(&self) -> LowR {
        LowR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High level limit"]
    #[inline(always)]
    pub fn high(&self) -> HighR {
        HighR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low level limit"]
    #[inline(always)]
    pub fn low(&mut self) -> LowW<'_, LimitSpec> {
        LowW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High level limit"]
    #[inline(always)]
    pub fn high(&mut self) -> HighW<'_, LimitSpec> {
        HighW::new(self, 16)
    }
}
#[doc = "Description cluster: High/low limits for event monitoring a channel\n\nYou can [`read`](crate::Reg::read) this register and get [`limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimitSpec;
impl crate::RegisterSpec for LimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limit::R`](R) reader structure"]
impl crate::Readable for LimitSpec {}
#[doc = "`write(|w| ..)` method takes [`limit::W`](W) writer structure"]
impl crate::Writable for LimitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LIMIT to value 0x7fff_8000"]
impl crate::Resettable for LimitSpec {
    const RESET_VALUE: u32 = 0x7fff_8000;
}
