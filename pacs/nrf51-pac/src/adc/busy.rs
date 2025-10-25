#[doc = "Register `BUSY` reader"]
pub type R = crate::R<BusySpec>;
#[doc = "ADC busy register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: No ongoing ADC conversion is taking place. ADC is ready."]
    Ready = 0,
    #[doc = "1: An ADC conversion is taking place. ADC is busy."]
    Busy = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - ADC busy register."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Ready,
            true => Busy::Busy,
        }
    }
    #[doc = "No ongoing ADC conversion is taking place. ADC is ready."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Busy::Ready
    }
    #[doc = "An ADC conversion is taking place. ADC is busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
}
impl R {
    #[doc = "Bit 0 - ADC busy register."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "ADC busy register.\n\nYou can [`read`](crate::Reg::read) this register and get [`busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusySpec;
impl crate::RegisterSpec for BusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busy::R`](R) reader structure"]
impl crate::Readable for BusySpec {}
#[doc = "`reset()` method sets BUSY to value 0"]
impl crate::Resettable for BusySpec {}
