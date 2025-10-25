#[doc = "Register `INTCAP` reader"]
pub type R = crate::R<IntcapSpec>;
#[doc = "Register `INTCAP` writer"]
pub type W = crate::W<IntcapSpec>;
#[doc = "Control usage of internal load capacitors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Intcap {
    #[doc = "0: Use external load capacitors"]
    External = 0,
    #[doc = "1: 6 pF internal load capacitance"]
    C6pf = 1,
    #[doc = "2: 7 pF internal load capacitance"]
    C7pf = 2,
    #[doc = "3: 9 pF internal load capacitance"]
    C9pf = 3,
}
impl From<Intcap> for u8 {
    #[inline(always)]
    fn from(variant: Intcap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Intcap {
    type Ux = u8;
}
impl crate::IsEnum for Intcap {}
#[doc = "Field `INTCAP` reader - Control usage of internal load capacitors"]
pub type IntcapR = crate::FieldReader<Intcap>;
impl IntcapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intcap {
        match self.bits {
            0 => Intcap::External,
            1 => Intcap::C6pf,
            2 => Intcap::C7pf,
            3 => Intcap::C9pf,
            _ => unreachable!(),
        }
    }
    #[doc = "Use external load capacitors"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Intcap::External
    }
    #[doc = "6 pF internal load capacitance"]
    #[inline(always)]
    pub fn is_c6pf(&self) -> bool {
        *self == Intcap::C6pf
    }
    #[doc = "7 pF internal load capacitance"]
    #[inline(always)]
    pub fn is_c7pf(&self) -> bool {
        *self == Intcap::C7pf
    }
    #[doc = "9 pF internal load capacitance"]
    #[inline(always)]
    pub fn is_c9pf(&self) -> bool {
        *self == Intcap::C9pf
    }
}
#[doc = "Field `INTCAP` writer - Control usage of internal load capacitors"]
pub type IntcapW<'a, REG> = crate::FieldWriter<'a, REG, 2, Intcap, crate::Safe>;
impl<'a, REG> IntcapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use external load capacitors"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Intcap::External)
    }
    #[doc = "6 pF internal load capacitance"]
    #[inline(always)]
    pub fn c6pf(self) -> &'a mut crate::W<REG> {
        self.variant(Intcap::C6pf)
    }
    #[doc = "7 pF internal load capacitance"]
    #[inline(always)]
    pub fn c7pf(self) -> &'a mut crate::W<REG> {
        self.variant(Intcap::C7pf)
    }
    #[doc = "9 pF internal load capacitance"]
    #[inline(always)]
    pub fn c9pf(self) -> &'a mut crate::W<REG> {
        self.variant(Intcap::C9pf)
    }
}
impl R {
    #[doc = "Bits 0:1 - Control usage of internal load capacitors"]
    #[inline(always)]
    pub fn intcap(&self) -> IntcapR {
        IntcapR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control usage of internal load capacitors"]
    #[inline(always)]
    pub fn intcap(&mut self) -> IntcapW<'_, IntcapSpec> {
        IntcapW::new(self, 0)
    }
}
#[doc = "Control usage of internal load capacitors\n\nYou can [`read`](crate::Reg::read) this register and get [`intcap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intcap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcapSpec;
impl crate::RegisterSpec for IntcapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intcap::R`](R) reader structure"]
impl crate::Readable for IntcapSpec {}
#[doc = "`write(|w| ..)` method takes [`intcap::W`](W) writer structure"]
impl crate::Writable for IntcapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTCAP to value 0"]
impl crate::Resettable for IntcapSpec {}
