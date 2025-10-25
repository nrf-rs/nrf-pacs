#[doc = "Register `LIMITH` reader"]
pub type R = crate::R<LimithSpec>;
#[doc = "Register `LIMITH` writer"]
pub type W = crate::W<LimithSpec>;
#[doc = "Field `CHIDX` reader - DPPI channel that event CH\\[n\\].LIMITH will publish to"]
pub type ChidxR = crate::FieldReader;
#[doc = "Field `CHIDX` writer - DPPI channel that event CH\\[n\\].LIMITH will publish to"]
pub type ChidxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable publishing"]
    Disabled = 0,
    #[doc = "1: Enable publishing"]
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
    #[doc = "Disable publishing"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En::Disabled
    }
    #[doc = "Enable publishing"]
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
    #[doc = "Disable publishing"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disabled)
    }
    #[doc = "Enable publishing"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - DPPI channel that event CH\\[n\\].LIMITH will publish to"]
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
    #[doc = "Bits 0:7 - DPPI channel that event CH\\[n\\].LIMITH will publish to"]
    #[inline(always)]
    pub fn chidx(&mut self) -> ChidxW<'_, LimithSpec> {
        ChidxW::new(self, 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, LimithSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITH\n\nYou can [`read`](crate::Reg::read) this register and get [`limith::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limith::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimithSpec;
impl crate::RegisterSpec for LimithSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limith::R`](R) reader structure"]
impl crate::Readable for LimithSpec {}
#[doc = "`write(|w| ..)` method takes [`limith::W`](W) writer structure"]
impl crate::Writable for LimithSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LIMITH to value 0"]
impl crate::Resettable for LimithSpec {}
