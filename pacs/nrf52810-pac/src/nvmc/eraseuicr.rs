#[doc = "Register `ERASEUICR` reader"]
pub type R = crate::R<EraseuicrSpec>;
#[doc = "Register `ERASEUICR` writer"]
pub type W = crate::W<EraseuicrSpec>;
#[doc = "Register starting erase of all user information configuration registers. Note that the erase must be enabled using CONFIG.WEN before the UICR can be erased.\n\nValue on reset: 0"]
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
#[doc = "Field `ERASEUICR` reader - Register starting erase of all user information configuration registers. Note that the erase must be enabled using CONFIG.WEN before the UICR can be erased."]
pub type EraseuicrR = crate::BitReader<Eraseuicr>;
impl EraseuicrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eraseuicr {
        match self.bits {
            false => Eraseuicr::NoOperation,
            true => Eraseuicr::Erase,
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == Eraseuicr::NoOperation
    }
    #[doc = "Start erase of UICR"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == Eraseuicr::Erase
    }
}
#[doc = "Field `ERASEUICR` writer - Register starting erase of all user information configuration registers. Note that the erase must be enabled using CONFIG.WEN before the UICR can be erased."]
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
impl R {
    #[doc = "Bit 0 - Register starting erase of all user information configuration registers. Note that the erase must be enabled using CONFIG.WEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseuicr(&self) -> EraseuicrR {
        EraseuicrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register starting erase of all user information configuration registers. Note that the erase must be enabled using CONFIG.WEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseuicr(&mut self) -> EraseuicrW<'_, EraseuicrSpec> {
        EraseuicrW::new(self, 0)
    }
}
#[doc = "Register for erasing user information configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`eraseuicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseuicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EraseuicrSpec;
impl crate::RegisterSpec for EraseuicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eraseuicr::R`](R) reader structure"]
impl crate::Readable for EraseuicrSpec {}
#[doc = "`write(|w| ..)` method takes [`eraseuicr::W`](W) writer structure"]
impl crate::Writable for EraseuicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEUICR to value 0"]
impl crate::Resettable for EraseuicrSpec {}
