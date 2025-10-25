#[doc = "Register `OVERRIDE4` reader"]
pub type R = crate::R<Override4Spec>;
#[doc = "Register `OVERRIDE4` writer"]
pub type W = crate::W<Override4Spec>;
#[doc = "Field `OVERRIDE4` reader - Trim value override 4."]
pub type Override4R = crate::FieldReader<u32>;
#[doc = "Field `OVERRIDE4` writer - Trim value override 4."]
pub type Override4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Enable or disable override of default trim values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Override trim values disabled."]
    Disabled = 0,
    #[doc = "1: Override trim values enabled."]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable or disable override of default trim values."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disabled,
            true => Enable::Enabled,
        }
    }
    #[doc = "Override trim values disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Override trim values enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - Enable or disable override of default trim values."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override trim values disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Override trim values enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:27 - Trim value override 4."]
    #[inline(always)]
    pub fn override4(&self) -> Override4R {
        Override4R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - Enable or disable override of default trim values."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - Trim value override 4."]
    #[inline(always)]
    pub fn override4(&mut self) -> Override4W<'_, Override4Spec> {
        Override4W::new(self, 0)
    }
    #[doc = "Bit 31 - Enable or disable override of default trim values."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Override4Spec> {
        EnableW::new(self, 31)
    }
}
#[doc = "Trim value override register 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`override4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`override4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Override4Spec;
impl crate::RegisterSpec for Override4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`override4::R`](R) reader structure"]
impl crate::Readable for Override4Spec {}
#[doc = "`write(|w| ..)` method takes [`override4::W`](W) writer structure"]
impl crate::Writable for Override4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OVERRIDE4 to value 0"]
impl crate::Resettable for Override4Spec {}
