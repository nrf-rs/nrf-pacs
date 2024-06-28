#[doc = "Register `ENDDELAY` reader"]
pub type R = crate::R<EnddelaySpec>;
#[doc = "Register `ENDDELAY` writer"]
pub type W = crate::W<EnddelaySpec>;
#[doc = "Field `CNT` reader - Time added after the sequence in PWM periods"]
pub type CntR = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - Time added after the sequence in PWM periods"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Time added after the sequence in PWM periods"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time added after the sequence in PWM periods"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<EnddelaySpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Description cluster: Time added after the sequence\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enddelay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enddelay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnddelaySpec;
impl crate::RegisterSpec for EnddelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enddelay::R`](R) reader structure"]
impl crate::Readable for EnddelaySpec {}
#[doc = "`write(|w| ..)` method takes [`enddelay::W`](W) writer structure"]
impl crate::Writable for EnddelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDDELAY to value 0"]
impl crate::Resettable for EnddelaySpec {
    const RESET_VALUE: u32 = 0;
}
