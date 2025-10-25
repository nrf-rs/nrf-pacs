#[doc = "Register `SYSTEMOFF` writer"]
pub type W = crate::W<SystemoffSpec>;
#[doc = "Enable System OFF mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Systemoff {
    #[doc = "1: Enable System OFF mode"]
    Enter = 1,
}
impl From<Systemoff> for bool {
    #[inline(always)]
    fn from(variant: Systemoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTEMOFF` writer - Enable System OFF mode"]
pub type SystemoffW<'a, REG> = crate::BitWriter<'a, REG, Systemoff>;
impl<'a, REG> SystemoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable System OFF mode"]
    #[inline(always)]
    pub fn enter(self) -> &'a mut crate::W<REG> {
        self.variant(Systemoff::Enter)
    }
}
impl W {
    #[doc = "Bit 0 - Enable System OFF mode"]
    #[inline(always)]
    pub fn systemoff(&mut self) -> SystemoffW<'_, SystemoffSpec> {
        SystemoffW::new(self, 0)
    }
}
#[doc = "System OFF register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemoff::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemoffSpec;
impl crate::RegisterSpec for SystemoffSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`systemoff::W`](W) writer structure"]
impl crate::Writable for SystemoffSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTEMOFF to value 0"]
impl crate::Resettable for SystemoffSpec {}
