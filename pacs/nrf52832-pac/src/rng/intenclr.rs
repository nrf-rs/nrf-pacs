#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to Disable interrupt for VALRDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Valrdy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Valrdy> for bool {
    #[inline(always)]
    fn from(variant: Valrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALRDY` reader - Write '1' to Disable interrupt for VALRDY event"]
pub type ValrdyR = crate::BitReader<Valrdy>;
impl ValrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Valrdy {
        match self.bits {
            false => Valrdy::Disabled,
            true => Valrdy::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Valrdy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Valrdy::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for VALRDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValrdyWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<ValrdyWO> for bool {
    #[inline(always)]
    fn from(variant: ValrdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALRDY` writer - Write '1' to Disable interrupt for VALRDY event"]
pub type ValrdyW<'a, REG> = crate::BitWriter<'a, REG, ValrdyWO>;
impl<'a, REG> ValrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ValrdyWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to Disable interrupt for VALRDY event"]
    #[inline(always)]
    pub fn valrdy(&self) -> ValrdyR {
        ValrdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Disable interrupt for VALRDY event"]
    #[inline(always)]
    pub fn valrdy(&mut self) -> ValrdyW<'_, IntenclrSpec> {
        ValrdyW::new(self, 0)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
