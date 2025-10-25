#[doc = "Register `ITCTRL` reader"]
pub type R = crate::R<ItctrlSpec>;
#[doc = "Register `ITCTRL` writer"]
pub type W = crate::W<ItctrlSpec>;
#[doc = "Enables the component to switch from functional mode to integration mode and back. If no integration functionality is implemented, this register must read as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Integrationmode {
    #[doc = "0: Integration mode is disabled."]
    Disabled = 0,
    #[doc = "1: Integration mode is Enabled."]
    Enabled = 1,
}
impl From<Integrationmode> for bool {
    #[inline(always)]
    fn from(variant: Integrationmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEGRATIONMODE` reader - Enables the component to switch from functional mode to integration mode and back. If no integration functionality is implemented, this register must read as zero."]
pub type IntegrationmodeR = crate::BitReader<Integrationmode>;
impl IntegrationmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Integrationmode {
        match self.bits {
            false => Integrationmode::Disabled,
            true => Integrationmode::Enabled,
        }
    }
    #[doc = "Integration mode is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Integrationmode::Disabled
    }
    #[doc = "Integration mode is Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Integrationmode::Enabled
    }
}
#[doc = "Field `INTEGRATIONMODE` writer - Enables the component to switch from functional mode to integration mode and back. If no integration functionality is implemented, this register must read as zero."]
pub type IntegrationmodeW<'a, REG> = crate::BitWriter<'a, REG, Integrationmode>;
impl<'a, REG> IntegrationmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Integration mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Integrationmode::Disabled)
    }
    #[doc = "Integration mode is Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Integrationmode::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enables the component to switch from functional mode to integration mode and back. If no integration functionality is implemented, this register must read as zero."]
    #[inline(always)]
    pub fn integrationmode(&self) -> IntegrationmodeR {
        IntegrationmodeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the component to switch from functional mode to integration mode and back. If no integration functionality is implemented, this register must read as zero."]
    #[inline(always)]
    pub fn integrationmode(&mut self) -> IntegrationmodeW<'_, ItctrlSpec> {
        IntegrationmodeW::new(self, 0)
    }
}
#[doc = "Used to enable topology detection. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for integration testing and topology solving.\n\nYou can [`read`](crate::Reg::read) this register and get [`itctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ItctrlSpec;
impl crate::RegisterSpec for ItctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itctrl::R`](R) reader structure"]
impl crate::Readable for ItctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`itctrl::W`](W) writer structure"]
impl crate::Writable for ItctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITCTRL to value 0"]
impl crate::Resettable for ItctrlSpec {}
