#[doc = "Register `ITCTRL` reader"]
pub type R = crate::R<ItctrlSpec>;
#[doc = "Register `ITCTRL` writer"]
pub type W = crate::W<ItctrlSpec>;
#[doc = "Field `INTEGRATION_MODE` reader - Allows the component to switch from functional mode to integration mode or back."]
pub type IntegrationModeR = crate::BitReader;
#[doc = "Field `INTEGRATION_MODE` writer - Allows the component to switch from functional mode to integration mode or back."]
pub type IntegrationModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Allows the component to switch from functional mode to integration mode or back."]
    #[inline(always)]
    pub fn integration_mode(&self) -> IntegrationModeR {
        IntegrationModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allows the component to switch from functional mode to integration mode or back."]
    #[inline(always)]
    pub fn integration_mode(&mut self) -> IntegrationModeW<'_, ItctrlSpec> {
        IntegrationModeW::new(self, 0)
    }
}
#[doc = "Integration Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`itctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
