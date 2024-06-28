#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event END and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndStart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<EndStart> for bool {
    #[inline(always)]
    fn from(variant: EndStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END_START` reader - Shortcut between event END and task START"]
pub type EndStartR = crate::BitReader<EndStart>;
impl EndStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EndStart {
        match self.bits {
            false => EndStart::Disabled,
            true => EndStart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EndStart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EndStart::Enabled
    }
}
#[doc = "Field `END_START` writer - Shortcut between event END and task START"]
pub type EndStartW<'a, REG> = crate::BitWriter<'a, REG, EndStart>;
impl<'a, REG> EndStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndStart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndStart::Enabled)
    }
}
impl R {
    #[doc = "Bit 17 - Shortcut between event END and task START"]
    #[inline(always)]
    pub fn end_start(&self) -> EndStartR {
        EndStartR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Shortcut between event END and task START"]
    #[inline(always)]
    #[must_use]
    pub fn end_start(&mut self) -> EndStartW<ShortsSpec> {
        EndStartW::new(self, 17)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shorts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {
    const RESET_VALUE: u32 = 0;
}
