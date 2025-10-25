#[doc = "Register `AES_HW_FLAGS` reader"]
pub type R = crate::R<AesHwFlagsSpec>;
#[doc = "Field `SUPPORT_256_192_KEY` reader - If this flag is set, the engine support 192 bits and 256 bits key size."]
pub type Support256_192KeyR = crate::BitReader;
#[doc = "Field `AES_LARGE_RKEK` reader - If this flag is set, the engine support AES_LARGE_RKEK."]
pub type AesLargeRkekR = crate::BitReader;
#[doc = "Field `DPA_CNTRMSR_EXIST` reader - If this flag is set, the engine support DPA countermeasures."]
pub type DpaCntrmsrExistR = crate::BitReader;
#[doc = "Field `CTR_EXIST` reader - If this flag is set, the engine support AES CTR mode."]
pub type CtrExistR = crate::BitReader;
#[doc = "Field `ONLY_ENCRYPT` reader - If this flag is set, the engine only support encrypt operations."]
pub type OnlyEncryptR = crate::BitReader;
#[doc = "Field `USE_SBOX_TABLE` reader - If this flag is set, the engine uses SBOX tables."]
pub type UseSboxTableR = crate::BitReader;
#[doc = "Field `USE_5_SBOXES` reader - If this flag is set, the engine uses 5 SBOX where each AES round takes 4 cycles."]
pub type Use5SboxesR = crate::BitReader;
#[doc = "Field `AES_SUPPORT_PREV_IV` reader - If this flag is set, the engine contains the PREV_IV register for faster AES XCBC MAC calculation."]
pub type AesSupportPrevIvR = crate::BitReader;
#[doc = "Field `AES_TUNNEL_EXIST` reader - If this flag is set, the engine support tunneling operations."]
pub type AesTunnelExistR = crate::BitReader;
#[doc = "Field `SECOND_REGS_SET_EXIST` reader - If this flag is set, the engine support a second register set for tunneling operations."]
pub type SecondRegsSetExistR = crate::BitReader;
#[doc = "Field `DFA_CNTRMSR_EXIST` reader - If this flag is set, the engine support DFA countermeasures."]
pub type DfaCntrmsrExistR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If this flag is set, the engine support 192 bits and 256 bits key size."]
    #[inline(always)]
    pub fn support_256_192_key(&self) -> Support256_192KeyR {
        Support256_192KeyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this flag is set, the engine support AES_LARGE_RKEK."]
    #[inline(always)]
    pub fn aes_large_rkek(&self) -> AesLargeRkekR {
        AesLargeRkekR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this flag is set, the engine support DPA countermeasures."]
    #[inline(always)]
    pub fn dpa_cntrmsr_exist(&self) -> DpaCntrmsrExistR {
        DpaCntrmsrExistR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this flag is set, the engine support AES CTR mode."]
    #[inline(always)]
    pub fn ctr_exist(&self) -> CtrExistR {
        CtrExistR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this flag is set, the engine only support encrypt operations."]
    #[inline(always)]
    pub fn only_encrypt(&self) -> OnlyEncryptR {
        OnlyEncryptR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this flag is set, the engine uses SBOX tables."]
    #[inline(always)]
    pub fn use_sbox_table(&self) -> UseSboxTableR {
        UseSboxTableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - If this flag is set, the engine uses 5 SBOX where each AES round takes 4 cycles."]
    #[inline(always)]
    pub fn use_5_sboxes(&self) -> Use5SboxesR {
        Use5SboxesR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If this flag is set, the engine contains the PREV_IV register for faster AES XCBC MAC calculation."]
    #[inline(always)]
    pub fn aes_support_prev_iv(&self) -> AesSupportPrevIvR {
        AesSupportPrevIvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If this flag is set, the engine support tunneling operations."]
    #[inline(always)]
    pub fn aes_tunnel_exist(&self) -> AesTunnelExistR {
        AesTunnelExistR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If this flag is set, the engine support a second register set for tunneling operations."]
    #[inline(always)]
    pub fn second_regs_set_exist(&self) -> SecondRegsSetExistR {
        SecondRegsSetExistR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If this flag is set, the engine support DFA countermeasures."]
    #[inline(always)]
    pub fn dfa_cntrmsr_exist(&self) -> DfaCntrmsrExistR {
        DfaCntrmsrExistR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Hardware configuration of the AES engine. Reset value holds the supported features.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_hw_flags::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesHwFlagsSpec;
impl crate::RegisterSpec for AesHwFlagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_hw_flags::R`](R) reader structure"]
impl crate::Readable for AesHwFlagsSpec {}
#[doc = "`reset()` method sets AES_HW_FLAGS to value 0x0108"]
impl crate::Resettable for AesHwFlagsSpec {
    const RESET_VALUE: u32 = 0x0108;
}
