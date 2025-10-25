#[doc = "Register `CHACHA_HW_FLAGS` reader"]
pub type R = crate::R<ChachaHwFlagsSpec>;
#[doc = "Field `CHACHA_EXISTS` reader - If this flag is set, the engine include ChaCha support"]
pub type ChachaExistsR = crate::BitReader;
#[doc = "Field `SALSA_EXISTS` reader - If this flag is set, the engine include Salsa support"]
pub type SalsaExistsR = crate::BitReader;
#[doc = "Field `FAST_CHACHA` reader - If this flag is set, the next matrix calculated when the current one is written to data output path."]
pub type FastChachaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If this flag is set, the engine include ChaCha support"]
    #[inline(always)]
    pub fn chacha_exists(&self) -> ChachaExistsR {
        ChachaExistsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this flag is set, the engine include Salsa support"]
    #[inline(always)]
    pub fn salsa_exists(&self) -> SalsaExistsR {
        SalsaExistsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this flag is set, the next matrix calculated when the current one is written to data output path."]
    #[inline(always)]
    pub fn fast_chacha(&self) -> FastChachaR {
        FastChachaR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Hardware configuration of the CHACHA engine. Reset value holds the supported features.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_hw_flags::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaHwFlagsSpec;
impl crate::RegisterSpec for ChachaHwFlagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chacha_hw_flags::R`](R) reader structure"]
impl crate::Readable for ChachaHwFlagsSpec {}
#[doc = "`reset()` method sets CHACHA_HW_FLAGS to value 0x01"]
impl crate::Resettable for ChachaHwFlagsSpec {
    const RESET_VALUE: u32 = 0x01;
}
