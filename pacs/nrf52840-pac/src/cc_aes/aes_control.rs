#[doc = "Register `AES_CONTROL` reader"]
pub type R = crate::R<AesControlSpec>;
#[doc = "Register `AES_CONTROL` writer"]
pub type W = crate::W<AesControlSpec>;
#[doc = "Set AES encrypt or decrypt mode in non-tunneling operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecKey0 {
    #[doc = "0: Perform AES encryption"]
    Encrypt = 0,
    #[doc = "1: Perform AES decryption"]
    Decrypt = 1,
}
impl From<DecKey0> for bool {
    #[inline(always)]
    fn from(variant: DecKey0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEC_KEY0` reader - Set AES encrypt or decrypt mode in non-tunneling operations."]
pub type DecKey0R = crate::BitReader<DecKey0>;
impl DecKey0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DecKey0 {
        match self.bits {
            false => DecKey0::Encrypt,
            true => DecKey0::Decrypt,
        }
    }
    #[doc = "Perform AES encryption"]
    #[inline(always)]
    pub fn is_encrypt(&self) -> bool {
        *self == DecKey0::Encrypt
    }
    #[doc = "Perform AES decryption"]
    #[inline(always)]
    pub fn is_decrypt(&self) -> bool {
        *self == DecKey0::Decrypt
    }
}
#[doc = "Field `DEC_KEY0` writer - Set AES encrypt or decrypt mode in non-tunneling operations."]
pub type DecKey0W<'a, REG> = crate::BitWriter<'a, REG, DecKey0>;
impl<'a, REG> DecKey0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Perform AES encryption"]
    #[inline(always)]
    pub fn encrypt(self) -> &'a mut crate::W<REG> {
        self.variant(DecKey0::Encrypt)
    }
    #[doc = "Perform AES decryption"]
    #[inline(always)]
    pub fn decrypt(self) -> &'a mut crate::W<REG> {
        self.variant(DecKey0::Decrypt)
    }
}
#[doc = "Set the AES mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ModeKey0 {
    #[doc = "0: Electronic codebook mode"]
    Ecb = 0,
    #[doc = "1: Cipher block chaining mode"]
    Cbc = 1,
    #[doc = "2: Counter mode"]
    Ctr = 2,
    #[doc = "3: Cipher Block Chaining Message Authentication Code"]
    CbcMac = 3,
    #[doc = "7: Cipher-based Message Authentication Code"]
    Cmac = 7,
}
impl From<ModeKey0> for u8 {
    #[inline(always)]
    fn from(variant: ModeKey0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ModeKey0 {
    type Ux = u8;
}
impl crate::IsEnum for ModeKey0 {}
#[doc = "Field `MODE_KEY0` reader - Set the AES mode."]
pub type ModeKey0R = crate::FieldReader<ModeKey0>;
impl ModeKey0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ModeKey0> {
        match self.bits {
            0 => Some(ModeKey0::Ecb),
            1 => Some(ModeKey0::Cbc),
            2 => Some(ModeKey0::Ctr),
            3 => Some(ModeKey0::CbcMac),
            7 => Some(ModeKey0::Cmac),
            _ => None,
        }
    }
    #[doc = "Electronic codebook mode"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == ModeKey0::Ecb
    }
    #[doc = "Cipher block chaining mode"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == ModeKey0::Cbc
    }
    #[doc = "Counter mode"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == ModeKey0::Ctr
    }
    #[doc = "Cipher Block Chaining Message Authentication Code"]
    #[inline(always)]
    pub fn is_cbc_mac(&self) -> bool {
        *self == ModeKey0::CbcMac
    }
    #[doc = "Cipher-based Message Authentication Code"]
    #[inline(always)]
    pub fn is_cmac(&self) -> bool {
        *self == ModeKey0::Cmac
    }
}
#[doc = "Field `MODE_KEY0` writer - Set the AES mode."]
pub type ModeKey0W<'a, REG> = crate::FieldWriter<'a, REG, 3, ModeKey0>;
impl<'a, REG> ModeKey0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Electronic codebook mode"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(ModeKey0::Ecb)
    }
    #[doc = "Cipher block chaining mode"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(ModeKey0::Cbc)
    }
    #[doc = "Counter mode"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(ModeKey0::Ctr)
    }
    #[doc = "Cipher Block Chaining Message Authentication Code"]
    #[inline(always)]
    pub fn cbc_mac(self) -> &'a mut crate::W<REG> {
        self.variant(ModeKey0::CbcMac)
    }
    #[doc = "Cipher-based Message Authentication Code"]
    #[inline(always)]
    pub fn cmac(self) -> &'a mut crate::W<REG> {
        self.variant(ModeKey0::Cmac)
    }
}
#[doc = "Set the AES key length.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NkKey0 {
    #[doc = "0: 128 bits key length"]
    _128bits = 0,
}
impl From<NkKey0> for u8 {
    #[inline(always)]
    fn from(variant: NkKey0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NkKey0 {
    type Ux = u8;
}
impl crate::IsEnum for NkKey0 {}
#[doc = "Field `NK_KEY0` reader - Set the AES key length."]
pub type NkKey0R = crate::FieldReader<NkKey0>;
impl NkKey0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NkKey0> {
        match self.bits {
            0 => Some(NkKey0::_128bits),
            _ => None,
        }
    }
    #[doc = "128 bits key length"]
    #[inline(always)]
    pub fn is_128bits(&self) -> bool {
        *self == NkKey0::_128bits
    }
}
#[doc = "Field `NK_KEY0` writer - Set the AES key length."]
pub type NkKey0W<'a, REG> = crate::FieldWriter<'a, REG, 2, NkKey0>;
impl<'a, REG> NkKey0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 bits key length"]
    #[inline(always)]
    pub fn _128bits(self) -> &'a mut crate::W<REG> {
        self.variant(NkKey0::_128bits)
    }
}
#[doc = "This field determines the value that is written to AES_KEY_0, when AES_SK is kicked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AesXorCryptokey {
    #[doc = "0: The value that is written to AES_KEY_0 is the value of the HW cryptokey as is."]
    Disable = 0,
    #[doc = "1: The value that is written to AES_KEY_0 is the value of the HW cryptokey XOR with the current value of AES_KEY_0."]
    Enable = 1,
}
impl From<AesXorCryptokey> for bool {
    #[inline(always)]
    fn from(variant: AesXorCryptokey) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AES_XOR_CRYPTOKEY` reader - This field determines the value that is written to AES_KEY_0, when AES_SK is kicked."]
pub type AesXorCryptokeyR = crate::BitReader<AesXorCryptokey>;
impl AesXorCryptokeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AesXorCryptokey {
        match self.bits {
            false => AesXorCryptokey::Disable,
            true => AesXorCryptokey::Enable,
        }
    }
    #[doc = "The value that is written to AES_KEY_0 is the value of the HW cryptokey as is."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AesXorCryptokey::Disable
    }
    #[doc = "The value that is written to AES_KEY_0 is the value of the HW cryptokey XOR with the current value of AES_KEY_0."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AesXorCryptokey::Enable
    }
}
#[doc = "Field `AES_XOR_CRYPTOKEY` writer - This field determines the value that is written to AES_KEY_0, when AES_SK is kicked."]
pub type AesXorCryptokeyW<'a, REG> = crate::BitWriter<'a, REG, AesXorCryptokey>;
impl<'a, REG> AesXorCryptokeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The value that is written to AES_KEY_0 is the value of the HW cryptokey as is."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AesXorCryptokey::Disable)
    }
    #[doc = "The value that is written to AES_KEY_0 is the value of the HW cryptokey XOR with the current value of AES_KEY_0."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AesXorCryptokey::Enable)
    }
}
#[doc = "Using direct access and not the DIN-DOUT DMA interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectAccess {
    #[doc = "0: Access using the DIN-DOUT DMA interface"]
    Disable = 0,
    #[doc = "1: Access using direct access"]
    Enable = 1,
}
impl From<DirectAccess> for bool {
    #[inline(always)]
    fn from(variant: DirectAccess) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRECT_ACCESS` reader - Using direct access and not the DIN-DOUT DMA interface"]
pub type DirectAccessR = crate::BitReader<DirectAccess>;
impl DirectAccessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectAccess {
        match self.bits {
            false => DirectAccess::Disable,
            true => DirectAccess::Enable,
        }
    }
    #[doc = "Access using the DIN-DOUT DMA interface"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DirectAccess::Disable
    }
    #[doc = "Access using direct access"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DirectAccess::Enable
    }
}
#[doc = "Field `DIRECT_ACCESS` writer - Using direct access and not the DIN-DOUT DMA interface"]
pub type DirectAccessW<'a, REG> = crate::BitWriter<'a, REG, DirectAccess>;
impl<'a, REG> DirectAccessW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access using the DIN-DOUT DMA interface"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DirectAccess::Disable)
    }
    #[doc = "Access using direct access"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DirectAccess::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Set AES encrypt or decrypt mode in non-tunneling operations."]
    #[inline(always)]
    pub fn dec_key0(&self) -> DecKey0R {
        DecKey0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Set the AES mode."]
    #[inline(always)]
    pub fn mode_key0(&self) -> ModeKey0R {
        ModeKey0R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Set the AES key length."]
    #[inline(always)]
    pub fn nk_key0(&self) -> NkKey0R {
        NkKey0R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 29 - This field determines the value that is written to AES_KEY_0, when AES_SK is kicked."]
    #[inline(always)]
    pub fn aes_xor_cryptokey(&self) -> AesXorCryptokeyR {
        AesXorCryptokeyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Using direct access and not the DIN-DOUT DMA interface"]
    #[inline(always)]
    pub fn direct_access(&self) -> DirectAccessR {
        DirectAccessR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set AES encrypt or decrypt mode in non-tunneling operations."]
    #[inline(always)]
    pub fn dec_key0(&mut self) -> DecKey0W<'_, AesControlSpec> {
        DecKey0W::new(self, 0)
    }
    #[doc = "Bits 2:4 - Set the AES mode."]
    #[inline(always)]
    pub fn mode_key0(&mut self) -> ModeKey0W<'_, AesControlSpec> {
        ModeKey0W::new(self, 2)
    }
    #[doc = "Bits 12:13 - Set the AES key length."]
    #[inline(always)]
    pub fn nk_key0(&mut self) -> NkKey0W<'_, AesControlSpec> {
        NkKey0W::new(self, 12)
    }
    #[doc = "Bit 29 - This field determines the value that is written to AES_KEY_0, when AES_SK is kicked."]
    #[inline(always)]
    pub fn aes_xor_cryptokey(&mut self) -> AesXorCryptokeyW<'_, AesControlSpec> {
        AesXorCryptokeyW::new(self, 29)
    }
    #[doc = "Bit 31 - Using direct access and not the DIN-DOUT DMA interface"]
    #[inline(always)]
    pub fn direct_access(&mut self) -> DirectAccessW<'_, AesControlSpec> {
        DirectAccessW::new(self, 31)
    }
}
#[doc = "Control the AES engine behavior.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesControlSpec;
impl crate::RegisterSpec for AesControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_control::R`](R) reader structure"]
impl crate::Readable for AesControlSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_control::W`](W) writer structure"]
impl crate::Writable for AesControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_CONTROL to value 0"]
impl crate::Resettable for AesControlSpec {}
