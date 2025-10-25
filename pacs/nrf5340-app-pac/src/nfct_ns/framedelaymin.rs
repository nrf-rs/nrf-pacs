#[doc = "Register `FRAMEDELAYMIN` reader"]
pub type R = crate::R<FramedelayminSpec>;
#[doc = "Register `FRAMEDELAYMIN` writer"]
pub type W = crate::W<FramedelayminSpec>;
#[doc = "Field `FRAMEDELAYMIN` reader - Minimum frame delay in number of 13.56 MHz clock cycles"]
pub type FramedelayminR = crate::FieldReader<u16>;
#[doc = "Field `FRAMEDELAYMIN` writer - Minimum frame delay in number of 13.56 MHz clock cycles"]
pub type FramedelayminW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Minimum frame delay in number of 13.56 MHz clock cycles"]
    #[inline(always)]
    pub fn framedelaymin(&self) -> FramedelayminR {
        FramedelayminR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum frame delay in number of 13.56 MHz clock cycles"]
    #[inline(always)]
    pub fn framedelaymin(&mut self) -> FramedelayminW<'_, FramedelayminSpec> {
        FramedelayminW::new(self, 0)
    }
}
#[doc = "Minimum frame delay\n\nYou can [`read`](crate::Reg::read) this register and get [`framedelaymin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framedelaymin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FramedelayminSpec;
impl crate::RegisterSpec for FramedelayminSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framedelaymin::R`](R) reader structure"]
impl crate::Readable for FramedelayminSpec {}
#[doc = "`write(|w| ..)` method takes [`framedelaymin::W`](W) writer structure"]
impl crate::Writable for FramedelayminSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMEDELAYMIN to value 0x0480"]
impl crate::Resettable for FramedelayminSpec {
    const RESET_VALUE: u32 = 0x0480;
}
