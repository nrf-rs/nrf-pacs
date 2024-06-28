#[doc = "Register `MCKFREQ` reader"]
pub type R = crate::R<MckfreqSpec>;
#[doc = "Register `MCKFREQ` writer"]
pub type W = crate::W<MckfreqSpec>;
#[doc = "Master clock generator frequency.\n\nValue on reset: 536870912"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Mckfreq {
    #[doc = "536870912: 32 MHz / 8 = 4.0 MHz"]
    _32mdiv8 = 536870912,
    #[doc = "402653184: 32 MHz / 10 = 3.2 MHz"]
    _32mdiv10 = 402653184,
    #[doc = "369098752: 32 MHz / 11 = 2.9090909 MHz"]
    _32mdiv11 = 369098752,
    #[doc = "285212672: 32 MHz / 15 = 2.1333333 MHz"]
    _32mdiv15 = 285212672,
    #[doc = "268435456: 32 MHz / 16 = 2.0 MHz"]
    _32mdiv16 = 268435456,
    #[doc = "201326592: 32 MHz / 21 = 1.5238095"]
    _32mdiv21 = 201326592,
    #[doc = "184549376: 32 MHz / 23 = 1.3913043 MHz"]
    _32mdiv23 = 184549376,
    #[doc = "142606336: 32 MHz / 30 = 1.0666667 MHz"]
    _32mdiv30 = 142606336,
    #[doc = "138412032: 32 MHz / 31 = 1.0322581 MHz"]
    _32mdiv31 = 138412032,
    #[doc = "134217728: 32 MHz / 32 = 1.0 MHz"]
    _32mdiv32 = 134217728,
    #[doc = "100663296: 32 MHz / 42 = 0.7619048 MHz"]
    _32mdiv42 = 100663296,
    #[doc = "68157440: 32 MHz / 63 = 0.5079365 MHz"]
    _32mdiv63 = 68157440,
    #[doc = "34340864: 32 MHz / 125 = 0.256 MHz"]
    _32mdiv125 = 34340864,
}
impl From<Mckfreq> for u32 {
    #[inline(always)]
    fn from(variant: Mckfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mckfreq {
    type Ux = u32;
}
impl crate::IsEnum for Mckfreq {}
#[doc = "Field `MCKFREQ` reader - Master clock generator frequency."]
pub type MckfreqR = crate::FieldReader<Mckfreq>;
impl MckfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mckfreq> {
        match self.bits {
            536870912 => Some(Mckfreq::_32mdiv8),
            402653184 => Some(Mckfreq::_32mdiv10),
            369098752 => Some(Mckfreq::_32mdiv11),
            285212672 => Some(Mckfreq::_32mdiv15),
            268435456 => Some(Mckfreq::_32mdiv16),
            201326592 => Some(Mckfreq::_32mdiv21),
            184549376 => Some(Mckfreq::_32mdiv23),
            142606336 => Some(Mckfreq::_32mdiv30),
            138412032 => Some(Mckfreq::_32mdiv31),
            134217728 => Some(Mckfreq::_32mdiv32),
            100663296 => Some(Mckfreq::_32mdiv42),
            68157440 => Some(Mckfreq::_32mdiv63),
            34340864 => Some(Mckfreq::_32mdiv125),
            _ => None,
        }
    }
    #[doc = "32 MHz / 8 = 4.0 MHz"]
    #[inline(always)]
    pub fn is_32mdiv8(&self) -> bool {
        *self == Mckfreq::_32mdiv8
    }
    #[doc = "32 MHz / 10 = 3.2 MHz"]
    #[inline(always)]
    pub fn is_32mdiv10(&self) -> bool {
        *self == Mckfreq::_32mdiv10
    }
    #[doc = "32 MHz / 11 = 2.9090909 MHz"]
    #[inline(always)]
    pub fn is_32mdiv11(&self) -> bool {
        *self == Mckfreq::_32mdiv11
    }
    #[doc = "32 MHz / 15 = 2.1333333 MHz"]
    #[inline(always)]
    pub fn is_32mdiv15(&self) -> bool {
        *self == Mckfreq::_32mdiv15
    }
    #[doc = "32 MHz / 16 = 2.0 MHz"]
    #[inline(always)]
    pub fn is_32mdiv16(&self) -> bool {
        *self == Mckfreq::_32mdiv16
    }
    #[doc = "32 MHz / 21 = 1.5238095"]
    #[inline(always)]
    pub fn is_32mdiv21(&self) -> bool {
        *self == Mckfreq::_32mdiv21
    }
    #[doc = "32 MHz / 23 = 1.3913043 MHz"]
    #[inline(always)]
    pub fn is_32mdiv23(&self) -> bool {
        *self == Mckfreq::_32mdiv23
    }
    #[doc = "32 MHz / 30 = 1.0666667 MHz"]
    #[inline(always)]
    pub fn is_32mdiv30(&self) -> bool {
        *self == Mckfreq::_32mdiv30
    }
    #[doc = "32 MHz / 31 = 1.0322581 MHz"]
    #[inline(always)]
    pub fn is_32mdiv31(&self) -> bool {
        *self == Mckfreq::_32mdiv31
    }
    #[doc = "32 MHz / 32 = 1.0 MHz"]
    #[inline(always)]
    pub fn is_32mdiv32(&self) -> bool {
        *self == Mckfreq::_32mdiv32
    }
    #[doc = "32 MHz / 42 = 0.7619048 MHz"]
    #[inline(always)]
    pub fn is_32mdiv42(&self) -> bool {
        *self == Mckfreq::_32mdiv42
    }
    #[doc = "32 MHz / 63 = 0.5079365 MHz"]
    #[inline(always)]
    pub fn is_32mdiv63(&self) -> bool {
        *self == Mckfreq::_32mdiv63
    }
    #[doc = "32 MHz / 125 = 0.256 MHz"]
    #[inline(always)]
    pub fn is_32mdiv125(&self) -> bool {
        *self == Mckfreq::_32mdiv125
    }
}
#[doc = "Field `MCKFREQ` writer - Master clock generator frequency."]
pub type MckfreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, Mckfreq>;
impl<'a, REG> MckfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "32 MHz / 8 = 4.0 MHz"]
    #[inline(always)]
    pub fn _32mdiv8(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv8)
    }
    #[doc = "32 MHz / 10 = 3.2 MHz"]
    #[inline(always)]
    pub fn _32mdiv10(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv10)
    }
    #[doc = "32 MHz / 11 = 2.9090909 MHz"]
    #[inline(always)]
    pub fn _32mdiv11(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv11)
    }
    #[doc = "32 MHz / 15 = 2.1333333 MHz"]
    #[inline(always)]
    pub fn _32mdiv15(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv15)
    }
    #[doc = "32 MHz / 16 = 2.0 MHz"]
    #[inline(always)]
    pub fn _32mdiv16(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv16)
    }
    #[doc = "32 MHz / 21 = 1.5238095"]
    #[inline(always)]
    pub fn _32mdiv21(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv21)
    }
    #[doc = "32 MHz / 23 = 1.3913043 MHz"]
    #[inline(always)]
    pub fn _32mdiv23(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv23)
    }
    #[doc = "32 MHz / 30 = 1.0666667 MHz"]
    #[inline(always)]
    pub fn _32mdiv30(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv30)
    }
    #[doc = "32 MHz / 31 = 1.0322581 MHz"]
    #[inline(always)]
    pub fn _32mdiv31(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv31)
    }
    #[doc = "32 MHz / 32 = 1.0 MHz"]
    #[inline(always)]
    pub fn _32mdiv32(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv32)
    }
    #[doc = "32 MHz / 42 = 0.7619048 MHz"]
    #[inline(always)]
    pub fn _32mdiv42(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv42)
    }
    #[doc = "32 MHz / 63 = 0.5079365 MHz"]
    #[inline(always)]
    pub fn _32mdiv63(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv63)
    }
    #[doc = "32 MHz / 125 = 0.256 MHz"]
    #[inline(always)]
    pub fn _32mdiv125(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv125)
    }
}
impl R {
    #[doc = "Bits 0:31 - Master clock generator frequency."]
    #[inline(always)]
    pub fn mckfreq(&self) -> MckfreqR {
        MckfreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Master clock generator frequency."]
    #[inline(always)]
    #[must_use]
    pub fn mckfreq(&mut self) -> MckfreqW<MckfreqSpec> {
        MckfreqW::new(self, 0)
    }
}
#[doc = "Master clock generator frequency.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mckfreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mckfreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MckfreqSpec;
impl crate::RegisterSpec for MckfreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mckfreq::R`](R) reader structure"]
impl crate::Readable for MckfreqSpec {}
#[doc = "`write(|w| ..)` method takes [`mckfreq::W`](W) writer structure"]
impl crate::Writable for MckfreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCKFREQ to value 0x2000_0000"]
impl crate::Resettable for MckfreqSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
