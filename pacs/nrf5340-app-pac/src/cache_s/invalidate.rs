#[doc = "Register `INVALIDATE` writer"]
pub type W = crate::W<InvalidateSpec>;
#[doc = "Invalidate the cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invalidate {
    #[doc = "1: Invalidate the cache"]
    Invalidate = 1,
}
impl From<Invalidate> for bool {
    #[inline(always)]
    fn from(variant: Invalidate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVALIDATE` writer - Invalidate the cache"]
pub type InvalidateW<'a, REG> = crate::BitWriter<'a, REG, Invalidate>;
impl<'a, REG> InvalidateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invalidate the cache"]
    #[inline(always)]
    pub fn invalidate(self) -> &'a mut crate::W<REG> {
        self.variant(Invalidate::Invalidate)
    }
}
impl W {
    #[doc = "Bit 0 - Invalidate the cache"]
    #[inline(always)]
    pub fn invalidate(&mut self) -> InvalidateW<'_, InvalidateSpec> {
        InvalidateW::new(self, 0)
    }
}
#[doc = "Invalidate the cache.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`invalidate::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InvalidateSpec;
impl crate::RegisterSpec for InvalidateSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`invalidate::W`](W) writer structure"]
impl crate::Writable for InvalidateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INVALIDATE to value 0"]
impl crate::Resettable for InvalidateSpec {}
