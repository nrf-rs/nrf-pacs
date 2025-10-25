#[doc = "Register `CRCCNF` reader"]
pub type R = crate::R<CrccnfSpec>;
#[doc = "Register `CRCCNF` writer"]
pub type W = crate::W<CrccnfSpec>;
#[doc = "CRC length. Decision point: START task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Len {
    #[doc = "0: CRC calculation disabled."]
    Disabled = 0,
    #[doc = "1: One byte long CRC."]
    One = 1,
    #[doc = "2: Two bytes long CRC."]
    Two = 2,
    #[doc = "3: Three bytes long CRC."]
    Three = 3,
}
impl From<Len> for u8 {
    #[inline(always)]
    fn from(variant: Len) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Len {
    type Ux = u8;
}
impl crate::IsEnum for Len {}
#[doc = "Field `LEN` reader - CRC length. Decision point: START task."]
pub type LenR = crate::FieldReader<Len>;
impl LenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Len {
        match self.bits {
            0 => Len::Disabled,
            1 => Len::One,
            2 => Len::Two,
            3 => Len::Three,
            _ => unreachable!(),
        }
    }
    #[doc = "CRC calculation disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Len::Disabled
    }
    #[doc = "One byte long CRC."]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Len::One
    }
    #[doc = "Two bytes long CRC."]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Len::Two
    }
    #[doc = "Three bytes long CRC."]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Len::Three
    }
}
#[doc = "Field `LEN` writer - CRC length. Decision point: START task."]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Len, crate::Safe>;
impl<'a, REG> LenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CRC calculation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Disabled)
    }
    #[doc = "One byte long CRC."]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Len::One)
    }
    #[doc = "Two bytes long CRC."]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Two)
    }
    #[doc = "Three bytes long CRC."]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Three)
    }
}
#[doc = "Leave packet address field out of the CRC calculation. Decision point: START task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skipaddr {
    #[doc = "0: Include packet address in CRC calculation."]
    Include = 0,
    #[doc = "1: Packet address is skipped in CRC calculation. The CRC calculation will start at the first byte after the address."]
    Skip = 1,
}
impl From<Skipaddr> for bool {
    #[inline(always)]
    fn from(variant: Skipaddr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKIPADDR` reader - Leave packet address field out of the CRC calculation. Decision point: START task."]
pub type SkipaddrR = crate::BitReader<Skipaddr>;
impl SkipaddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Skipaddr {
        match self.bits {
            false => Skipaddr::Include,
            true => Skipaddr::Skip,
        }
    }
    #[doc = "Include packet address in CRC calculation."]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Skipaddr::Include
    }
    #[doc = "Packet address is skipped in CRC calculation. The CRC calculation will start at the first byte after the address."]
    #[inline(always)]
    pub fn is_skip(&self) -> bool {
        *self == Skipaddr::Skip
    }
}
#[doc = "Field `SKIPADDR` writer - Leave packet address field out of the CRC calculation. Decision point: START task."]
pub type SkipaddrW<'a, REG> = crate::BitWriter<'a, REG, Skipaddr>;
impl<'a, REG> SkipaddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Include packet address in CRC calculation."]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Skipaddr::Include)
    }
    #[doc = "Packet address is skipped in CRC calculation. The CRC calculation will start at the first byte after the address."]
    #[inline(always)]
    pub fn skip(self) -> &'a mut crate::W<REG> {
        self.variant(Skipaddr::Skip)
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC length. Decision point: START task."]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Leave packet address field out of the CRC calculation. Decision point: START task."]
    #[inline(always)]
    pub fn skipaddr(&self) -> SkipaddrR {
        SkipaddrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC length. Decision point: START task."]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<'_, CrccnfSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bit 8 - Leave packet address field out of the CRC calculation. Decision point: START task."]
    #[inline(always)]
    pub fn skipaddr(&mut self) -> SkipaddrW<'_, CrccnfSpec> {
        SkipaddrW::new(self, 8)
    }
}
#[doc = "CRC configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`crccnf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccnf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrccnfSpec;
impl crate::RegisterSpec for CrccnfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crccnf::R`](R) reader structure"]
impl crate::Readable for CrccnfSpec {}
#[doc = "`write(|w| ..)` method takes [`crccnf::W`](W) writer structure"]
impl crate::Writable for CrccnfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCCNF to value 0"]
impl crate::Resettable for CrccnfSpec {}
