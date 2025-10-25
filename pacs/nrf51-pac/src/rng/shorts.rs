#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between VALRDY event and STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValrdyStop {
    #[doc = "0: Shortcut disabled."]
    Disabled = 0,
    #[doc = "1: Shortcut enabled."]
    Enabled = 1,
}
impl From<ValrdyStop> for bool {
    #[inline(always)]
    fn from(variant: ValrdyStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALRDY_STOP` reader - Shortcut between VALRDY event and STOP task."]
pub type ValrdyStopR = crate::BitReader<ValrdyStop>;
impl ValrdyStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ValrdyStop {
        match self.bits {
            false => ValrdyStop::Disabled,
            true => ValrdyStop::Enabled,
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ValrdyStop::Disabled
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ValrdyStop::Enabled
    }
}
#[doc = "Field `VALRDY_STOP` writer - Shortcut between VALRDY event and STOP task."]
pub type ValrdyStopW<'a, REG> = crate::BitWriter<'a, REG, ValrdyStop>;
impl<'a, REG> ValrdyStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ValrdyStop::Disabled)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ValrdyStop::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between VALRDY event and STOP task."]
    #[inline(always)]
    pub fn valrdy_stop(&self) -> ValrdyStopR {
        ValrdyStopR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between VALRDY event and STOP task."]
    #[inline(always)]
    pub fn valrdy_stop(&mut self) -> ValrdyStopW<'_, ShortsSpec> {
        ValrdyStopW::new(self, 0)
    }
}
#[doc = "Shortcuts for the RNG.\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {}
