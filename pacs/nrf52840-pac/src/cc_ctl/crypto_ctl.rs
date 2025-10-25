#[doc = "Register `CRYPTO_CTL` writer"]
pub type W = crate::W<CryptoCtlSpec>;
#[doc = "Configure the cryptographic engine mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Bypass cryptographic engine"]
    Bypass = 0,
    #[doc = "1: Use AES engine"]
    Aesactive = 1,
    #[doc = "2: Pipe AES engine output to HASH engine input"]
    AestoHashActive = 2,
    #[doc = "3: Process input using both AES and HASH engine in parallell"]
    AesandHashActive = 3,
    #[doc = "7: Use HASH engine"]
    HashActive = 7,
    #[doc = "9: Calculate AES MAC and bypass"]
    AesmacandBypassActive = 9,
    #[doc = "10: Pipe AES engine output to HASH engine input. The resulting digest output is piped to DOUT buffer."]
    AestoHashAndDoutactive = 10,
    #[doc = "16: Use CHACHA engine"]
    ChaChaActive = 16,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` writer - Configure the cryptographic engine mode."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass cryptographic engine"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Bypass)
    }
    #[doc = "Use AES engine"]
    #[inline(always)]
    pub fn aesactive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Aesactive)
    }
    #[doc = "Pipe AES engine output to HASH engine input"]
    #[inline(always)]
    pub fn aesto_hash_active(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AestoHashActive)
    }
    #[doc = "Process input using both AES and HASH engine in parallell"]
    #[inline(always)]
    pub fn aesand_hash_active(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AesandHashActive)
    }
    #[doc = "Use HASH engine"]
    #[inline(always)]
    pub fn hash_active(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::HashActive)
    }
    #[doc = "Calculate AES MAC and bypass"]
    #[inline(always)]
    pub fn aesmacand_bypass_active(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AesmacandBypassActive)
    }
    #[doc = "Pipe AES engine output to HASH engine input. The resulting digest output is piped to DOUT buffer."]
    #[inline(always)]
    pub fn aesto_hash_and_doutactive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AestoHashAndDoutactive)
    }
    #[doc = "Use CHACHA engine"]
    #[inline(always)]
    pub fn cha_cha_active(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::ChaChaActive)
    }
}
impl W {
    #[doc = "Bits 0:4 - Configure the cryptographic engine mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CryptoCtlSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Defines the cryptographic flow.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_ctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoCtlSpec;
impl crate::RegisterSpec for CryptoCtlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crypto_ctl::W`](W) writer structure"]
impl crate::Writable for CryptoCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRYPTO_CTL to value 0"]
impl crate::Resettable for CryptoCtlSpec {}
