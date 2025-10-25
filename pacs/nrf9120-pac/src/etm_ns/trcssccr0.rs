#[doc = "Register `TRCSSCCR0` reader"]
pub type R = crate::R<Trcssccr0Spec>;
#[doc = "Register `TRCSSCCR0` writer"]
pub type W = crate::W<Trcssccr0Spec>;
#[doc = "Enables the single-shot comparator resource to be reset when it occurs, to enable another comparator match to be detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rst {
    #[doc = "0: Multiple matches can not be detected."]
    Disabled = 0,
    #[doc = "1: Multiple matches can occur."]
    Enabled = 1,
}
impl From<Rst> for bool {
    #[inline(always)]
    fn from(variant: Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - Enables the single-shot comparator resource to be reset when it occurs, to enable another comparator match to be detected"]
pub type RstR = crate::BitReader<Rst>;
impl RstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rst {
        match self.bits {
            false => Rst::Disabled,
            true => Rst::Enabled,
        }
    }
    #[doc = "Multiple matches can not be detected."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rst::Disabled
    }
    #[doc = "Multiple matches can occur."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rst::Enabled
    }
}
#[doc = "Field `RST` writer - Enables the single-shot comparator resource to be reset when it occurs, to enable another comparator match to be detected"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG, Rst>;
impl<'a, REG> RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiple matches can not be detected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::Disabled)
    }
    #[doc = "Multiple matches can occur."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::Enabled)
    }
}
impl R {
    #[doc = "Bit 24 - Enables the single-shot comparator resource to be reset when it occurs, to enable another comparator match to be detected"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Enables the single-shot comparator resource to be reset when it occurs, to enable another comparator match to be detected"]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<'_, Trcssccr0Spec> {
        RstW::new(self, 24)
    }
}
#[doc = "Controls the single-shot comparator.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcssccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcssccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Trcssccr0Spec;
impl crate::RegisterSpec for Trcssccr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcssccr0::R`](R) reader structure"]
impl crate::Readable for Trcssccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`trcssccr0::W`](W) writer structure"]
impl crate::Writable for Trcssccr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCSSCCR0 to value 0"]
impl crate::Resettable for Trcssccr0Spec {}
