#[doc = "Register `SENSRES` reader"]
pub type R = crate::R<SensresSpec>;
#[doc = "Register `SENSRES` writer"]
pub type W = crate::W<SensresSpec>;
#[doc = "Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bitframesdd {
    #[doc = "0: SDD pattern 00000"]
    Sdd00000 = 0,
    #[doc = "1: SDD pattern 00001"]
    Sdd00001 = 1,
    #[doc = "2: SDD pattern 00010"]
    Sdd00010 = 2,
    #[doc = "4: SDD pattern 00100"]
    Sdd00100 = 4,
    #[doc = "8: SDD pattern 01000"]
    Sdd01000 = 8,
    #[doc = "16: SDD pattern 10000"]
    Sdd10000 = 16,
}
impl From<Bitframesdd> for u8 {
    #[inline(always)]
    fn from(variant: Bitframesdd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bitframesdd {
    type Ux = u8;
}
impl crate::IsEnum for Bitframesdd {}
#[doc = "Field `BITFRAMESDD` reader - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type BitframesddR = crate::FieldReader<Bitframesdd>;
impl BitframesddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bitframesdd> {
        match self.bits {
            0 => Some(Bitframesdd::Sdd00000),
            1 => Some(Bitframesdd::Sdd00001),
            2 => Some(Bitframesdd::Sdd00010),
            4 => Some(Bitframesdd::Sdd00100),
            8 => Some(Bitframesdd::Sdd01000),
            16 => Some(Bitframesdd::Sdd10000),
            _ => None,
        }
    }
    #[doc = "SDD pattern 00000"]
    #[inline(always)]
    pub fn is_sdd00000(&self) -> bool {
        *self == Bitframesdd::Sdd00000
    }
    #[doc = "SDD pattern 00001"]
    #[inline(always)]
    pub fn is_sdd00001(&self) -> bool {
        *self == Bitframesdd::Sdd00001
    }
    #[doc = "SDD pattern 00010"]
    #[inline(always)]
    pub fn is_sdd00010(&self) -> bool {
        *self == Bitframesdd::Sdd00010
    }
    #[doc = "SDD pattern 00100"]
    #[inline(always)]
    pub fn is_sdd00100(&self) -> bool {
        *self == Bitframesdd::Sdd00100
    }
    #[doc = "SDD pattern 01000"]
    #[inline(always)]
    pub fn is_sdd01000(&self) -> bool {
        *self == Bitframesdd::Sdd01000
    }
    #[doc = "SDD pattern 10000"]
    #[inline(always)]
    pub fn is_sdd10000(&self) -> bool {
        *self == Bitframesdd::Sdd10000
    }
}
#[doc = "Field `BITFRAMESDD` writer - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type BitframesddW<'a, REG> = crate::FieldWriter<'a, REG, 5, Bitframesdd>;
impl<'a, REG> BitframesddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDD pattern 00000"]
    #[inline(always)]
    pub fn sdd00000(self) -> &'a mut crate::W<REG> {
        self.variant(Bitframesdd::Sdd00000)
    }
    #[doc = "SDD pattern 00001"]
    #[inline(always)]
    pub fn sdd00001(self) -> &'a mut crate::W<REG> {
        self.variant(Bitframesdd::Sdd00001)
    }
    #[doc = "SDD pattern 00010"]
    #[inline(always)]
    pub fn sdd00010(self) -> &'a mut crate::W<REG> {
        self.variant(Bitframesdd::Sdd00010)
    }
    #[doc = "SDD pattern 00100"]
    #[inline(always)]
    pub fn sdd00100(self) -> &'a mut crate::W<REG> {
        self.variant(Bitframesdd::Sdd00100)
    }
    #[doc = "SDD pattern 01000"]
    #[inline(always)]
    pub fn sdd01000(self) -> &'a mut crate::W<REG> {
        self.variant(Bitframesdd::Sdd01000)
    }
    #[doc = "SDD pattern 10000"]
    #[inline(always)]
    pub fn sdd10000(self) -> &'a mut crate::W<REG> {
        self.variant(Bitframesdd::Sdd10000)
    }
}
#[doc = "Field `RFU5` reader - Reserved for future use. Shall be 0."]
pub type Rfu5R = crate::BitReader;
#[doc = "Field `RFU5` writer - Reserved for future use. Shall be 0."]
pub type Rfu5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "NFCID1 size. This value is used by the auto collision resolution engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfcidsize {
    #[doc = "0: NFCID1 size: single (4 bytes)"]
    Nfcid1single = 0,
    #[doc = "1: NFCID1 size: double (7 bytes)"]
    Nfcid1double = 1,
    #[doc = "2: NFCID1 size: triple (10 bytes)"]
    Nfcid1triple = 2,
}
impl From<Nfcidsize> for u8 {
    #[inline(always)]
    fn from(variant: Nfcidsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfcidsize {
    type Ux = u8;
}
impl crate::IsEnum for Nfcidsize {}
#[doc = "Field `NFCIDSIZE` reader - NFCID1 size. This value is used by the auto collision resolution engine."]
pub type NfcidsizeR = crate::FieldReader<Nfcidsize>;
impl NfcidsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nfcidsize> {
        match self.bits {
            0 => Some(Nfcidsize::Nfcid1single),
            1 => Some(Nfcidsize::Nfcid1double),
            2 => Some(Nfcidsize::Nfcid1triple),
            _ => None,
        }
    }
    #[doc = "NFCID1 size: single (4 bytes)"]
    #[inline(always)]
    pub fn is_nfcid1single(&self) -> bool {
        *self == Nfcidsize::Nfcid1single
    }
    #[doc = "NFCID1 size: double (7 bytes)"]
    #[inline(always)]
    pub fn is_nfcid1double(&self) -> bool {
        *self == Nfcidsize::Nfcid1double
    }
    #[doc = "NFCID1 size: triple (10 bytes)"]
    #[inline(always)]
    pub fn is_nfcid1triple(&self) -> bool {
        *self == Nfcidsize::Nfcid1triple
    }
}
#[doc = "Field `NFCIDSIZE` writer - NFCID1 size. This value is used by the auto collision resolution engine."]
pub type NfcidsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nfcidsize>;
impl<'a, REG> NfcidsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NFCID1 size: single (4 bytes)"]
    #[inline(always)]
    pub fn nfcid1single(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcidsize::Nfcid1single)
    }
    #[doc = "NFCID1 size: double (7 bytes)"]
    #[inline(always)]
    pub fn nfcid1double(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcidsize::Nfcid1double)
    }
    #[doc = "NFCID1 size: triple (10 bytes)"]
    #[inline(always)]
    pub fn nfcid1triple(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcidsize::Nfcid1triple)
    }
}
#[doc = "Field `PLATFCONFIG` reader - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type PlatfconfigR = crate::FieldReader;
#[doc = "Field `PLATFCONFIG` writer - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type PlatfconfigW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RFU74` reader - Reserved for future use. Shall be 0."]
pub type Rfu74R = crate::FieldReader;
#[doc = "Field `RFU74` writer - Reserved for future use. Shall be 0."]
pub type Rfu74W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn bitframesdd(&self) -> BitframesddR {
        BitframesddR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu5(&self) -> Rfu5R {
        Rfu5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - NFCID1 size. This value is used by the auto collision resolution engine."]
    #[inline(always)]
    pub fn nfcidsize(&self) -> NfcidsizeR {
        NfcidsizeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn platfconfig(&self) -> PlatfconfigR {
        PlatfconfigR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu74(&self) -> Rfu74R {
        Rfu74R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn bitframesdd(&mut self) -> BitframesddW<'_, SensresSpec> {
        BitframesddW::new(self, 0)
    }
    #[doc = "Bit 5 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu5(&mut self) -> Rfu5W<'_, SensresSpec> {
        Rfu5W::new(self, 5)
    }
    #[doc = "Bits 6:7 - NFCID1 size. This value is used by the auto collision resolution engine."]
    #[inline(always)]
    pub fn nfcidsize(&mut self) -> NfcidsizeW<'_, SensresSpec> {
        NfcidsizeW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn platfconfig(&mut self) -> PlatfconfigW<'_, SensresSpec> {
        PlatfconfigW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu74(&mut self) -> Rfu74W<'_, SensresSpec> {
        Rfu74W::new(self, 12)
    }
}
#[doc = "NFC-A SENS_RES auto-response settings\n\nYou can [`read`](crate::Reg::read) this register and get [`sensres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sensres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SensresSpec;
impl crate::RegisterSpec for SensresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sensres::R`](R) reader structure"]
impl crate::Readable for SensresSpec {}
#[doc = "`write(|w| ..)` method takes [`sensres::W`](W) writer structure"]
impl crate::Writable for SensresSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SENSRES to value 0x01"]
impl crate::Resettable for SensresSpec {
    const RESET_VALUE: u32 = 0x01;
}
