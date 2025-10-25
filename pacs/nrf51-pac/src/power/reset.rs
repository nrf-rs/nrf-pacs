#[doc = "Register `RESET` reader"]
pub type R = crate::R<ResetSpec>;
#[doc = "Register `RESET` writer"]
pub type W = crate::W<ResetSpec>;
#[doc = "Enable or disable pin reset in debug interface mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "0: Pin reset in debug interface mode disabled."]
    Disabled = 0,
    #[doc = "1: Pin reset in debug interface mode enabled."]
    Enabled = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` reader - Enable or disable pin reset in debug interface mode."]
pub type ResetR = crate::BitReader<Reset>;
impl ResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reset {
        match self.bits {
            false => Reset::Disabled,
            true => Reset::Enabled,
        }
    }
    #[doc = "Pin reset in debug interface mode disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Reset::Disabled
    }
    #[doc = "Pin reset in debug interface mode enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Reset::Enabled
    }
}
#[doc = "Field `RESET` writer - Enable or disable pin reset in debug interface mode."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin reset in debug interface mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Disabled)
    }
    #[doc = "Pin reset in debug interface mode enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable pin reset in debug interface mode."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable pin reset in debug interface mode."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, ResetSpec> {
        ResetW::new(self, 0)
    }
}
#[doc = "Pin reset functionality configuration register. This register is a retained register.\n\nYou can [`read`](crate::Reg::read) this register and get [`reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetSpec;
impl crate::RegisterSpec for ResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset::R`](R) reader structure"]
impl crate::Readable for ResetSpec {}
#[doc = "`write(|w| ..)` method takes [`reset::W`](W) writer structure"]
impl crate::Writable for ResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESET to value 0"]
impl crate::Resettable for ResetSpec {}
