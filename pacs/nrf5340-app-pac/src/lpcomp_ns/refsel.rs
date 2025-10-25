#[doc = "Register `REFSEL` reader"]
pub type R = crate::R<RefselSpec>;
#[doc = "Register `REFSEL` writer"]
pub type W = crate::W<RefselSpec>;
#[doc = "Reference select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: VDD * 1/8 selected as reference"]
    Ref1_8vdd = 0,
    #[doc = "1: VDD * 2/8 selected as reference"]
    Ref2_8vdd = 1,
    #[doc = "2: VDD * 3/8 selected as reference"]
    Ref3_8vdd = 2,
    #[doc = "3: VDD * 4/8 selected as reference"]
    Ref4_8vdd = 3,
    #[doc = "4: VDD * 5/8 selected as reference"]
    Ref5_8vdd = 4,
    #[doc = "5: VDD * 6/8 selected as reference"]
    Ref6_8vdd = 5,
    #[doc = "6: VDD * 7/8 selected as reference"]
    Ref7_8vdd = 6,
    #[doc = "7: External analog reference selected"]
    Aref = 7,
    #[doc = "8: VDD * 1/16 selected as reference"]
    Ref1_16vdd = 8,
    #[doc = "9: VDD * 3/16 selected as reference"]
    Ref3_16vdd = 9,
    #[doc = "10: VDD * 5/16 selected as reference"]
    Ref5_16vdd = 10,
    #[doc = "11: VDD * 7/16 selected as reference"]
    Ref7_16vdd = 11,
    #[doc = "12: VDD * 9/16 selected as reference"]
    Ref9_16vdd = 12,
    #[doc = "13: VDD * 11/16 selected as reference"]
    Ref11_16vdd = 13,
    #[doc = "14: VDD * 13/16 selected as reference"]
    Ref13_16vdd = 14,
    #[doc = "15: VDD * 15/16 selected as reference"]
    Ref15_16vdd = 15,
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsel {
    type Ux = u8;
}
impl crate::IsEnum for Refsel {}
#[doc = "Field `REFSEL` reader - Reference select"]
pub type RefselR = crate::FieldReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refsel {
        match self.bits {
            0 => Refsel::Ref1_8vdd,
            1 => Refsel::Ref2_8vdd,
            2 => Refsel::Ref3_8vdd,
            3 => Refsel::Ref4_8vdd,
            4 => Refsel::Ref5_8vdd,
            5 => Refsel::Ref6_8vdd,
            6 => Refsel::Ref7_8vdd,
            7 => Refsel::Aref,
            8 => Refsel::Ref1_16vdd,
            9 => Refsel::Ref3_16vdd,
            10 => Refsel::Ref5_16vdd,
            11 => Refsel::Ref7_16vdd,
            12 => Refsel::Ref9_16vdd,
            13 => Refsel::Ref11_16vdd,
            14 => Refsel::Ref13_16vdd,
            15 => Refsel::Ref15_16vdd,
            _ => unreachable!(),
        }
    }
    #[doc = "VDD * 1/8 selected as reference"]
    #[inline(always)]
    pub fn is_ref1_8vdd(&self) -> bool {
        *self == Refsel::Ref1_8vdd
    }
    #[doc = "VDD * 2/8 selected as reference"]
    #[inline(always)]
    pub fn is_ref2_8vdd(&self) -> bool {
        *self == Refsel::Ref2_8vdd
    }
    #[doc = "VDD * 3/8 selected as reference"]
    #[inline(always)]
    pub fn is_ref3_8vdd(&self) -> bool {
        *self == Refsel::Ref3_8vdd
    }
    #[doc = "VDD * 4/8 selected as reference"]
    #[inline(always)]
    pub fn is_ref4_8vdd(&self) -> bool {
        *self == Refsel::Ref4_8vdd
    }
    #[doc = "VDD * 5/8 selected as reference"]
    #[inline(always)]
    pub fn is_ref5_8vdd(&self) -> bool {
        *self == Refsel::Ref5_8vdd
    }
    #[doc = "VDD * 6/8 selected as reference"]
    #[inline(always)]
    pub fn is_ref6_8vdd(&self) -> bool {
        *self == Refsel::Ref6_8vdd
    }
    #[doc = "VDD * 7/8 selected as reference"]
    #[inline(always)]
    pub fn is_ref7_8vdd(&self) -> bool {
        *self == Refsel::Ref7_8vdd
    }
    #[doc = "External analog reference selected"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == Refsel::Aref
    }
    #[doc = "VDD * 1/16 selected as reference"]
    #[inline(always)]
    pub fn is_ref1_16vdd(&self) -> bool {
        *self == Refsel::Ref1_16vdd
    }
    #[doc = "VDD * 3/16 selected as reference"]
    #[inline(always)]
    pub fn is_ref3_16vdd(&self) -> bool {
        *self == Refsel::Ref3_16vdd
    }
    #[doc = "VDD * 5/16 selected as reference"]
    #[inline(always)]
    pub fn is_ref5_16vdd(&self) -> bool {
        *self == Refsel::Ref5_16vdd
    }
    #[doc = "VDD * 7/16 selected as reference"]
    #[inline(always)]
    pub fn is_ref7_16vdd(&self) -> bool {
        *self == Refsel::Ref7_16vdd
    }
    #[doc = "VDD * 9/16 selected as reference"]
    #[inline(always)]
    pub fn is_ref9_16vdd(&self) -> bool {
        *self == Refsel::Ref9_16vdd
    }
    #[doc = "VDD * 11/16 selected as reference"]
    #[inline(always)]
    pub fn is_ref11_16vdd(&self) -> bool {
        *self == Refsel::Ref11_16vdd
    }
    #[doc = "VDD * 13/16 selected as reference"]
    #[inline(always)]
    pub fn is_ref13_16vdd(&self) -> bool {
        *self == Refsel::Ref13_16vdd
    }
    #[doc = "VDD * 15/16 selected as reference"]
    #[inline(always)]
    pub fn is_ref15_16vdd(&self) -> bool {
        *self == Refsel::Ref15_16vdd
    }
}
#[doc = "Field `REFSEL` writer - Reference select"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Refsel, crate::Safe>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VDD * 1/8 selected as reference"]
    #[inline(always)]
    pub fn ref1_8vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref1_8vdd)
    }
    #[doc = "VDD * 2/8 selected as reference"]
    #[inline(always)]
    pub fn ref2_8vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref2_8vdd)
    }
    #[doc = "VDD * 3/8 selected as reference"]
    #[inline(always)]
    pub fn ref3_8vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref3_8vdd)
    }
    #[doc = "VDD * 4/8 selected as reference"]
    #[inline(always)]
    pub fn ref4_8vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref4_8vdd)
    }
    #[doc = "VDD * 5/8 selected as reference"]
    #[inline(always)]
    pub fn ref5_8vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref5_8vdd)
    }
    #[doc = "VDD * 6/8 selected as reference"]
    #[inline(always)]
    pub fn ref6_8vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref6_8vdd)
    }
    #[doc = "VDD * 7/8 selected as reference"]
    #[inline(always)]
    pub fn ref7_8vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref7_8vdd)
    }
    #[doc = "External analog reference selected"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Aref)
    }
    #[doc = "VDD * 1/16 selected as reference"]
    #[inline(always)]
    pub fn ref1_16vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref1_16vdd)
    }
    #[doc = "VDD * 3/16 selected as reference"]
    #[inline(always)]
    pub fn ref3_16vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref3_16vdd)
    }
    #[doc = "VDD * 5/16 selected as reference"]
    #[inline(always)]
    pub fn ref5_16vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref5_16vdd)
    }
    #[doc = "VDD * 7/16 selected as reference"]
    #[inline(always)]
    pub fn ref7_16vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref7_16vdd)
    }
    #[doc = "VDD * 9/16 selected as reference"]
    #[inline(always)]
    pub fn ref9_16vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref9_16vdd)
    }
    #[doc = "VDD * 11/16 selected as reference"]
    #[inline(always)]
    pub fn ref11_16vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref11_16vdd)
    }
    #[doc = "VDD * 13/16 selected as reference"]
    #[inline(always)]
    pub fn ref13_16vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref13_16vdd)
    }
    #[doc = "VDD * 15/16 selected as reference"]
    #[inline(always)]
    pub fn ref15_16vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Ref15_16vdd)
    }
}
impl R {
    #[doc = "Bits 0:3 - Reference select"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reference select"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, RefselSpec> {
        RefselW::new(self, 0)
    }
}
#[doc = "Reference select\n\nYou can [`read`](crate::Reg::read) this register and get [`refsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefselSpec;
impl crate::RegisterSpec for RefselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refsel::R`](R) reader structure"]
impl crate::Readable for RefselSpec {}
#[doc = "`write(|w| ..)` method takes [`refsel::W`](W) writer structure"]
impl crate::Writable for RefselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REFSEL to value 0x04"]
impl crate::Resettable for RefselSpec {
    const RESET_VALUE: u32 = 0x04;
}
