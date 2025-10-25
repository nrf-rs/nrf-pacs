#[doc = "Register `CTICONTROL` reader"]
pub type R = crate::R<CticontrolSpec>;
#[doc = "Register `CTICONTROL` writer"]
pub type W = crate::W<CticontrolSpec>;
#[doc = "Enables or disables the CTI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Glben {
    #[doc = "0: All cross-triggering mapping logic functionality is disabled."]
    Disabled = 0,
    #[doc = "1: Cross-triggering mapping logic functionality is enabled."]
    Enabled = 1,
}
impl From<Glben> for bool {
    #[inline(always)]
    fn from(variant: Glben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLBEN` reader - Enables or disables the CTI."]
pub type GlbenR = crate::BitReader<Glben>;
impl GlbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Glben {
        match self.bits {
            false => Glben::Disabled,
            true => Glben::Enabled,
        }
    }
    #[doc = "All cross-triggering mapping logic functionality is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Glben::Disabled
    }
    #[doc = "Cross-triggering mapping logic functionality is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Glben::Enabled
    }
}
#[doc = "Field `GLBEN` writer - Enables or disables the CTI."]
pub type GlbenW<'a, REG> = crate::BitWriter<'a, REG, Glben>;
impl<'a, REG> GlbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All cross-triggering mapping logic functionality is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Glben::Disabled)
    }
    #[doc = "Cross-triggering mapping logic functionality is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Glben::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enables or disables the CTI."]
    #[inline(always)]
    pub fn glben(&self) -> GlbenR {
        GlbenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables or disables the CTI."]
    #[inline(always)]
    pub fn glben(&mut self) -> GlbenW<'_, CticontrolSpec> {
        GlbenW::new(self, 0)
    }
}
#[doc = "CTI Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cticontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cticontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CticontrolSpec;
impl crate::RegisterSpec for CticontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cticontrol::R`](R) reader structure"]
impl crate::Readable for CticontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`cticontrol::W`](W) writer structure"]
impl crate::Writable for CticontrolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTICONTROL to value 0"]
impl crate::Resettable for CticontrolSpec {}
