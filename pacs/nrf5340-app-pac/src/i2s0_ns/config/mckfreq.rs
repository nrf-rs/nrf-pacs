#[doc = "Register `MCKFREQ` reader"]
pub type R = crate::R<MckfreqSpec>;
#[doc = "Register `MCKFREQ` writer"]
pub type W = crate::W<MckfreqSpec>;
#[doc = "I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero.\n\nValue on reset: 536870912"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Mckfreq {
    #[doc = "2147483648: 32 MHz / 2 = 16.0 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv2 = 2147483648,
    #[doc = "1342177280: 32 MHz / 3 = 10.6666667 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv3 = 1342177280,
    #[doc = "1073741824: 32 MHz / 4 = 8.0 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv4 = 1073741824,
    #[doc = "805306368: 32 MHz / 5 = 6.4 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv5 = 805306368,
    #[doc = "671088640: 32 MHz / 6 = 5.3333333 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv6 = 671088640,
    #[doc = "536870912: 32 MHz / 8 = 4.0 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv8 = 536870912,
    #[doc = "402653184: 32 MHz / 10 = 3.2 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv10 = 402653184,
    #[doc = "369098752: 32 MHz / 11 = 2.9090909 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv11 = 369098752,
    #[doc = "285212672: 32 MHz / 15 = 2.1333333 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv15 = 285212672,
    #[doc = "268435456: 32 MHz / 16 = 2.0 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv16 = 268435456,
    #[doc = "201326592: 32 MHz / 21 = 1.5238095 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv21 = 201326592,
    #[doc = "184549376: 32 MHz / 23 = 1.3913043 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv23 = 184549376,
    #[doc = "142606336: 32 MHz / 30 = 1.0666667 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv30 = 142606336,
    #[doc = "138412032: 32 MHz / 31 = 1.0322581 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv31 = 138412032,
    #[doc = "134217728: 32 MHz / 32 = 1.0 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv32 = 134217728,
    #[doc = "100663296: 32 MHz / 42 = 0.7619048 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv42 = 100663296,
    #[doc = "68157440: 32 MHz / 63 = 0.5079365 MHz Deprecated, use MCKFREQ equation."]
    _32mdiv63 = 68157440,
    #[doc = "34340864: 32 MHz / 125 = 0.256 MHz Deprecated, use MCKFREQ equation."]
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
#[doc = "Field `MCKFREQ` reader - I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero."]
pub type MckfreqR = crate::FieldReader<Mckfreq>;
impl MckfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mckfreq> {
        match self.bits {
            2147483648 => Some(Mckfreq::_32mdiv2),
            1342177280 => Some(Mckfreq::_32mdiv3),
            1073741824 => Some(Mckfreq::_32mdiv4),
            805306368 => Some(Mckfreq::_32mdiv5),
            671088640 => Some(Mckfreq::_32mdiv6),
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
    #[doc = "32 MHz / 2 = 16.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv2(&self) -> bool {
        *self == Mckfreq::_32mdiv2
    }
    #[doc = "32 MHz / 3 = 10.6666667 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv3(&self) -> bool {
        *self == Mckfreq::_32mdiv3
    }
    #[doc = "32 MHz / 4 = 8.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv4(&self) -> bool {
        *self == Mckfreq::_32mdiv4
    }
    #[doc = "32 MHz / 5 = 6.4 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv5(&self) -> bool {
        *self == Mckfreq::_32mdiv5
    }
    #[doc = "32 MHz / 6 = 5.3333333 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv6(&self) -> bool {
        *self == Mckfreq::_32mdiv6
    }
    #[doc = "32 MHz / 8 = 4.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv8(&self) -> bool {
        *self == Mckfreq::_32mdiv8
    }
    #[doc = "32 MHz / 10 = 3.2 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv10(&self) -> bool {
        *self == Mckfreq::_32mdiv10
    }
    #[doc = "32 MHz / 11 = 2.9090909 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv11(&self) -> bool {
        *self == Mckfreq::_32mdiv11
    }
    #[doc = "32 MHz / 15 = 2.1333333 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv15(&self) -> bool {
        *self == Mckfreq::_32mdiv15
    }
    #[doc = "32 MHz / 16 = 2.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv16(&self) -> bool {
        *self == Mckfreq::_32mdiv16
    }
    #[doc = "32 MHz / 21 = 1.5238095 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv21(&self) -> bool {
        *self == Mckfreq::_32mdiv21
    }
    #[doc = "32 MHz / 23 = 1.3913043 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv23(&self) -> bool {
        *self == Mckfreq::_32mdiv23
    }
    #[doc = "32 MHz / 30 = 1.0666667 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv30(&self) -> bool {
        *self == Mckfreq::_32mdiv30
    }
    #[doc = "32 MHz / 31 = 1.0322581 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv31(&self) -> bool {
        *self == Mckfreq::_32mdiv31
    }
    #[doc = "32 MHz / 32 = 1.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv32(&self) -> bool {
        *self == Mckfreq::_32mdiv32
    }
    #[doc = "32 MHz / 42 = 0.7619048 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv42(&self) -> bool {
        *self == Mckfreq::_32mdiv42
    }
    #[doc = "32 MHz / 63 = 0.5079365 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv63(&self) -> bool {
        *self == Mckfreq::_32mdiv63
    }
    #[doc = "32 MHz / 125 = 0.256 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn is_32mdiv125(&self) -> bool {
        *self == Mckfreq::_32mdiv125
    }
}
#[doc = "Field `MCKFREQ` writer - I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero."]
pub type MckfreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, Mckfreq>;
impl<'a, REG> MckfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "32 MHz / 2 = 16.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv2)
    }
    #[doc = "32 MHz / 3 = 10.6666667 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv3(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv3)
    }
    #[doc = "32 MHz / 4 = 8.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv4(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv4)
    }
    #[doc = "32 MHz / 5 = 6.4 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv5(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv5)
    }
    #[doc = "32 MHz / 6 = 5.3333333 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv6(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv6)
    }
    #[doc = "32 MHz / 8 = 4.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv8(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv8)
    }
    #[doc = "32 MHz / 10 = 3.2 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv10(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv10)
    }
    #[doc = "32 MHz / 11 = 2.9090909 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv11(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv11)
    }
    #[doc = "32 MHz / 15 = 2.1333333 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv15(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv15)
    }
    #[doc = "32 MHz / 16 = 2.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv16(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv16)
    }
    #[doc = "32 MHz / 21 = 1.5238095 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv21(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv21)
    }
    #[doc = "32 MHz / 23 = 1.3913043 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv23(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv23)
    }
    #[doc = "32 MHz / 30 = 1.0666667 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv30(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv30)
    }
    #[doc = "32 MHz / 31 = 1.0322581 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv31(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv31)
    }
    #[doc = "32 MHz / 32 = 1.0 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv32(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv32)
    }
    #[doc = "32 MHz / 42 = 0.7619048 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv42(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv42)
    }
    #[doc = "32 MHz / 63 = 0.5079365 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv63(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv63)
    }
    #[doc = "32 MHz / 125 = 0.256 MHz Deprecated, use MCKFREQ equation."]
    #[inline(always)]
    pub fn _32mdiv125(self) -> &'a mut crate::W<REG> {
        self.variant(Mckfreq::_32mdiv125)
    }
}
impl R {
    #[doc = "Bits 0:31 - I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero."]
    #[inline(always)]
    pub fn mckfreq(&self) -> MckfreqR {
        MckfreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - I2S MCK frequency configuration NOTE: Enumerations are deprecated, use MCKFREQ equation. NOTE: The 12 least significant bits of the register are ignored and shall be set to zero."]
    #[inline(always)]
    pub fn mckfreq(&mut self) -> MckfreqW<'_, MckfreqSpec> {
        MckfreqW::new(self, 0)
    }
}
#[doc = "I2S clock generator control\n\nYou can [`read`](crate::Reg::read) this register and get [`mckfreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mckfreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MckfreqSpec;
impl crate::RegisterSpec for MckfreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mckfreq::R`](R) reader structure"]
impl crate::Readable for MckfreqSpec {}
#[doc = "`write(|w| ..)` method takes [`mckfreq::W`](W) writer structure"]
impl crate::Writable for MckfreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCKFREQ to value 0x2000_0000"]
impl crate::Resettable for MckfreqSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
