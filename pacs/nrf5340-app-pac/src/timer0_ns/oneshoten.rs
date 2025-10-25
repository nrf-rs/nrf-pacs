#[doc = "Register `ONESHOTEN[%s]` reader"]
pub type R = crate::R<OneshotenSpec>;
#[doc = "Register `ONESHOTEN[%s]` writer"]
pub type W = crate::W<OneshotenSpec>;
#[doc = "Enable one-shot operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oneshoten {
    #[doc = "0: Disable one-shot operation"]
    Disable = 0,
    #[doc = "1: Enable one-shot operation"]
    Enable = 1,
}
impl From<Oneshoten> for bool {
    #[inline(always)]
    fn from(variant: Oneshoten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONESHOTEN` reader - Enable one-shot operation"]
pub type OneshotenR = crate::BitReader<Oneshoten>;
impl OneshotenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oneshoten {
        match self.bits {
            false => Oneshoten::Disable,
            true => Oneshoten::Enable,
        }
    }
    #[doc = "Disable one-shot operation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Oneshoten::Disable
    }
    #[doc = "Enable one-shot operation"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Oneshoten::Enable
    }
}
#[doc = "Field `ONESHOTEN` writer - Enable one-shot operation"]
pub type OneshotenW<'a, REG> = crate::BitWriter<'a, REG, Oneshoten>;
impl<'a, REG> OneshotenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable one-shot operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshoten::Disable)
    }
    #[doc = "Enable one-shot operation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshoten::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable one-shot operation"]
    #[inline(always)]
    pub fn oneshoten(&self) -> OneshotenR {
        OneshotenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable one-shot operation"]
    #[inline(always)]
    pub fn oneshoten(&mut self) -> OneshotenW<'_, OneshotenSpec> {
        OneshotenW::new(self, 0)
    }
}
#[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n\n\nYou can [`read`](crate::Reg::read) this register and get [`oneshoten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oneshoten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OneshotenSpec;
impl crate::RegisterSpec for OneshotenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oneshoten::R`](R) reader structure"]
impl crate::Readable for OneshotenSpec {}
#[doc = "`write(|w| ..)` method takes [`oneshoten::W`](W) writer structure"]
impl crate::Writable for OneshotenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ONESHOTEN[%s] to value 0"]
impl crate::Resettable for OneshotenSpec {}
