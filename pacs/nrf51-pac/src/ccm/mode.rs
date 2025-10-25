#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "CCM mode operation.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: CCM mode TX"]
    Encryption = 0,
    #[doc = "1: CCM mode TX"]
    Decryption = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - CCM mode operation."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Encryption,
            true => Mode::Decryption,
        }
    }
    #[doc = "CCM mode TX"]
    #[inline(always)]
    pub fn is_encryption(&self) -> bool {
        *self == Mode::Encryption
    }
    #[doc = "CCM mode TX"]
    #[inline(always)]
    pub fn is_decryption(&self) -> bool {
        *self == Mode::Decryption
    }
}
#[doc = "Field `MODE` writer - CCM mode operation."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCM mode TX"]
    #[inline(always)]
    pub fn encryption(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Encryption)
    }
    #[doc = "CCM mode TX"]
    #[inline(always)]
    pub fn decryption(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Decryption)
    }
}
impl R {
    #[doc = "Bit 0 - CCM mode operation."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM mode operation."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ModeSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Operation mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0x01"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0x01;
}
