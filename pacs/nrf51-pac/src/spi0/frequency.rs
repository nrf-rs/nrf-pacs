#[doc = "Register `FREQUENCY` reader"]
pub type R = crate::R<FrequencySpec>;
#[doc = "Register `FREQUENCY` writer"]
pub type W = crate::W<FrequencySpec>;
#[doc = "SPI data rate.\n\nValue on reset: 67108864"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Frequency {
    #[doc = "33554432: 125kbps."]
    K125 = 33554432,
    #[doc = "67108864: 250kbps."]
    K250 = 67108864,
    #[doc = "134217728: 500kbps."]
    K500 = 134217728,
    #[doc = "268435456: 1Mbps."]
    M1 = 268435456,
    #[doc = "536870912: 2Mbps."]
    M2 = 536870912,
    #[doc = "1073741824: 4Mbps."]
    M4 = 1073741824,
    #[doc = "2147483648: 8Mbps."]
    M8 = 2147483648,
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
#[doc = "Field `FREQUENCY` reader - SPI data rate."]
pub type FrequencyR = crate::FieldReader<Frequency>;
impl FrequencyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frequency> {
        match self.bits {
            33554432 => Some(Frequency::K125),
            67108864 => Some(Frequency::K250),
            134217728 => Some(Frequency::K500),
            268435456 => Some(Frequency::M1),
            536870912 => Some(Frequency::M2),
            1073741824 => Some(Frequency::M4),
            2147483648 => Some(Frequency::M8),
            _ => None,
        }
    }
    #[doc = "125kbps."]
    #[inline(always)]
    pub fn is_k125(&self) -> bool {
        *self == Frequency::K125
    }
    #[doc = "250kbps."]
    #[inline(always)]
    pub fn is_k250(&self) -> bool {
        *self == Frequency::K250
    }
    #[doc = "500kbps."]
    #[inline(always)]
    pub fn is_k500(&self) -> bool {
        *self == Frequency::K500
    }
    #[doc = "1Mbps."]
    #[inline(always)]
    pub fn is_m1(&self) -> bool {
        *self == Frequency::M1
    }
    #[doc = "2Mbps."]
    #[inline(always)]
    pub fn is_m2(&self) -> bool {
        *self == Frequency::M2
    }
    #[doc = "4Mbps."]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        *self == Frequency::M4
    }
    #[doc = "8Mbps."]
    #[inline(always)]
    pub fn is_m8(&self) -> bool {
        *self == Frequency::M8
    }
}
#[doc = "Field `FREQUENCY` writer - SPI data rate."]
pub type FrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 32, Frequency>;
impl<'a, REG> FrequencyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "125kbps."]
    #[inline(always)]
    pub fn k125(self) -> &'a mut crate::W<REG> {
        self.variant(Frequency::K125)
    }
    #[doc = "250kbps."]
    #[inline(always)]
    pub fn k250(self) -> &'a mut crate::W<REG> {
        self.variant(Frequency::K250)
    }
    #[doc = "500kbps."]
    #[inline(always)]
    pub fn k500(self) -> &'a mut crate::W<REG> {
        self.variant(Frequency::K500)
    }
    #[doc = "1Mbps."]
    #[inline(always)]
    pub fn m1(self) -> &'a mut crate::W<REG> {
        self.variant(Frequency::M1)
    }
    #[doc = "2Mbps."]
    #[inline(always)]
    pub fn m2(self) -> &'a mut crate::W<REG> {
        self.variant(Frequency::M2)
    }
    #[doc = "4Mbps."]
    #[inline(always)]
    pub fn m4(self) -> &'a mut crate::W<REG> {
        self.variant(Frequency::M4)
    }
    #[doc = "8Mbps."]
    #[inline(always)]
    pub fn m8(self) -> &'a mut crate::W<REG> {
        self.variant(Frequency::M8)
    }
}
impl R {
    #[doc = "Bits 0:31 - SPI data rate."]
    #[inline(always)]
    pub fn frequency(&self) -> FrequencyR {
        FrequencyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI data rate."]
    #[inline(always)]
    pub fn frequency(&mut self) -> FrequencyW<'_, FrequencySpec> {
        FrequencyW::new(self, 0)
    }
}
#[doc = "SPI frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`frequency::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frequency::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
