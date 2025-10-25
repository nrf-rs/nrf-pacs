#[doc = "Register `ERASE` writer"]
pub type W = crate::W<EraseSpec>;
#[doc = "Erase the cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erase {
    #[doc = "1: Erase cache"]
    Erase = 1,
}
impl From<Erase> for bool {
    #[inline(always)]
    fn from(variant: Erase) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASE` writer - Erase the cache"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG, Erase>;
impl<'a, REG> EraseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase cache"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(Erase::Erase)
    }
}
impl W {
    #[doc = "Bit 0 - Erase the cache"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<'_, EraseSpec> {
        EraseW::new(self, 0)
    }
}
#[doc = "Erase the cache.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erase::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EraseSpec;
impl crate::RegisterSpec for EraseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`erase::W`](W) writer structure"]
impl crate::Writable for EraseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASE to value 0"]
impl crate::Resettable for EraseSpec {}
