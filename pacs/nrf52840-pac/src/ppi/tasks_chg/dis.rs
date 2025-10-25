#[doc = "Register `DIS` writer"]
pub type W = crate::W<DisSpec>;
#[doc = "Disable channel group n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dis {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<Dis> for bool {
    #[inline(always)]
    fn from(variant: Dis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS` writer - Disable channel group n"]
pub type DisW<'a, REG> = crate::BitWriter<'a, REG, Dis>;
impl<'a, REG> DisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Dis::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Disable channel group n"]
    #[inline(always)]
    pub fn dis(&mut self) -> DisW<'_, DisSpec> {
        DisW::new(self, 0)
    }
}
#[doc = "Description cluster: Disable channel group n\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisSpec;
impl crate::RegisterSpec for DisSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dis::W`](W) writer structure"]
impl crate::Writable for DisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIS to value 0"]
impl crate::Resettable for DisSpec {}
