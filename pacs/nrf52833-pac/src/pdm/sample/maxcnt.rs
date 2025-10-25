#[doc = "Register `MAXCNT` reader"]
pub type R = crate::R<MaxcntSpec>;
#[doc = "Register `MAXCNT` writer"]
pub type W = crate::W<MaxcntSpec>;
#[doc = "Field `BUFFSIZE` reader - Length of DMA RAM allocation in number of samples"]
pub type BuffsizeR = crate::FieldReader<u16>;
#[doc = "Field `BUFFSIZE` writer - Length of DMA RAM allocation in number of samples"]
pub type BuffsizeW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Length of DMA RAM allocation in number of samples"]
    #[inline(always)]
    pub fn buffsize(&self) -> BuffsizeR {
        BuffsizeR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Length of DMA RAM allocation in number of samples"]
    #[inline(always)]
    pub fn buffsize(&mut self) -> BuffsizeW<'_, MaxcntSpec> {
        BuffsizeW::new(self, 0)
    }
}
#[doc = "Number of samples to allocate memory for in EasyDMA mode\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxcntSpec;
impl crate::RegisterSpec for MaxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxcnt::R`](R) reader structure"]
impl crate::Readable for MaxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`maxcnt::W`](W) writer structure"]
impl crate::Writable for MaxcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAXCNT to value 0"]
impl crate::Resettable for MaxcntSpec {}
