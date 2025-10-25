#[doc = "Register `REFSEL` reader"]
pub type R = crate::R<RefselSpec>;
#[doc = "Register `REFSEL` writer"]
pub type W = crate::W<RefselSpec>;
#[doc = "Reference select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    Int1v2 = 0,
    #[doc = "1: VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    Int1v8 = 1,
    #[doc = "2: VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    Int2v4 = 2,
    #[doc = "4: VREF = VDD"]
    Vdd = 4,
    #[doc = "7: VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    Aref = 7,
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
    pub const fn variant(&self) -> Option<Refsel> {
        match self.bits {
            0 => Some(Refsel::Int1v2),
            1 => Some(Refsel::Int1v8),
            2 => Some(Refsel::Int2v4),
            4 => Some(Refsel::Vdd),
            7 => Some(Refsel::Aref),
            _ => None,
        }
    }
    #[doc = "VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    #[inline(always)]
    pub fn is_int1v2(&self) -> bool {
        *self == Refsel::Int1v2
    }
    #[doc = "VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    #[inline(always)]
    pub fn is_int1v8(&self) -> bool {
        *self == Refsel::Int1v8
    }
    #[doc = "VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    #[inline(always)]
    pub fn is_int2v4(&self) -> bool {
        *self == Refsel::Int2v4
    }
    #[doc = "VREF = VDD"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == Refsel::Vdd
    }
    #[doc = "VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == Refsel::Aref
    }
}
#[doc = "Field `REFSEL` writer - Reference select"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Refsel>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    #[inline(always)]
    pub fn int1v2(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Int1v2)
    }
    #[doc = "VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    #[inline(always)]
    pub fn int1v8(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Int1v8)
    }
    #[doc = "VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    #[inline(always)]
    pub fn int2v4(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Int2v4)
    }
    #[doc = "VREF = VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Vdd)
    }
    #[doc = "VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Aref)
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference select"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference select"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<'_, RefselSpec> {
        RefselW::new(self, 0)
    }
}
#[doc = "Reference source select for single-ended mode\n\nYou can [`read`](crate::Reg::read) this register and get [`refsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
