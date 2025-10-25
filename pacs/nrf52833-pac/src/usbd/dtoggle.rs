#[doc = "Register `DTOGGLE` reader"]
pub type R = crate::R<DtoggleSpec>;
#[doc = "Register `DTOGGLE` writer"]
pub type W = crate::W<DtoggleSpec>;
#[doc = "Field `EP` reader - Select bulk endpoint number"]
pub type EpR = crate::FieldReader;
#[doc = "Field `EP` writer - Select bulk endpoint number"]
pub type EpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Selects IN or OUT endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Io {
    #[doc = "0: Selects OUT endpoint"]
    Out = 0,
    #[doc = "1: Selects IN endpoint"]
    In = 1,
}
impl From<Io> for bool {
    #[inline(always)]
    fn from(variant: Io) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO` reader - Selects IN or OUT endpoint"]
pub type IoR = crate::BitReader<Io>;
impl IoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Io {
        match self.bits {
            false => Io::Out,
            true => Io::In,
        }
    }
    #[doc = "Selects OUT endpoint"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Io::Out
    }
    #[doc = "Selects IN endpoint"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Io::In
    }
}
#[doc = "Field `IO` writer - Selects IN or OUT endpoint"]
pub type IoW<'a, REG> = crate::BitWriter<'a, REG, Io>;
impl<'a, REG> IoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects OUT endpoint"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Io::Out)
    }
    #[doc = "Selects IN endpoint"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Io::In)
    }
}
#[doc = "Data toggle value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Value {
    #[doc = "0: No action on data toggle when writing the register with this value"]
    Nop = 0,
    #[doc = "1: Data toggle is DATA0 on endpoint set by EP and IO"]
    Data0 = 1,
    #[doc = "2: Data toggle is DATA1 on endpoint set by EP and IO"]
    Data1 = 2,
}
impl From<Value> for u8 {
    #[inline(always)]
    fn from(variant: Value) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Value {
    type Ux = u8;
}
impl crate::IsEnum for Value {}
#[doc = "Field `VALUE` reader - Data toggle value"]
pub type ValueR = crate::FieldReader<Value>;
impl ValueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Value> {
        match self.bits {
            0 => Some(Value::Nop),
            1 => Some(Value::Data0),
            2 => Some(Value::Data1),
            _ => None,
        }
    }
    #[doc = "No action on data toggle when writing the register with this value"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == Value::Nop
    }
    #[doc = "Data toggle is DATA0 on endpoint set by EP and IO"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == Value::Data0
    }
    #[doc = "Data toggle is DATA1 on endpoint set by EP and IO"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == Value::Data1
    }
}
#[doc = "Field `VALUE` writer - Data toggle value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 2, Value>;
impl<'a, REG> ValueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on data toggle when writing the register with this value"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(Value::Nop)
    }
    #[doc = "Data toggle is DATA0 on endpoint set by EP and IO"]
    #[inline(always)]
    pub fn data0(self) -> &'a mut crate::W<REG> {
        self.variant(Value::Data0)
    }
    #[doc = "Data toggle is DATA1 on endpoint set by EP and IO"]
    #[inline(always)]
    pub fn data1(self) -> &'a mut crate::W<REG> {
        self.variant(Value::Data1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Select bulk endpoint number"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn io(&self) -> IoR {
        IoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data toggle value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select bulk endpoint number"]
    #[inline(always)]
    pub fn ep(&mut self) -> EpW<'_, DtoggleSpec> {
        EpW::new(self, 0)
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn io(&mut self) -> IoW<'_, DtoggleSpec> {
        IoW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Data toggle value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, DtoggleSpec> {
        ValueW::new(self, 8)
    }
}
#[doc = "Data toggle control and status\n\nYou can [`read`](crate::Reg::read) this register and get [`dtoggle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtoggle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtoggleSpec;
impl crate::RegisterSpec for DtoggleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtoggle::R`](R) reader structure"]
impl crate::Readable for DtoggleSpec {}
#[doc = "`write(|w| ..)` method takes [`dtoggle::W`](W) writer structure"]
impl crate::Writable for DtoggleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTOGGLE to value 0x0100"]
impl crate::Resettable for DtoggleSpec {
    const RESET_VALUE: u32 = 0x0100;
}
