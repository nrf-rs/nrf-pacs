#[doc = "Register `FREQUENCY` reader"]
pub type R = crate::R<FrequencySpec>;
#[doc = "Register `FREQUENCY` writer"]
pub type W = crate::W<FrequencySpec>;
#[doc = "TWI master clock frequency\n\nValue on reset: 67108864"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Frequency {
    #[doc = "26738688: 100 kbps"]
    K100 = 26738688,
    #[doc = "67108864: 250 kbps"]
    K250 = 67108864,
    #[doc = "104857600: 400 kbps"]
    K400 = 104857600,
}
impl From<Frequency> for u32 {
    #[inline(always)]
    fn from(variant: Frequency) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frequency {
    type Ux = u32;
}
impl crate::IsEnum for Frequency {}
#[doc = "Field `FREQUENCY` reader - TWI master clock frequency"]
pub type FrequencyR = crate::FieldReader<Frequency>;
impl FrequencyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frequency> {
        match self.bits {
            26738688 => Some(Frequency::K100),
            67108864 => Some(Frequency::K250),
            104857600 => Some(Frequency::K400),
            _ => None,
        }
    }
    #[doc = "100 kbps"]
    #[inline(always)]
    pub fn is_k100(&self) -> bool {
        *self == Frequency::K100
    }
    #[doc = "250 kbps"]
    #[inline(always)]
    pub fn is_k250(&self) -> bool {
        *self == Frequency::K250
    }
    #[doc = "400 kbps"]
    #[inline(always)]
    pub fn is_k400(&self) -> bool {
        *self == Frequency::K400
    }
}
#[doc = "Field `FREQUENCY` writer - TWI master clock frequency"]
pub type FrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 32, Frequency>;
impl<'a, REG> FrequencyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "100 kbps"]
    #[inline(always)]
    pub fn k100(self) -> &'a mut crate::W<REG> {
        self.variant(Frequency::K100)
    }
    #[doc = "250 kbps"]
    #[inline(always)]
    pub fn k250(self) -> &'a mut crate::W<REG> {
        self.variant(Frequency::K250)
    }
    #[doc = "400 kbps"]
    #[inline(always)]
    pub fn k400(self) -> &'a mut crate::W<REG> {
        self.variant(Frequency::K400)
    }
}
impl R {
    #[doc = "Bits 0:31 - TWI master clock frequency"]
    #[inline(always)]
    pub fn frequency(&self) -> FrequencyR {
        FrequencyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TWI master clock frequency"]
    #[inline(always)]
    pub fn frequency(&mut self) -> FrequencyW<'_, FrequencySpec> {
        FrequencyW::new(self, 0)
    }
}
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected.\n\nYou can [`read`](crate::Reg::read) this register and get [`frequency::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frequency::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrequencySpec;
impl crate::RegisterSpec for FrequencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frequency::R`](R) reader structure"]
impl crate::Readable for FrequencySpec {}
#[doc = "`write(|w| ..)` method takes [`frequency::W`](W) writer structure"]
impl crate::Writable for FrequencySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FREQUENCY to value 0x0400_0000"]
impl crate::Resettable for FrequencySpec {
    const RESET_VALUE: u32 = 0x0400_0000;
}
