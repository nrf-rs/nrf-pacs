#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkstarted {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Hfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: Hfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKSTARTED` reader - Read pending status of interrupt for event HFCLKSTARTED"]
pub type HfclkstartedR = crate::BitReader<Hfclkstarted>;
impl HfclkstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclkstarted {
        match self.bits {
            false => Hfclkstarted::NotPending,
            true => Hfclkstarted::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Hfclkstarted::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Hfclkstarted::Pending
    }
}
#[doc = "Read pending status of interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfclkstarted {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Lfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: Lfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` reader - Read pending status of interrupt for event LFCLKSTARTED"]
pub type LfclkstartedR = crate::BitReader<Lfclkstarted>;
impl LfclkstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfclkstarted {
        match self.bits {
            false => Lfclkstarted::NotPending,
            true => Lfclkstarted::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Lfclkstarted::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Lfclkstarted::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&self) -> HfclkstartedR {
        HfclkstartedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LfclkstartedR {
        LfclkstartedR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Pending interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpend::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpendSpec;
impl crate::RegisterSpec for IntpendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpend::R`](R) reader structure"]
impl crate::Readable for IntpendSpec {}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for IntpendSpec {
    const RESET_VALUE: u32 = 0;
}
