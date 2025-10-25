#[doc = "Register `VREQH` reader"]
pub type R = crate::R<VreqhSpec>;
#[doc = "Register `VREQH` writer"]
pub type W = crate::W<VreqhSpec>;
#[doc = "Request high voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vreqh {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Vreqh> for bool {
    #[inline(always)]
    fn from(variant: Vreqh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREQH` reader - Request high voltage"]
pub type VreqhR = crate::BitReader<Vreqh>;
impl VreqhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vreqh {
        match self.bits {
            false => Vreqh::Disabled,
            true => Vreqh::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vreqh::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vreqh::Enabled
    }
}
#[doc = "Field `VREQH` writer - Request high voltage"]
pub type VreqhW<'a, REG> = crate::BitWriter<'a, REG, Vreqh>;
impl<'a, REG> VreqhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vreqh::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vreqh::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Request high voltage"]
    #[inline(always)]
    pub fn vreqh(&self) -> VreqhR {
        VreqhR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request high voltage"]
    #[inline(always)]
    pub fn vreqh(&mut self) -> VreqhW<'_, VreqhSpec> {
        VreqhW::new(self, 0)
    }
}
#[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready\n\nYou can [`read`](crate::Reg::read) this register and get [`vreqh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreqh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VreqhSpec;
impl crate::RegisterSpec for VreqhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreqh::R`](R) reader structure"]
impl crate::Readable for VreqhSpec {}
#[doc = "`write(|w| ..)` method takes [`vreqh::W`](W) writer structure"]
impl crate::Writable for VreqhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREQH to value 0"]
impl crate::Resettable for VreqhSpec {}
