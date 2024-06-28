#[doc = "Register `HFXOCNT` reader"]
pub type R = crate::R<HfxocntSpec>;
#[doc = "Register `HFXOCNT` writer"]
pub type W = crate::W<HfxocntSpec>;
#[doc = "HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfxocnt {
    #[doc = "0: Min debounce time = (0*64 us + 0.5 us)"]
    MinDebounceTime = 0,
    #[doc = "255: Max debounce time = (255*64 us + 0.5 us)"]
    MaxDebounceTime = 255,
}
impl From<Hfxocnt> for u8 {
    #[inline(always)]
    fn from(variant: Hfxocnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfxocnt {
    type Ux = u8;
}
impl crate::IsEnum for Hfxocnt {}
#[doc = "Field `HFXOCNT` reader - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
pub type HfxocntR = crate::FieldReader<Hfxocnt>;
impl HfxocntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hfxocnt> {
        match self.bits {
            0 => Some(Hfxocnt::MinDebounceTime),
            255 => Some(Hfxocnt::MaxDebounceTime),
            _ => None,
        }
    }
    #[doc = "Min debounce time = (0*64 us + 0.5 us)"]
    #[inline(always)]
    pub fn is_min_debounce_time(&self) -> bool {
        *self == Hfxocnt::MinDebounceTime
    }
    #[doc = "Max debounce time = (255*64 us + 0.5 us)"]
    #[inline(always)]
    pub fn is_max_debounce_time(&self) -> bool {
        *self == Hfxocnt::MaxDebounceTime
    }
}
#[doc = "Field `HFXOCNT` writer - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
pub type HfxocntW<'a, REG> = crate::FieldWriter<'a, REG, 8, Hfxocnt>;
impl<'a, REG> HfxocntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Min debounce time = (0*64 us + 0.5 us)"]
    #[inline(always)]
    pub fn min_debounce_time(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxocnt::MinDebounceTime)
    }
    #[doc = "Max debounce time = (255*64 us + 0.5 us)"]
    #[inline(always)]
    pub fn max_debounce_time(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxocnt::MaxDebounceTime)
    }
}
impl R {
    #[doc = "Bits 0:7 - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
    #[inline(always)]
    pub fn hfxocnt(&self) -> HfxocntR {
        HfxocntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
    #[inline(always)]
    #[must_use]
    pub fn hfxocnt(&mut self) -> HfxocntW<HfxocntSpec> {
        HfxocntW::new(self, 0)
    }
}
#[doc = "HFXO startup counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxocnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxocnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxocntSpec;
impl crate::RegisterSpec for HfxocntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxocnt::R`](R) reader structure"]
impl crate::Readable for HfxocntSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxocnt::W`](W) writer structure"]
impl crate::Writable for HfxocntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXOCNT to value 0xffff_ffff"]
impl crate::Resettable for HfxocntSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
