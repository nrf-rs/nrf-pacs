#[doc = "Register `NFCID1_3RD_LAST` reader"]
pub type R = crate::R<Nfcid1_3rdLastSpec>;
#[doc = "Register `NFCID1_3RD_LAST` writer"]
pub type W = crate::W<Nfcid1_3rdLastSpec>;
#[doc = "Field `NFCID1_S` reader - NFCID1 byte S"]
pub type Nfcid1SR = crate::FieldReader;
#[doc = "Field `NFCID1_S` writer - NFCID1 byte S"]
pub type Nfcid1SW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NFCID1_R` reader - NFCID1 byte R"]
pub type Nfcid1RR = crate::FieldReader;
#[doc = "Field `NFCID1_R` writer - NFCID1 byte R"]
pub type Nfcid1RW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NFCID1_Q` reader - NFCID1 byte Q"]
pub type Nfcid1QR = crate::FieldReader;
#[doc = "Field `NFCID1_Q` writer - NFCID1 byte Q"]
pub type Nfcid1QW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte S"]
    #[inline(always)]
    pub fn nfcid1_s(&self) -> Nfcid1SR {
        Nfcid1SR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte R"]
    #[inline(always)]
    pub fn nfcid1_r(&self) -> Nfcid1RR {
        Nfcid1RR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte Q"]
    #[inline(always)]
    pub fn nfcid1_q(&self) -> Nfcid1QR {
        Nfcid1QR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte S"]
    #[inline(always)]
    pub fn nfcid1_s(&mut self) -> Nfcid1SW<'_, Nfcid1_3rdLastSpec> {
        Nfcid1SW::new(self, 0)
    }
    #[doc = "Bits 8:15 - NFCID1 byte R"]
    #[inline(always)]
    pub fn nfcid1_r(&mut self) -> Nfcid1RW<'_, Nfcid1_3rdLastSpec> {
        Nfcid1RW::new(self, 8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte Q"]
    #[inline(always)]
    pub fn nfcid1_q(&mut self) -> Nfcid1QW<'_, Nfcid1_3rdLastSpec> {
        Nfcid1QW::new(self, 16)
    }
}
#[doc = "Third last NFCID1 part (10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcid1_3rd_last::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcid1_3rd_last::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Nfcid1_3rdLastSpec;
impl crate::RegisterSpec for Nfcid1_3rdLastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nfcid1_3rd_last::R`](R) reader structure"]
impl crate::Readable for Nfcid1_3rdLastSpec {}
#[doc = "`write(|w| ..)` method takes [`nfcid1_3rd_last::W`](W) writer structure"]
impl crate::Writable for Nfcid1_3rdLastSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NFCID1_3RD_LAST to value 0"]
impl crate::Resettable for Nfcid1_3rdLastSpec {}
