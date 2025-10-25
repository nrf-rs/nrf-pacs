#[doc = "Register `SUBSCRIBE_CLR[%s]` reader"]
pub type R = crate::R<SubscribeClrSpec>;
#[doc = "Register `SUBSCRIBE_CLR[%s]` writer"]
pub type W = crate::W<SubscribeClrSpec>;
#[doc = "Field `CHIDX` reader - DPPI channel that task CLR\\[n\\] will subscribe to"]
pub type ChidxR = crate::FieldReader;
#[doc = "Field `CHIDX` writer - DPPI channel that task CLR\\[n\\] will subscribe to"]
pub type ChidxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable subscription"]
    Disabled = 0,
    #[doc = "1: Enable subscription"]
    Enabled = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - "]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disabled,
            true => En::Enabled,
        }
    }
    #[doc = "Disable subscription"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En::Disabled
    }
    #[doc = "Enable subscription"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == En::Enabled
    }
}
#[doc = "Field `EN` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable subscription"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disabled)
    }
    #[doc = "Enable subscription"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - DPPI channel that task CLR\\[n\\] will subscribe to"]
    #[inline(always)]
    pub fn chidx(&self) -> ChidxR {
        ChidxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DPPI channel that task CLR\\[n\\] will subscribe to"]
    #[inline(always)]
    pub fn chidx(&mut self) -> ChidxW<'_, SubscribeClrSpec> {
        ChidxW::new(self, 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, SubscribeClrSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "Description collection: Subscribe configuration for task CLR\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubscribeClrSpec;
impl crate::RegisterSpec for SubscribeClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subscribe_clr::R`](R) reader structure"]
impl crate::Readable for SubscribeClrSpec {}
#[doc = "`write(|w| ..)` method takes [`subscribe_clr::W`](W) writer structure"]
impl crate::Writable for SubscribeClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUBSCRIBE_CLR[%s] to value 0"]
impl crate::Resettable for SubscribeClrSpec {}
