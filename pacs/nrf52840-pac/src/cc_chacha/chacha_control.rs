#[doc = "Register `CHACHA_CONTROL` reader"]
pub type R = crate::R<ChachaControlSpec>;
#[doc = "Register `CHACHA_CONTROL` writer"]
pub type W = crate::W<ChachaControlSpec>;
#[doc = "Run engine in ChaCha or Salsa mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChachaOrSalsa {
    #[doc = "0: Run engine in ChaCha mode"]
    ChaCha = 0,
    #[doc = "1: Run engine in Salsa mode"]
    Salsa = 1,
}
impl From<ChachaOrSalsa> for bool {
    #[inline(always)]
    fn from(variant: ChachaOrSalsa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHA_OR_SALSA` reader - Run engine in ChaCha or Salsa mode"]
pub type ChachaOrSalsaR = crate::BitReader<ChachaOrSalsa>;
impl ChachaOrSalsaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChachaOrSalsa {
        match self.bits {
            false => ChachaOrSalsa::ChaCha,
            true => ChachaOrSalsa::Salsa,
        }
    }
    #[doc = "Run engine in ChaCha mode"]
    #[inline(always)]
    pub fn is_cha_cha(&self) -> bool {
        *self == ChachaOrSalsa::ChaCha
    }
    #[doc = "Run engine in Salsa mode"]
    #[inline(always)]
    pub fn is_salsa(&self) -> bool {
        *self == ChachaOrSalsa::Salsa
    }
}
#[doc = "Field `CHACHA_OR_SALSA` writer - Run engine in ChaCha or Salsa mode"]
pub type ChachaOrSalsaW<'a, REG> = crate::BitWriter<'a, REG, ChachaOrSalsa>;
impl<'a, REG> ChachaOrSalsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Run engine in ChaCha mode"]
    #[inline(always)]
    pub fn cha_cha(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaOrSalsa::ChaCha)
    }
    #[doc = "Run engine in Salsa mode"]
    #[inline(always)]
    pub fn salsa(self) -> &'a mut crate::W<REG> {
        self.variant(ChachaOrSalsa::Salsa)
    }
}
#[doc = "Perform initialization for a new message\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Init {
    #[doc = "0: Message already initialized"]
    Disable = 0,
    #[doc = "1: Initialize new message"]
    Enable = 1,
}
impl From<Init> for bool {
    #[inline(always)]
    fn from(variant: Init) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Perform initialization for a new message"]
pub type InitR = crate::BitReader<Init>;
impl InitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Init {
        match self.bits {
            false => Init::Disable,
            true => Init::Enable,
        }
    }
    #[doc = "Message already initialized"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Init::Disable
    }
    #[doc = "Initialize new message"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Init::Enable
    }
}
#[doc = "Field `INIT` writer - Perform initialization for a new message"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG, Init>;
impl<'a, REG> InitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Message already initialized"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Init::Disable)
    }
    #[doc = "Initialize new message"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Init::Enable)
    }
}
#[doc = "Generate the key to use in Poly1305 message authentication code calculation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenKeyPoly1305 {
    #[doc = "0: Do not generate Poly1305 key"]
    Disable = 0,
    #[doc = "1: Generate Poly1305 key"]
    Enable = 1,
}
impl From<GenKeyPoly1305> for bool {
    #[inline(always)]
    fn from(variant: GenKeyPoly1305) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GEN_KEY_POLY1305` reader - Generate the key to use in Poly1305 message authentication code calculation."]
pub type GenKeyPoly1305R = crate::BitReader<GenKeyPoly1305>;
impl GenKeyPoly1305R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GenKeyPoly1305 {
        match self.bits {
            false => GenKeyPoly1305::Disable,
            true => GenKeyPoly1305::Enable,
        }
    }
    #[doc = "Do not generate Poly1305 key"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GenKeyPoly1305::Disable
    }
    #[doc = "Generate Poly1305 key"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GenKeyPoly1305::Enable
    }
}
#[doc = "Field `GEN_KEY_POLY1305` writer - Generate the key to use in Poly1305 message authentication code calculation."]
pub type GenKeyPoly1305W<'a, REG> = crate::BitWriter<'a, REG, GenKeyPoly1305>;
impl<'a, REG> GenKeyPoly1305W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not generate Poly1305 key"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GenKeyPoly1305::Disable)
    }
    #[doc = "Generate Poly1305 key"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GenKeyPoly1305::Enable)
    }
}
#[doc = "Key length selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyLen {
    #[doc = "0: Use 256 bits key length"]
    _256bits = 0,
    #[doc = "1: Use 128 bits key length"]
    _128bits = 1,
}
impl From<KeyLen> for bool {
    #[inline(always)]
    fn from(variant: KeyLen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEY_LEN` reader - Key length selection."]
pub type KeyLenR = crate::BitReader<KeyLen>;
impl KeyLenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KeyLen {
        match self.bits {
            false => KeyLen::_256bits,
            true => KeyLen::_128bits,
        }
    }
    #[doc = "Use 256 bits key length"]
    #[inline(always)]
    pub fn is_256bits(&self) -> bool {
        *self == KeyLen::_256bits
    }
    #[doc = "Use 128 bits key length"]
    #[inline(always)]
    pub fn is_128bits(&self) -> bool {
        *self == KeyLen::_128bits
    }
}
#[doc = "Field `KEY_LEN` writer - Key length selection."]
pub type KeyLenW<'a, REG> = crate::BitWriter<'a, REG, KeyLen>;
impl<'a, REG> KeyLenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 256 bits key length"]
    #[inline(always)]
    pub fn _256bits(self) -> &'a mut crate::W<REG> {
        self.variant(KeyLen::_256bits)
    }
    #[doc = "Use 128 bits key length"]
    #[inline(always)]
    pub fn _128bits(self) -> &'a mut crate::W<REG> {
        self.variant(KeyLen::_128bits)
    }
}
#[doc = "Set number of permutation rounds, default value is 20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NumOfRounds {
    #[doc = "0: Use 20 rounds of rotation (default)"]
    Default = 0,
    #[doc = "1: Use 12 rounds of rotation"]
    _12rounds = 1,
    #[doc = "2: Use 8 rounds of rotation"]
    _8rounds = 2,
}
impl From<NumOfRounds> for u8 {
    #[inline(always)]
    fn from(variant: NumOfRounds) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NumOfRounds {
    type Ux = u8;
}
impl crate::IsEnum for NumOfRounds {}
#[doc = "Field `NUM_OF_ROUNDS` reader - Set number of permutation rounds, default value is 20."]
pub type NumOfRoundsR = crate::FieldReader<NumOfRounds>;
impl NumOfRoundsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NumOfRounds> {
        match self.bits {
            0 => Some(NumOfRounds::Default),
            1 => Some(NumOfRounds::_12rounds),
            2 => Some(NumOfRounds::_8rounds),
            _ => None,
        }
    }
    #[doc = "Use 20 rounds of rotation (default)"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == NumOfRounds::Default
    }
    #[doc = "Use 12 rounds of rotation"]
    #[inline(always)]
    pub fn is_12rounds(&self) -> bool {
        *self == NumOfRounds::_12rounds
    }
    #[doc = "Use 8 rounds of rotation"]
    #[inline(always)]
    pub fn is_8rounds(&self) -> bool {
        *self == NumOfRounds::_8rounds
    }
}
#[doc = "Field `NUM_OF_ROUNDS` writer - Set number of permutation rounds, default value is 20."]
pub type NumOfRoundsW<'a, REG> = crate::FieldWriter<'a, REG, 2, NumOfRounds>;
impl<'a, REG> NumOfRoundsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use 20 rounds of rotation (default)"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(NumOfRounds::Default)
    }
    #[doc = "Use 12 rounds of rotation"]
    #[inline(always)]
    pub fn _12rounds(self) -> &'a mut crate::W<REG> {
        self.variant(NumOfRounds::_12rounds)
    }
    #[doc = "Use 8 rounds of rotation"]
    #[inline(always)]
    pub fn _8rounds(self) -> &'a mut crate::W<REG> {
        self.variant(NumOfRounds::_8rounds)
    }
}
#[doc = "Reset block counter for new messages\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetBlockCnt {
    #[doc = "0: Use current block counter value"]
    Disable = 0,
    #[doc = "1: Reset block counter value to zero"]
    Enable = 1,
}
impl From<ResetBlockCnt> for bool {
    #[inline(always)]
    fn from(variant: ResetBlockCnt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET_BLOCK_CNT` reader - Reset block counter for new messages"]
pub type ResetBlockCntR = crate::BitReader<ResetBlockCnt>;
impl ResetBlockCntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResetBlockCnt {
        match self.bits {
            false => ResetBlockCnt::Disable,
            true => ResetBlockCnt::Enable,
        }
    }
    #[doc = "Use current block counter value"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ResetBlockCnt::Disable
    }
    #[doc = "Reset block counter value to zero"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ResetBlockCnt::Enable
    }
}
#[doc = "Field `RESET_BLOCK_CNT` writer - Reset block counter for new messages"]
pub type ResetBlockCntW<'a, REG> = crate::BitWriter<'a, REG, ResetBlockCnt>;
impl<'a, REG> ResetBlockCntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use current block counter value"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ResetBlockCnt::Disable)
    }
    #[doc = "Reset block counter value to zero"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ResetBlockCnt::Enable)
    }
}
#[doc = "Use 96 bits Initialization Vector (IV)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UseIv96bit {
    #[doc = "0: Use default size IV of 64 bit"]
    Disable = 0,
    #[doc = "1: The IV is 96 bits"]
    Enable = 1,
}
impl From<UseIv96bit> for bool {
    #[inline(always)]
    fn from(variant: UseIv96bit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USE_IV_96BIT` reader - Use 96 bits Initialization Vector (IV)"]
pub type UseIv96bitR = crate::BitReader<UseIv96bit>;
impl UseIv96bitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UseIv96bit {
        match self.bits {
            false => UseIv96bit::Disable,
            true => UseIv96bit::Enable,
        }
    }
    #[doc = "Use default size IV of 64 bit"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UseIv96bit::Disable
    }
    #[doc = "The IV is 96 bits"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UseIv96bit::Enable
    }
}
#[doc = "Field `USE_IV_96BIT` writer - Use 96 bits Initialization Vector (IV)"]
pub type UseIv96bitW<'a, REG> = crate::BitWriter<'a, REG, UseIv96bit>;
impl<'a, REG> UseIv96bitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use default size IV of 64 bit"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(UseIv96bit::Disable)
    }
    #[doc = "The IV is 96 bits"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(UseIv96bit::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Run engine in ChaCha or Salsa mode"]
    #[inline(always)]
    pub fn chacha_or_salsa(&self) -> ChachaOrSalsaR {
        ChachaOrSalsaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Perform initialization for a new message"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate the key to use in Poly1305 message authentication code calculation."]
    #[inline(always)]
    pub fn gen_key_poly1305(&self) -> GenKeyPoly1305R {
        GenKeyPoly1305R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Key length selection."]
    #[inline(always)]
    pub fn key_len(&self) -> KeyLenR {
        KeyLenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Set number of permutation rounds, default value is 20."]
    #[inline(always)]
    pub fn num_of_rounds(&self) -> NumOfRoundsR {
        NumOfRoundsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 9 - Reset block counter for new messages"]
    #[inline(always)]
    pub fn reset_block_cnt(&self) -> ResetBlockCntR {
        ResetBlockCntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Use 96 bits Initialization Vector (IV)"]
    #[inline(always)]
    pub fn use_iv_96bit(&self) -> UseIv96bitR {
        UseIv96bitR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run engine in ChaCha or Salsa mode"]
    #[inline(always)]
    pub fn chacha_or_salsa(&mut self) -> ChachaOrSalsaW<'_, ChachaControlSpec> {
        ChachaOrSalsaW::new(self, 0)
    }
    #[doc = "Bit 1 - Perform initialization for a new message"]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<'_, ChachaControlSpec> {
        InitW::new(self, 1)
    }
    #[doc = "Bit 2 - Generate the key to use in Poly1305 message authentication code calculation."]
    #[inline(always)]
    pub fn gen_key_poly1305(&mut self) -> GenKeyPoly1305W<'_, ChachaControlSpec> {
        GenKeyPoly1305W::new(self, 2)
    }
    #[doc = "Bit 3 - Key length selection."]
    #[inline(always)]
    pub fn key_len(&mut self) -> KeyLenW<'_, ChachaControlSpec> {
        KeyLenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Set number of permutation rounds, default value is 20."]
    #[inline(always)]
    pub fn num_of_rounds(&mut self) -> NumOfRoundsW<'_, ChachaControlSpec> {
        NumOfRoundsW::new(self, 4)
    }
    #[doc = "Bit 9 - Reset block counter for new messages"]
    #[inline(always)]
    pub fn reset_block_cnt(&mut self) -> ResetBlockCntW<'_, ChachaControlSpec> {
        ResetBlockCntW::new(self, 9)
    }
    #[doc = "Bit 10 - Use 96 bits Initialization Vector (IV)"]
    #[inline(always)]
    pub fn use_iv_96bit(&mut self) -> UseIv96bitW<'_, ChachaControlSpec> {
        UseIv96bitW::new(self, 10)
    }
}
#[doc = "Control the CHACHA engine behavior.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaControlSpec;
impl crate::RegisterSpec for ChachaControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chacha_control::R`](R) reader structure"]
impl crate::Readable for ChachaControlSpec {}
#[doc = "`write(|w| ..)` method takes [`chacha_control::W`](W) writer structure"]
impl crate::Writable for ChachaControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHACHA_CONTROL to value 0"]
impl crate::Resettable for ChachaControlSpec {}
