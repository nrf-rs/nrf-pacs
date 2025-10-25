#[doc = "Register `CLK_STATUS` reader"]
pub type R = crate::R<ClkStatusSpec>;
#[doc = "Status of AES engine clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AesClk {
    #[doc = "0: Clock for AES engine is disabled"]
    Disabled = 0,
    #[doc = "1: Clock for AES engine is enabled"]
    Enabled = 1,
}
impl From<AesClk> for bool {
    #[inline(always)]
    fn from(variant: AesClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AES_CLK` reader - Status of AES engine clock."]
pub type AesClkR = crate::BitReader<AesClk>;
impl AesClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AesClk {
        match self.bits {
            false => AesClk::Disabled,
            true => AesClk::Enabled,
        }
    }
    #[doc = "Clock for AES engine is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AesClk::Disabled
    }
    #[doc = "Clock for AES engine is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AesClk::Enabled
    }
}
#[doc = "Status of HASH engine clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashClk {
    #[doc = "0: Clock for HASH engine is disabled"]
    Disabled = 0,
    #[doc = "1: Clock for HASH engine is enabled"]
    Enabled = 1,
}
impl From<HashClk> for bool {
    #[inline(always)]
    fn from(variant: HashClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASH_CLK` reader - Status of HASH engine clock."]
pub type HashClkR = crate::BitReader<HashClk>;
impl HashClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HashClk {
        match self.bits {
            false => HashClk::Disabled,
            true => HashClk::Enabled,
        }
    }
    #[doc = "Clock for HASH engine is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HashClk::Disabled
    }
    #[doc = "Clock for HASH engine is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HashClk::Enabled
    }
}
#[doc = "Status of PKA engine clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PkaClk {
    #[doc = "0: Clock for PKA engine is disabled"]
    Disabled = 0,
    #[doc = "1: Clock for PKA engine is enabled"]
    Enabled = 1,
}
impl From<PkaClk> for bool {
    #[inline(always)]
    fn from(variant: PkaClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKA_CLK` reader - Status of PKA engine clock."]
pub type PkaClkR = crate::BitReader<PkaClk>;
impl PkaClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PkaClk {
        match self.bits {
            false => PkaClk::Disabled,
            true => PkaClk::Enabled,
        }
    }
    #[doc = "Clock for PKA engine is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PkaClk::Disabled
    }
    #[doc = "Clock for PKA engine is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PkaClk::Enabled
    }
}
#[doc = "Status of CHACHA engine clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChachaClk {
    #[doc = "0: Clock for CHACHA engine is disabled"]
    Disabled = 0,
    #[doc = "1: Clock for CHACHA engine is enabled"]
    Enabled = 1,
}
impl From<ChachaClk> for bool {
    #[inline(always)]
    fn from(variant: ChachaClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHA_CLK` reader - Status of CHACHA engine clock."]
pub type ChachaClkR = crate::BitReader<ChachaClk>;
impl ChachaClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChachaClk {
        match self.bits {
            false => ChachaClk::Disabled,
            true => ChachaClk::Enabled,
        }
    }
    #[doc = "Clock for CHACHA engine is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ChachaClk::Disabled
    }
    #[doc = "Clock for CHACHA engine is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ChachaClk::Enabled
    }
}
#[doc = "Status of DMA engines clock.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaClk {
    #[doc = "0: Clocks for DMA engines are disabled"]
    Disabled = 0,
    #[doc = "1: Clocks for DMA engines are enabled"]
    Enabled = 1,
}
impl From<DmaClk> for bool {
    #[inline(always)]
    fn from(variant: DmaClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_CLK` reader - Status of DMA engines clock."]
pub type DmaClkR = crate::BitReader<DmaClk>;
impl DmaClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaClk {
        match self.bits {
            false => DmaClk::Disabled,
            true => DmaClk::Enabled,
        }
    }
    #[doc = "Clocks for DMA engines are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaClk::Disabled
    }
    #[doc = "Clocks for DMA engines are enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaClk::Enabled
    }
}
impl R {
    #[doc = "Bit 0 - Status of AES engine clock."]
    #[inline(always)]
    pub fn aes_clk(&self) -> AesClkR {
        AesClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Status of HASH engine clock."]
    #[inline(always)]
    pub fn hash_clk(&self) -> HashClkR {
        HashClkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of PKA engine clock."]
    #[inline(always)]
    pub fn pka_clk(&self) -> PkaClkR {
        PkaClkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of CHACHA engine clock."]
    #[inline(always)]
    pub fn chacha_clk(&self) -> ChachaClkR {
        ChachaClkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of DMA engines clock."]
    #[inline(always)]
    pub fn dma_clk(&self) -> DmaClkR {
        DmaClkR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "CRYPTOCELL clocks status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkStatusSpec;
impl crate::RegisterSpec for ClkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_status::R`](R) reader structure"]
impl crate::Readable for ClkStatusSpec {}
#[doc = "`reset()` method sets CLK_STATUS to value 0x0100"]
impl crate::Resettable for ClkStatusSpec {
    const RESET_VALUE: u32 = 0x0100;
}
