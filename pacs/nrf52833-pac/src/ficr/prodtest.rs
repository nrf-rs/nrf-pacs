#[doc = "Register `PRODTEST[%s]` reader"]
pub type R = crate::R<ProdtestSpec>;
#[doc = "Production test signature n\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Prodtest {
    #[doc = "3141677471: Production tests done"]
    Done = 3141677471,
    #[doc = "4294967295: Production tests not done"]
    NotDone = 4294967295,
}
impl From<Prodtest> for u32 {
    #[inline(always)]
    fn from(variant: Prodtest) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prodtest {
    type Ux = u32;
}
impl crate::IsEnum for Prodtest {}
#[doc = "Field `PRODTEST` reader - Production test signature n"]
pub type ProdtestR = crate::FieldReader<Prodtest>;
impl ProdtestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prodtest> {
        match self.bits {
            3141677471 => Some(Prodtest::Done),
            4294967295 => Some(Prodtest::NotDone),
            _ => None,
        }
    }
    #[doc = "Production tests done"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == Prodtest::Done
    }
    #[doc = "Production tests not done"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == Prodtest::NotDone
    }
}
impl R {
    #[doc = "Bits 0:31 - Production test signature n"]
    #[inline(always)]
    pub fn prodtest(&self) -> ProdtestR {
        ProdtestR::new(self.bits)
    }
}
#[doc = "Description collection: Production test signature n\n\nYou can [`read`](crate::Reg::read) this register and get [`prodtest::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProdtestSpec;
impl crate::RegisterSpec for ProdtestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prodtest::R`](R) reader structure"]
impl crate::Readable for ProdtestSpec {}
#[doc = "`reset()` method sets PRODTEST[%s] to value 0xffff_ffff"]
impl crate::Resettable for ProdtestSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
