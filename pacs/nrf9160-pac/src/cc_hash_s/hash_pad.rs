#[doc = "Register `HASH_PAD` reader"]
pub type R = crate::R<HashPadSpec>;
#[doc = "Register `HASH_PAD` writer"]
pub type W = crate::W<HashPadSpec>;
#[doc = "Configure hardware padding feature.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable hardware padding feature."]
    Disable = 0,
    #[doc = "1: Enable hardware padding feature."]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Configure hardware padding feature."]
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
    #[doc = "Disable hardware padding feature."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "Enable hardware padding feature."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - Configure hardware padding feature."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable hardware padding feature."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Enable hardware padding feature."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Configure hardware padding feature."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure hardware padding feature."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, HashPadSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Enable the hardware padding feature of the HASH engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_pad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_pad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashPadSpec;
impl crate::RegisterSpec for HashPadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_pad::R`](R) reader structure"]
impl crate::Readable for HashPadSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_pad::W`](W) writer structure"]
impl crate::Writable for HashPadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_PAD to value 0x01"]
impl crate::Resettable for HashPadSpec {
    const RESET_VALUE: u32 = 0x01;
}
