#[doc = "Register `ERRORSTATUS` reader"]
pub type R = crate::R<ErrorstatusSpec>;
#[doc = "Register `ERRORSTATUS` writer"]
pub type W = crate::W<ErrorstatusSpec>;
#[doc = "Field `FRAMEDELAYTIMEOUT` reader - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
pub type FramedelaytimeoutR = crate::BitReader;
#[doc = "Field `FRAMEDELAYTIMEOUT` writer - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
pub type FramedelaytimeoutW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NFCFIELDTOOSTRONG` reader - Field level is too high at max load resistance"]
pub type NfcfieldtoostrongR = crate::BitReader;
#[doc = "Field `NFCFIELDTOOSTRONG` writer - Field level is too high at max load resistance"]
pub type NfcfieldtoostrongW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NFCFIELDTOOWEAK` reader - Field level is too low at min load resistance"]
pub type NfcfieldtooweakR = crate::BitReader;
#[doc = "Field `NFCFIELDTOOWEAK` writer - Field level is too low at min load resistance"]
pub type NfcfieldtooweakW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn framedelaytimeout(&self) -> FramedelaytimeoutR {
        FramedelaytimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Field level is too high at max load resistance"]
    #[inline(always)]
    pub fn nfcfieldtoostrong(&self) -> NfcfieldtoostrongR {
        NfcfieldtoostrongR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Field level is too low at min load resistance"]
    #[inline(always)]
    pub fn nfcfieldtooweak(&self) -> NfcfieldtooweakR {
        NfcfieldtooweakR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn framedelaytimeout(&mut self) -> FramedelaytimeoutW<'_, ErrorstatusSpec> {
        FramedelaytimeoutW::new(self, 0)
    }
    #[doc = "Bit 2 - Field level is too high at max load resistance"]
    #[inline(always)]
    pub fn nfcfieldtoostrong(&mut self) -> NfcfieldtoostrongW<'_, ErrorstatusSpec> {
        NfcfieldtoostrongW::new(self, 2)
    }
    #[doc = "Bit 3 - Field level is too low at min load resistance"]
    #[inline(always)]
    pub fn nfcfieldtooweak(&mut self) -> NfcfieldtooweakW<'_, ErrorstatusSpec> {
        NfcfieldtooweakW::new(self, 3)
    }
}
#[doc = "NFC Error Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`errorstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorstatusSpec;
impl crate::RegisterSpec for ErrorstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errorstatus::R`](R) reader structure"]
impl crate::Readable for ErrorstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`errorstatus::W`](W) writer structure"]
impl crate::Writable for ErrorstatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0d;
}
#[doc = "`reset()` method sets ERRORSTATUS to value 0"]
impl crate::Resettable for ErrorstatusSpec {}
