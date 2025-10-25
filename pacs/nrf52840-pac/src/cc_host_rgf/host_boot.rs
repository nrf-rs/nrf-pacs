#[doc = "Register `HOST_BOOT` reader"]
pub type R = crate::R<HostBootSpec>;
#[doc = "Field `POWER_GATING_EXISTS_LOCAL` reader - If this flag is set, full power gating is implemented"]
pub type PowerGatingExistsLocalR = crate::BitReader;
#[doc = "Field `LARGE_RKEK_LOCAL` reader - If this flag is set, large RKEK is supported"]
pub type LargeRkekLocalR = crate::BitReader;
#[doc = "Field `HASH_IN_FUSES_LOCAL` reader - If this flag is set, HASH in fuses is supported"]
pub type HashInFusesLocalR = crate::BitReader;
#[doc = "Field `EXT_MEM_SECURED_LOCAL` reader - If this flag is set, external secure memory is supported"]
pub type ExtMemSecuredLocalR = crate::BitReader;
#[doc = "Field `RKEK_ECC_EXISTS_LOCAL_N` reader - If this flag is set, RKEK ECC is supported"]
pub type RkekEccExistsLocalNR = crate::BitReader;
#[doc = "Field `SRAM_SIZE_LOCAL` reader - SRAM size"]
pub type SramSizeLocalR = crate::FieldReader;
#[doc = "Field `DSCRPTR_EXISTS_LOCAL` reader - If this flag is set, Descriptors are supported"]
pub type DscrptrExistsLocalR = crate::BitReader;
#[doc = "Field `PAU_EXISTS_LOCAL` reader - If this flag is set, PAU is supported"]
pub type PauExistsLocalR = crate::BitReader;
#[doc = "Field `RNG_EXISTS_LOCAL` reader - If this flag is set, the RNG engine is present"]
pub type RngExistsLocalR = crate::BitReader;
#[doc = "Field `PKA_EXISTS_LOCAL` reader - If this flag is set, the PKA engine is present"]
pub type PkaExistsLocalR = crate::BitReader;
#[doc = "Field `RC4_EXISTS_LOCAL` reader - If this flag is set, the RC4 engine is present"]
pub type Rc4ExistsLocalR = crate::BitReader;
#[doc = "Field `SHA_512_PRSNT_LOCAL` reader - If this flag is set, the HASH engine supports SHA512"]
pub type Sha512PrsntLocalR = crate::BitReader;
#[doc = "Field `SHA_256_PRSNT_LOCAL` reader - If this flag is set, the HASH engine supports SHA256"]
pub type Sha256PrsntLocalR = crate::BitReader;
#[doc = "Field `MD5_PRSNT_LOCAL` reader - If this flag is set, the HASH engine supports MD5"]
pub type Md5PrsntLocalR = crate::BitReader;
#[doc = "Field `HASH_EXISTS_LOCAL` reader - If this flag is set, the HASH engine is present"]
pub type HashExistsLocalR = crate::BitReader;
#[doc = "Field `C2_EXISTS_LOCAL` reader - If this flag is set, the C2 engine is present"]
pub type C2ExistsLocalR = crate::BitReader;
#[doc = "Field `DES_EXISTS_LOCAL` reader - If this flag is set, the DES engine is present"]
pub type DesExistsLocalR = crate::BitReader;
#[doc = "Field `AES_XCBC_MAC_EXISTS_LOCAL` reader - If this flag is set, AES XCBC-MAC mode is supported"]
pub type AesXcbcMacExistsLocalR = crate::BitReader;
#[doc = "Field `AES_CMAC_EXISTS_LOCAL` reader - If this flag is set, AES CMAC mode is supported"]
pub type AesCmacExistsLocalR = crate::BitReader;
#[doc = "Field `AES_CCM_EXISTS_LOCAL` reader - If this flag is set, AES CCM mode is supported"]
pub type AesCcmExistsLocalR = crate::BitReader;
#[doc = "Field `AES_XEX_HW_T_CALC_LOCAL` reader - If this flag is set, AES XEX mode T-value calculation in HW is supported"]
pub type AesXexHwTCalcLocalR = crate::BitReader;
#[doc = "Field `AES_XEX_EXISTS_LOCAL` reader - If this flag is set, AES XEX mode is supported"]
pub type AesXexExistsLocalR = crate::BitReader;
#[doc = "Field `CTR_EXISTS_LOCAL` reader - If this flag is set, AES CTR mode is supported"]
pub type CtrExistsLocalR = crate::BitReader;
#[doc = "Field `AES_DIN_BYTE_RESOLUTION_LOCAL` reader - If this flag is set, the AES engine data input support byte size resolution"]
pub type AesDinByteResolutionLocalR = crate::BitReader;
#[doc = "Field `TUNNELING_ENB_LOCAL` reader - If this flag is set, the AES engine supports tunneling operations"]
pub type TunnelingEnbLocalR = crate::BitReader;
#[doc = "Field `SUPPORT_256_192_KEY_LOCAL` reader - If this flag is set, the AES engine supports 192/256 bits key sizes"]
pub type Support256_192KeyLocalR = crate::BitReader;
#[doc = "Field `ONLY_ENCRYPT_LOCAL` reader - If this flag is set, the AES engine only support encryption"]
pub type OnlyEncryptLocalR = crate::BitReader;
#[doc = "Field `AES_EXISTS_LOCAL` reader - If this flag is set, the AES engine is present"]
pub type AesExistsLocalR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If this flag is set, full power gating is implemented"]
    #[inline(always)]
    pub fn power_gating_exists_local(&self) -> PowerGatingExistsLocalR {
        PowerGatingExistsLocalR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this flag is set, large RKEK is supported"]
    #[inline(always)]
    pub fn large_rkek_local(&self) -> LargeRkekLocalR {
        LargeRkekLocalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this flag is set, HASH in fuses is supported"]
    #[inline(always)]
    pub fn hash_in_fuses_local(&self) -> HashInFusesLocalR {
        HashInFusesLocalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this flag is set, external secure memory is supported"]
    #[inline(always)]
    pub fn ext_mem_secured_local(&self) -> ExtMemSecuredLocalR {
        ExtMemSecuredLocalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - If this flag is set, RKEK ECC is supported"]
    #[inline(always)]
    pub fn rkek_ecc_exists_local_n(&self) -> RkekEccExistsLocalNR {
        RkekEccExistsLocalNR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - SRAM size"]
    #[inline(always)]
    pub fn sram_size_local(&self) -> SramSizeLocalR {
        SramSizeLocalR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - If this flag is set, Descriptors are supported"]
    #[inline(always)]
    pub fn dscrptr_exists_local(&self) -> DscrptrExistsLocalR {
        DscrptrExistsLocalR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If this flag is set, PAU is supported"]
    #[inline(always)]
    pub fn pau_exists_local(&self) -> PauExistsLocalR {
        PauExistsLocalR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If this flag is set, the RNG engine is present"]
    #[inline(always)]
    pub fn rng_exists_local(&self) -> RngExistsLocalR {
        RngExistsLocalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If this flag is set, the PKA engine is present"]
    #[inline(always)]
    pub fn pka_exists_local(&self) -> PkaExistsLocalR {
        PkaExistsLocalR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If this flag is set, the RC4 engine is present"]
    #[inline(always)]
    pub fn rc4_exists_local(&self) -> Rc4ExistsLocalR {
        Rc4ExistsLocalR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If this flag is set, the HASH engine supports SHA512"]
    #[inline(always)]
    pub fn sha_512_prsnt_local(&self) -> Sha512PrsntLocalR {
        Sha512PrsntLocalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If this flag is set, the HASH engine supports SHA256"]
    #[inline(always)]
    pub fn sha_256_prsnt_local(&self) -> Sha256PrsntLocalR {
        Sha256PrsntLocalR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - If this flag is set, the HASH engine supports MD5"]
    #[inline(always)]
    pub fn md5_prsnt_local(&self) -> Md5PrsntLocalR {
        Md5PrsntLocalR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If this flag is set, the HASH engine is present"]
    #[inline(always)]
    pub fn hash_exists_local(&self) -> HashExistsLocalR {
        HashExistsLocalR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If this flag is set, the C2 engine is present"]
    #[inline(always)]
    pub fn c2_exists_local(&self) -> C2ExistsLocalR {
        C2ExistsLocalR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - If this flag is set, the DES engine is present"]
    #[inline(always)]
    pub fn des_exists_local(&self) -> DesExistsLocalR {
        DesExistsLocalR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If this flag is set, AES XCBC-MAC mode is supported"]
    #[inline(always)]
    pub fn aes_xcbc_mac_exists_local(&self) -> AesXcbcMacExistsLocalR {
        AesXcbcMacExistsLocalR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - If this flag is set, AES CMAC mode is supported"]
    #[inline(always)]
    pub fn aes_cmac_exists_local(&self) -> AesCmacExistsLocalR {
        AesCmacExistsLocalR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - If this flag is set, AES CCM mode is supported"]
    #[inline(always)]
    pub fn aes_ccm_exists_local(&self) -> AesCcmExistsLocalR {
        AesCcmExistsLocalR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - If this flag is set, AES XEX mode T-value calculation in HW is supported"]
    #[inline(always)]
    pub fn aes_xex_hw_t_calc_local(&self) -> AesXexHwTCalcLocalR {
        AesXexHwTCalcLocalR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - If this flag is set, AES XEX mode is supported"]
    #[inline(always)]
    pub fn aes_xex_exists_local(&self) -> AesXexExistsLocalR {
        AesXexExistsLocalR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If this flag is set, AES CTR mode is supported"]
    #[inline(always)]
    pub fn ctr_exists_local(&self) -> CtrExistsLocalR {
        CtrExistsLocalR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If this flag is set, the AES engine data input support byte size resolution"]
    #[inline(always)]
    pub fn aes_din_byte_resolution_local(&self) -> AesDinByteResolutionLocalR {
        AesDinByteResolutionLocalR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - If this flag is set, the AES engine supports tunneling operations"]
    #[inline(always)]
    pub fn tunneling_enb_local(&self) -> TunnelingEnbLocalR {
        TunnelingEnbLocalR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - If this flag is set, the AES engine supports 192/256 bits key sizes"]
    #[inline(always)]
    pub fn support_256_192_key_local(&self) -> Support256_192KeyLocalR {
        Support256_192KeyLocalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - If this flag is set, the AES engine only support encryption"]
    #[inline(always)]
    pub fn only_encrypt_local(&self) -> OnlyEncryptLocalR {
        OnlyEncryptLocalR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If this flag is set, the AES engine is present"]
    #[inline(always)]
    pub fn aes_exists_local(&self) -> AesExistsLocalR {
        AesExistsLocalR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "Hardware configuration of the CRYPTOCELL subsystem. Reset value holds the supported features.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_boot::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostBootSpec;
impl crate::RegisterSpec for HostBootSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_boot::R`](R) reader structure"]
impl crate::Readable for HostBootSpec {}
#[doc = "`reset()` method sets HOST_BOOT to value 0x4622_982c"]
impl crate::Resettable for HostBootSpec {
    const RESET_VALUE: u32 = 0x4622_982c;
}
