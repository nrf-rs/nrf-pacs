#[doc = "Register `ITATBCTR2` writer"]
pub type W = crate::W<Itatbctr2Spec>;
#[doc = "Field `ATREADYS` writer - Set the value of atreadys."]
pub type AtreadysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFVALIDS` writer - Set the value of afvalids."]
pub type AfvalidsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set the value of atreadys."]
    #[inline(always)]
    pub fn atreadys(&mut self) -> AtreadysW<'_, Itatbctr2Spec> {
        AtreadysW::new(self, 0)
    }
    #[doc = "Bit 1 - Set the value of afvalids."]
    #[inline(always)]
    pub fn afvalids(&mut self) -> AfvalidsW<'_, Itatbctr2Spec> {
        AfvalidsW::new(self, 1)
    }
}
#[doc = "Integration Test ATB Control Register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbctr2Spec;
impl crate::RegisterSpec for Itatbctr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`itatbctr2::W`](W) writer structure"]
impl crate::Writable for Itatbctr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITATBCTR2 to value 0"]
impl crate::Resettable for Itatbctr2Spec {}
