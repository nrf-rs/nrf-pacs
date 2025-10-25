#[doc = "Register `POFCON` reader"]
pub type R = crate::R<PofconSpec>;
#[doc = "Register `POFCON` writer"]
pub type W = crate::W<PofconSpec>;
#[doc = "Power failure comparator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pof {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Enabled."]
    Enabled = 1,
}
impl From<Pof> for bool {
    #[inline(always)]
    fn from(variant: Pof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POF` reader - Power failure comparator enable."]
pub type PofR = crate::BitReader<Pof>;
impl PofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pof {
        match self.bits {
            false => Pof::Disabled,
            true => Pof::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pof::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pof::Enabled
    }
}
#[doc = "Field `POF` writer - Power failure comparator enable."]
pub type PofW<'a, REG> = crate::BitWriter<'a, REG, Pof>;
impl<'a, REG> PofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pof::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pof::Enabled)
    }
}
#[doc = "Set threshold level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Threshold {
    #[doc = "0: Set threshold to 2.1Volts."]
    V21 = 0,
    #[doc = "1: Set threshold to 2.3Volts."]
    V23 = 1,
    #[doc = "2: Set threshold to 2.5Volts."]
    V25 = 2,
    #[doc = "3: Set threshold to 2.7Volts."]
    V27 = 3,
}
impl From<Threshold> for u8 {
    #[inline(always)]
    fn from(variant: Threshold) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Threshold {
    type Ux = u8;
}
impl crate::IsEnum for Threshold {}
#[doc = "Field `THRESHOLD` reader - Set threshold level."]
pub type ThresholdR = crate::FieldReader<Threshold>;
impl ThresholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Threshold {
        match self.bits {
            0 => Threshold::V21,
            1 => Threshold::V23,
            2 => Threshold::V25,
            3 => Threshold::V27,
            _ => unreachable!(),
        }
    }
    #[doc = "Set threshold to 2.1Volts."]
    #[inline(always)]
    pub fn is_v21(&self) -> bool {
        *self == Threshold::V21
    }
    #[doc = "Set threshold to 2.3Volts."]
    #[inline(always)]
    pub fn is_v23(&self) -> bool {
        *self == Threshold::V23
    }
    #[doc = "Set threshold to 2.5Volts."]
    #[inline(always)]
    pub fn is_v25(&self) -> bool {
        *self == Threshold::V25
    }
    #[doc = "Set threshold to 2.7Volts."]
    #[inline(always)]
    pub fn is_v27(&self) -> bool {
        *self == Threshold::V27
    }
}
#[doc = "Field `THRESHOLD` writer - Set threshold level."]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Threshold, crate::Safe>;
impl<'a, REG> ThresholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set threshold to 2.1Volts."]
    #[inline(always)]
    pub fn v21(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V21)
    }
    #[doc = "Set threshold to 2.3Volts."]
    #[inline(always)]
    pub fn v23(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V23)
    }
    #[doc = "Set threshold to 2.5Volts."]
    #[inline(always)]
    pub fn v25(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V25)
    }
    #[doc = "Set threshold to 2.7Volts."]
    #[inline(always)]
    pub fn v27(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V27)
    }
}
impl R {
    #[doc = "Bit 0 - Power failure comparator enable."]
    #[inline(always)]
    pub fn pof(&self) -> PofR {
        PofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Set threshold level."]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power failure comparator enable."]
    #[inline(always)]
    pub fn pof(&mut self) -> PofW<'_, PofconSpec> {
        PofW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Set threshold level."]
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<'_, PofconSpec> {
        ThresholdW::new(self, 1)
    }
}
#[doc = "Power failure configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`pofcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pofcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PofconSpec;
impl crate::RegisterSpec for PofconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pofcon::R`](R) reader structure"]
impl crate::Readable for PofconSpec {}
#[doc = "`write(|w| ..)` method takes [`pofcon::W`](W) writer structure"]
impl crate::Writable for PofconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POFCON to value 0"]
impl crate::Resettable for PofconSpec {}
