#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event ENDKSGEN and task CRYPT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndksgenCrypt {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<EndksgenCrypt> for bool {
    #[inline(always)]
    fn from(variant: EndksgenCrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDKSGEN_CRYPT` reader - Shortcut between event ENDKSGEN and task CRYPT"]
pub type EndksgenCryptR = crate::BitReader<EndksgenCrypt>;
impl EndksgenCryptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EndksgenCrypt {
        match self.bits {
            false => EndksgenCrypt::Disabled,
            true => EndksgenCrypt::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EndksgenCrypt::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EndksgenCrypt::Enabled
    }
}
#[doc = "Field `ENDKSGEN_CRYPT` writer - Shortcut between event ENDKSGEN and task CRYPT"]
pub type EndksgenCryptW<'a, REG> = crate::BitWriter<'a, REG, EndksgenCrypt>;
impl<'a, REG> EndksgenCryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndksgenCrypt::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndksgenCrypt::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event ENDKSGEN and task CRYPT"]
    #[inline(always)]
    pub fn endksgen_crypt(&self) -> EndksgenCryptR {
        EndksgenCryptR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event ENDKSGEN and task CRYPT"]
    #[inline(always)]
    pub fn endksgen_crypt(&mut self) -> EndksgenCryptW<'_, ShortsSpec> {
        EndksgenCryptW::new(self, 0)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
