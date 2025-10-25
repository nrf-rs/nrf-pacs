#[doc = "Register `ERASEALL` reader"]
pub type R = crate::R<EraseallSpec>;
#[doc = "Register `ERASEALL` writer"]
pub type W = crate::W<EraseallSpec>;
#[doc = "Erase all non-volatile memory including UICR registers. Note that the erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eraseall {
    #[doc = "0: No operation"]
    NoOperation = 0,
    #[doc = "1: Start erase of chip"]
    Erase = 1,
}
impl From<Eraseall> for bool {
    #[inline(always)]
    fn from(variant: Eraseall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASEALL` reader - Erase all non-volatile memory including UICR registers. Note that the erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased."]
pub type EraseallR = crate::BitReader<Eraseall>;
impl EraseallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eraseall {
        match self.bits {
            false => Eraseall::NoOperation,
            true => Eraseall::Erase,
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == Eraseall::NoOperation
    }
    #[doc = "Start erase of chip"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == Eraseall::Erase
    }
}
#[doc = "Field `ERASEALL` writer - Erase all non-volatile memory including UICR registers. Note that the erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased."]
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
    #[doc = "Start erase of chip"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(Eraseall::Erase)
    }
}
impl R {
    #[doc = "Bit 0 - Erase all non-volatile memory including UICR registers. Note that the erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased."]
    #[inline(always)]
    pub fn eraseall(&self) -> EraseallR {
        EraseallR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase all non-volatile memory including UICR registers. Note that the erase must be enabled using CONFIG.WEN before the non-volatile memory can be erased."]
    #[inline(always)]
    pub fn eraseall(&mut self) -> EraseallW<'_, EraseallSpec> {
        EraseallW::new(self, 0)
    }
}
#[doc = "Register for erasing all non-volatile user memory\n\nYou can [`read`](crate::Reg::read) this register and get [`eraseall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EraseallSpec;
impl crate::RegisterSpec for EraseallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eraseall::R`](R) reader structure"]
impl crate::Readable for EraseallSpec {}
#[doc = "`write(|w| ..)` method takes [`eraseall::W`](W) writer structure"]
impl crate::Writable for EraseallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEALL to value 0"]
impl crate::Resettable for EraseallSpec {}
