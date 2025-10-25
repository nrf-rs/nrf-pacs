#[doc = "Register `HASH_PAD_FORCE` reader"]
pub type R = crate::R<HashPadForceSpec>;
#[doc = "Register `HASH_PAD_FORCE` writer"]
pub type W = crate::W<HashPadForceSpec>;
#[doc = "Trigger hardware padding operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Do not force hardware padding to trigger."]
    Disable = 0,
    #[doc = "1: Force hardware padding to trigger."]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Trigger hardware padding operation."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "Do not force hardware padding to trigger."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "Force hardware padding to trigger."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - Trigger hardware padding operation."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not force hardware padding to trigger."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Force hardware padding to trigger."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl R {
    #[doc = "Bit 2 - Trigger hardware padding operation."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Trigger hardware padding operation."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, HashPadForceSpec> {
        EnableW::new(self, 2)
    }
}
#[doc = "Force the hardware padding operation to trigger if the input data length is zero bytes.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_pad_force::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_pad_force::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashPadForceSpec;
impl crate::RegisterSpec for HashPadForceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_pad_force::R`](R) reader structure"]
impl crate::Readable for HashPadForceSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_pad_force::W`](W) writer structure"]
impl crate::Writable for HashPadForceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_PAD_FORCE to value 0"]
impl crate::Resettable for HashPadForceSpec {}
