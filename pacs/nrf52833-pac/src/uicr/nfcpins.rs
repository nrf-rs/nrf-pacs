#[doc = "Register `NFCPINS` reader"]
pub type R = crate::R<NfcpinsSpec>;
#[doc = "Register `NFCPINS` writer"]
pub type W = crate::W<NfcpinsSpec>;
#[doc = "Setting of pins dedicated to NFC functionality\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protect {
    #[doc = "0: Operation as GPIO pins. Same protection as normal GPIO pins."]
    Disabled = 0,
    #[doc = "1: Operation as NFC antenna pins. Configures the protection for NFC operation."]
    Nfc = 1,
}
impl From<Protect> for bool {
    #[inline(always)]
    fn from(variant: Protect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTECT` reader - Setting of pins dedicated to NFC functionality"]
pub type ProtectR = crate::BitReader<Protect>;
impl ProtectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Protect {
        match self.bits {
            false => Protect::Disabled,
            true => Protect::Nfc,
        }
    }
    #[doc = "Operation as GPIO pins. Same protection as normal GPIO pins."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Protect::Disabled
    }
    #[doc = "Operation as NFC antenna pins. Configures the protection for NFC operation."]
    #[inline(always)]
    pub fn is_nfc(&self) -> bool {
        *self == Protect::Nfc
    }
}
#[doc = "Field `PROTECT` writer - Setting of pins dedicated to NFC functionality"]
pub type ProtectW<'a, REG> = crate::BitWriter<'a, REG, Protect>;
impl<'a, REG> ProtectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation as GPIO pins. Same protection as normal GPIO pins."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::Disabled)
    }
    #[doc = "Operation as NFC antenna pins. Configures the protection for NFC operation."]
    #[inline(always)]
    pub fn nfc(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::Nfc)
    }
}
impl R {
    #[doc = "Bit 0 - Setting of pins dedicated to NFC functionality"]
    #[inline(always)]
    pub fn protect(&self) -> ProtectR {
        ProtectR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting of pins dedicated to NFC functionality"]
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<'_, NfcpinsSpec> {
        ProtectW::new(self, 0)
    }
}
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcpins::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcpins::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NfcpinsSpec;
impl crate::RegisterSpec for NfcpinsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nfcpins::R`](R) reader structure"]
impl crate::Readable for NfcpinsSpec {}
#[doc = "`write(|w| ..)` method takes [`nfcpins::W`](W) writer structure"]
impl crate::Writable for NfcpinsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NFCPINS to value 0xffff_ffff"]
impl crate::Resettable for NfcpinsSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
