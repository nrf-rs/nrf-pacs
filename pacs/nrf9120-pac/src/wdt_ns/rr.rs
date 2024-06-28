#[doc = "Register `RR[%s]` writer"]
pub type W = crate::W<RrSpec>;
#[doc = "Reload request register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Rr {
    #[doc = "1850885685: Value to request a reload of the watchdog timer"]
    Reload = 1850885685,
}
impl From<Rr> for u32 {
    #[inline(always)]
    fn from(variant: Rr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rr {
    type Ux = u32;
}
impl crate::IsEnum for Rr {}
#[doc = "Field `RR` writer - Reload request register"]
pub type RrW<'a, REG> = crate::FieldWriter<'a, REG, 32, Rr>;
impl<'a, REG> RrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Value to request a reload of the watchdog timer"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(Rr::Reload)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reload request register"]
    #[inline(always)]
    #[must_use]
    pub fn rr(&mut self) -> RrW<RrSpec> {
        RrW::new(self, 0)
    }
}
#[doc = "Description collection: Reload request n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrSpec;
impl crate::RegisterSpec for RrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rr::W`](W) writer structure"]
impl crate::Writable for RrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RR[%s]
to value 0"]
impl crate::Resettable for RrSpec {
    const RESET_VALUE: u32 = 0;
}
