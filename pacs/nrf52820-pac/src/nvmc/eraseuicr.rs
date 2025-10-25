#[doc = "Register `ERASEUICR` writer"]
pub type W = crate::W<EraseuicrSpec>;
#[doc = "Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eraseuicr {
    #[doc = "0: No operation"]
    NoOperation = 0,
    #[doc = "1: Start erase of UICR"]
    Erase = 1,
}
impl From<Eraseuicr> for bool {
    #[inline(always)]
    fn from(variant: Eraseuicr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASEUICR` writer - Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased."]
pub type EraseuicrW<'a, REG> = crate::BitWriter<'a, REG, Eraseuicr>;
impl<'a, REG> EraseuicrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Eraseuicr::NoOperation)
    }
    #[doc = "Start erase of UICR"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(Eraseuicr::Erase)
    }
}
impl W {
    #[doc = "Bit 0 - Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseuicr(&mut self) -> EraseuicrW<'_, EraseuicrSpec> {
        EraseuicrW::new(self, 0)
    }
}
#[doc = "Register for erasing user information configuration registers\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseuicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EraseuicrSpec;
impl crate::RegisterSpec for EraseuicrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eraseuicr::W`](W) writer structure"]
impl crate::Writable for EraseuicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEUICR to value 0"]
impl crate::Resettable for EraseuicrSpec {}
