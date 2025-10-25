#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Disable interrupt on POFWARN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pofwarn {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Pofwarn> for bool {
    #[inline(always)]
    fn from(variant: Pofwarn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFWARN` reader - Disable interrupt on POFWARN event."]
pub type PofwarnR = crate::BitReader<Pofwarn>;
impl PofwarnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pofwarn {
        match self.bits {
            false => Pofwarn::Disabled,
            true => Pofwarn::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pofwarn::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pofwarn::Enabled
    }
}
#[doc = "Disable interrupt on POFWARN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PofwarnWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<PofwarnWO> for bool {
    #[inline(always)]
    fn from(variant: PofwarnWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFWARN` writer - Disable interrupt on POFWARN event."]
pub type PofwarnW<'a, REG> = crate::BitWriter<'a, REG, PofwarnWO>;
impl<'a, REG> PofwarnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PofwarnWO::Clear)
    }
}
impl R {
    #[doc = "Bit 2 - Disable interrupt on POFWARN event."]
    #[inline(always)]
    pub fn pofwarn(&self) -> PofwarnR {
        PofwarnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Disable interrupt on POFWARN event."]
    #[inline(always)]
    pub fn pofwarn(&mut self) -> PofwarnW<'_, IntenclrSpec> {
        PofwarnW::new(self, 2)
    }
}
#[doc = "Interrupt enable clear register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
