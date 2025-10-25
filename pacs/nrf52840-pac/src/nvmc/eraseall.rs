#[doc = "Register `ERASEALL` writer"]
pub type W = crate::W<EraseallSpec>;
#[doc = "Erase all non-volatile memory including UICR registers. The erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eraseall {
    #[doc = "0: No operation"]
    NoOperation = 0,
    #[doc = "1: Start chip erase"]
    Erase = 1,
}
impl From<Eraseall> for bool {
    #[inline(always)]
    fn from(variant: Eraseall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASEALL` writer - Erase all non-volatile memory including UICR registers. The erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased."]
pub type EraseallW<'a, REG> = crate::BitWriter<'a, REG, Eraseall>;
impl<'a, REG> EraseallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Eraseall::NoOperation)
    }
    #[doc = "Start chip erase"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(Eraseall::Erase)
    }
}
impl W {
    #[doc = "Bit 0 - Erase all non-volatile memory including UICR registers. The erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased."]
    #[inline(always)]
    pub fn eraseall(&mut self) -> EraseallW<'_, EraseallSpec> {
        EraseallW::new(self, 0)
    }
}
#[doc = "Register for erasing all non-volatile user memory\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseall::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EraseallSpec;
impl crate::RegisterSpec for EraseallSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eraseall::W`](W) writer structure"]
impl crate::Writable for EraseallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEALL to value 0"]
impl crate::Resettable for EraseallSpec {}
