#[doc = "Register `SELRES` reader"]
pub type R = crate::R<SelresSpec>;
#[doc = "Register `SELRES` writer"]
pub type W = crate::W<SelresSpec>;
#[doc = "Field `RFU10` reader - Reserved for future use. Shall be 0."]
pub type Rfu10R = crate::FieldReader;
#[doc = "Field `RFU10` writer - Reserved for future use. Shall be 0."]
pub type Rfu10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Cascade bit (controlled by hardware, write has no effect)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cascade {
    #[doc = "0: NFCID1 complete"]
    Complete = 0,
    #[doc = "1: NFCID1 not complete"]
    NotComplete = 1,
}
impl From<Cascade> for bool {
    #[inline(always)]
    fn from(variant: Cascade) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASCADE` reader - Cascade bit (controlled by hardware, write has no effect)"]
pub type CascadeR = crate::BitReader<Cascade>;
impl CascadeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cascade {
        match self.bits {
            false => Cascade::Complete,
            true => Cascade::NotComplete,
        }
    }
    #[doc = "NFCID1 complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Cascade::Complete
    }
    #[doc = "NFCID1 not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == Cascade::NotComplete
    }
}
#[doc = "Field `CASCADE` writer - Cascade bit (controlled by hardware, write has no effect)"]
pub type CascadeW<'a, REG> = crate::BitWriter<'a, REG, Cascade>;
impl<'a, REG> CascadeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NFCID1 complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(Cascade::Complete)
    }
    #[doc = "NFCID1 not complete"]
    #[inline(always)]
    pub fn not_complete(self) -> &'a mut crate::W<REG> {
        self.variant(Cascade::NotComplete)
    }
}
#[doc = "Field `RFU43` reader - Reserved for future use. Shall be 0."]
pub type Rfu43R = crate::FieldReader;
#[doc = "Field `RFU43` writer - Reserved for future use. Shall be 0."]
pub type Rfu43W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PROTOCOL` reader - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type ProtocolR = crate::FieldReader;
#[doc = "Field `PROTOCOL` writer - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type ProtocolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RFU7` reader - Reserved for future use. Shall be 0."]
pub type Rfu7R = crate::BitReader;
#[doc = "Field `RFU7` writer - Reserved for future use. Shall be 0."]
pub type Rfu7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu10(&self) -> Rfu10R {
        Rfu10R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Cascade bit (controlled by hardware, write has no effect)"]
    #[inline(always)]
    pub fn cascade(&self) -> CascadeR {
        CascadeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu43(&self) -> Rfu43R {
        Rfu43R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn protocol(&self) -> ProtocolR {
        ProtocolR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu7(&self) -> Rfu7R {
        Rfu7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu10(&mut self) -> Rfu10W<'_, SelresSpec> {
        Rfu10W::new(self, 0)
    }
    #[doc = "Bit 2 - Cascade bit (controlled by hardware, write has no effect)"]
    #[inline(always)]
    pub fn cascade(&mut self) -> CascadeW<'_, SelresSpec> {
        CascadeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu43(&mut self) -> Rfu43W<'_, SelresSpec> {
        Rfu43W::new(self, 3)
    }
    #[doc = "Bits 5:6 - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn protocol(&mut self) -> ProtocolW<'_, SelresSpec> {
        ProtocolW::new(self, 5)
    }
    #[doc = "Bit 7 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu7(&mut self) -> Rfu7W<'_, SelresSpec> {
        Rfu7W::new(self, 7)
    }
}
#[doc = "NFC-A SEL_RES auto-response settings\n\nYou can [`read`](crate::Reg::read) this register and get [`selres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SelresSpec;
impl crate::RegisterSpec for SelresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`selres::R`](R) reader structure"]
impl crate::Readable for SelresSpec {}
#[doc = "`write(|w| ..)` method takes [`selres::W`](W) writer structure"]
impl crate::Writable for SelresSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SELRES to value 0"]
impl crate::Resettable for SelresSpec {}
