#[doc = "Register `NFCID1_2ND_LAST` reader"]
pub type R = crate::R<Nfcid1_2ndLastSpec>;
#[doc = "Register `NFCID1_2ND_LAST` writer"]
pub type W = crate::W<Nfcid1_2ndLastSpec>;
#[doc = "Field `NFCID1_V` reader - NFCID1 byte V"]
pub type Nfcid1VR = crate::FieldReader;
#[doc = "Field `NFCID1_V` writer - NFCID1 byte V"]
pub type Nfcid1VW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NFCID1_U` reader - NFCID1 byte U"]
pub type Nfcid1UR = crate::FieldReader;
#[doc = "Field `NFCID1_U` writer - NFCID1 byte U"]
pub type Nfcid1UW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NFCID1_T` reader - NFCID1 byte T"]
pub type Nfcid1TR = crate::FieldReader;
#[doc = "Field `NFCID1_T` writer - NFCID1 byte T"]
pub type Nfcid1TW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte V"]
    #[inline(always)]
    pub fn nfcid1_v(&self) -> Nfcid1VR {
        Nfcid1VR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte U"]
    #[inline(always)]
    pub fn nfcid1_u(&self) -> Nfcid1UR {
        Nfcid1UR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte T"]
    #[inline(always)]
    pub fn nfcid1_t(&self) -> Nfcid1TR {
        Nfcid1TR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte V"]
    #[inline(always)]
    pub fn nfcid1_v(&mut self) -> Nfcid1VW<'_, Nfcid1_2ndLastSpec> {
        Nfcid1VW::new(self, 0)
    }
    #[doc = "Bits 8:15 - NFCID1 byte U"]
    #[inline(always)]
    pub fn nfcid1_u(&mut self) -> Nfcid1UW<'_, Nfcid1_2ndLastSpec> {
        Nfcid1UW::new(self, 8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte T"]
    #[inline(always)]
    pub fn nfcid1_t(&mut self) -> Nfcid1TW<'_, Nfcid1_2ndLastSpec> {
        Nfcid1TW::new(self, 16)
    }
}
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcid1_2nd_last::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcid1_2nd_last::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Nfcid1_2ndLastSpec;
impl crate::RegisterSpec for Nfcid1_2ndLastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nfcid1_2nd_last::R`](R) reader structure"]
impl crate::Readable for Nfcid1_2ndLastSpec {}
#[doc = "`write(|w| ..)` method takes [`nfcid1_2nd_last::W`](W) writer structure"]
impl crate::Writable for Nfcid1_2ndLastSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NFCID1_2ND_LAST to value 0"]
impl crate::Resettable for Nfcid1_2ndLastSpec {}
