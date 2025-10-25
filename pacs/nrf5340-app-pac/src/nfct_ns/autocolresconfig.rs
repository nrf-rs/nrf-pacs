#[doc = "Register `AUTOCOLRESCONFIG` reader"]
pub type R = crate::R<AutocolresconfigSpec>;
#[doc = "Register `AUTOCOLRESCONFIG` writer"]
pub type W = crate::W<AutocolresconfigSpec>;
#[doc = "Enables/disables auto collision resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Auto collision resolution enabled"]
    Enabled = 0,
    #[doc = "1: Auto collision resolution disabled"]
    Disabled = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Enables/disables auto collision resolution"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Enabled,
            true => Mode::Disabled,
        }
    }
    #[doc = "Auto collision resolution enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mode::Enabled
    }
    #[doc = "Auto collision resolution disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode::Disabled
    }
}
#[doc = "Field `MODE` writer - Enables/disables auto collision resolution"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto collision resolution enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Enabled)
    }
    #[doc = "Auto collision resolution disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enables/disables auto collision resolution"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables/disables auto collision resolution"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, AutocolresconfigSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is activated.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocolresconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocolresconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutocolresconfigSpec;
impl crate::RegisterSpec for AutocolresconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocolresconfig::R`](R) reader structure"]
impl crate::Readable for AutocolresconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`autocolresconfig::W`](W) writer structure"]
impl crate::Writable for AutocolresconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTOCOLRESCONFIG to value 0x02"]
impl crate::Resettable for AutocolresconfigSpec {
    const RESET_VALUE: u32 = 0x02;
}
