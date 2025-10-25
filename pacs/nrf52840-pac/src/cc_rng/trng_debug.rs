#[doc = "Register `TRNG_DEBUG` reader"]
pub type R = crate::R<TrngDebugSpec>;
#[doc = "Register `TRNG_DEBUG` writer"]
pub type W = crate::W<TrngDebugSpec>;
#[doc = "Bypass the von Neumann corrector post-processing test, including the 32 consecutive bits test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VncBypass {
    #[doc = "0: von Neumann corrector post-processing is active"]
    Disabled = 0,
    #[doc = "1: Bypass the von Neumann corrector"]
    Enabled = 1,
}
impl From<VncBypass> for bool {
    #[inline(always)]
    fn from(variant: VncBypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VNC_BYPASS` reader - Bypass the von Neumann corrector post-processing test, including the 32 consecutive bits test."]
pub type VncBypassR = crate::BitReader<VncBypass>;
impl VncBypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VncBypass {
        match self.bits {
            false => VncBypass::Disabled,
            true => VncBypass::Enabled,
        }
    }
    #[doc = "von Neumann corrector post-processing is active"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VncBypass::Disabled
    }
    #[doc = "Bypass the von Neumann corrector"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VncBypass::Enabled
    }
}
#[doc = "Field `VNC_BYPASS` writer - Bypass the von Neumann corrector post-processing test, including the 32 consecutive bits test."]
pub type VncBypassW<'a, REG> = crate::BitWriter<'a, REG, VncBypass>;
impl<'a, REG> VncBypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "von Neumann corrector post-processing is active"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VncBypass::Disabled)
    }
    #[doc = "Bypass the von Neumann corrector"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VncBypass::Enabled)
    }
}
#[doc = "Bypass the Continuous Random Number Generator Test (CRNGT).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrngtBypass {
    #[doc = "0: CRNGT is active"]
    Disabled = 0,
    #[doc = "1: Bypass CRNGT"]
    Enabled = 1,
}
impl From<CrngtBypass> for bool {
    #[inline(always)]
    fn from(variant: CrngtBypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRNGT_BYPASS` reader - Bypass the Continuous Random Number Generator Test (CRNGT)."]
pub type CrngtBypassR = crate::BitReader<CrngtBypass>;
impl CrngtBypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrngtBypass {
        match self.bits {
            false => CrngtBypass::Disabled,
            true => CrngtBypass::Enabled,
        }
    }
    #[doc = "CRNGT is active"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CrngtBypass::Disabled
    }
    #[doc = "Bypass CRNGT"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CrngtBypass::Enabled
    }
}
#[doc = "Field `CRNGT_BYPASS` writer - Bypass the Continuous Random Number Generator Test (CRNGT)."]
pub type CrngtBypassW<'a, REG> = crate::BitWriter<'a, REG, CrngtBypass>;
impl<'a, REG> CrngtBypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRNGT is active"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CrngtBypass::Disabled)
    }
    #[doc = "Bypass CRNGT"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CrngtBypass::Enabled)
    }
}
#[doc = "Bypass the autocorrelation test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutocorrBypass {
    #[doc = "0: Autocorrelation test is active"]
    Disabled = 0,
    #[doc = "1: Bypass the autocorrelation test"]
    Enabled = 1,
}
impl From<AutocorrBypass> for bool {
    #[inline(always)]
    fn from(variant: AutocorrBypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCORR_BYPASS` reader - Bypass the autocorrelation test."]
pub type AutocorrBypassR = crate::BitReader<AutocorrBypass>;
impl AutocorrBypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutocorrBypass {
        match self.bits {
            false => AutocorrBypass::Disabled,
            true => AutocorrBypass::Enabled,
        }
    }
    #[doc = "Autocorrelation test is active"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AutocorrBypass::Disabled
    }
    #[doc = "Bypass the autocorrelation test"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AutocorrBypass::Enabled
    }
}
#[doc = "Field `AUTOCORR_BYPASS` writer - Bypass the autocorrelation test."]
pub type AutocorrBypassW<'a, REG> = crate::BitWriter<'a, REG, AutocorrBypass>;
impl<'a, REG> AutocorrBypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Autocorrelation test is active"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AutocorrBypass::Disabled)
    }
    #[doc = "Bypass the autocorrelation test"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AutocorrBypass::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Bypass the von Neumann corrector post-processing test, including the 32 consecutive bits test."]
    #[inline(always)]
    pub fn vnc_bypass(&self) -> VncBypassR {
        VncBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bypass the Continuous Random Number Generator Test (CRNGT)."]
    #[inline(always)]
    pub fn crngt_bypass(&self) -> CrngtBypassR {
        CrngtBypassR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bypass the autocorrelation test."]
    #[inline(always)]
    pub fn autocorr_bypass(&self) -> AutocorrBypassR {
        AutocorrBypassR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Bypass the von Neumann corrector post-processing test, including the 32 consecutive bits test."]
    #[inline(always)]
    pub fn vnc_bypass(&mut self) -> VncBypassW<'_, TrngDebugSpec> {
        VncBypassW::new(self, 1)
    }
    #[doc = "Bit 2 - Bypass the Continuous Random Number Generator Test (CRNGT)."]
    #[inline(always)]
    pub fn crngt_bypass(&mut self) -> CrngtBypassW<'_, TrngDebugSpec> {
        CrngtBypassW::new(self, 2)
    }
    #[doc = "Bit 3 - Bypass the autocorrelation test."]
    #[inline(always)]
    pub fn autocorr_bypass(&mut self) -> AutocorrBypassW<'_, TrngDebugSpec> {
        AutocorrBypassW::new(self, 3)
    }
}
#[doc = "Debug register for the TRNG. This register is used to bypass TRNG tests in hardware.\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngDebugSpec;
impl crate::RegisterSpec for TrngDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_debug::R`](R) reader structure"]
impl crate::Readable for TrngDebugSpec {}
#[doc = "`write(|w| ..)` method takes [`trng_debug::W`](W) writer structure"]
impl crate::Writable for TrngDebugSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRNG_DEBUG to value 0"]
impl crate::Resettable for TrngDebugSpec {}
