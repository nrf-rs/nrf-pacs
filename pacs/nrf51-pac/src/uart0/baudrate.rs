#[doc = "Register `BAUDRATE` reader"]
pub type R = crate::R<BaudrateSpec>;
#[doc = "Register `BAUDRATE` writer"]
pub type W = crate::W<BaudrateSpec>;
#[doc = "UART baudrate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Baudrate {
    #[doc = "323584: 1200 baud."]
    Baud1200 = 323584,
    #[doc = "643072: 2400 baud."]
    Baud2400 = 643072,
    #[doc = "1290240: 4800 baud."]
    Baud4800 = 1290240,
    #[doc = "2576384: 9600 baud."]
    Baud9600 = 2576384,
    #[doc = "3866624: 14400 baud."]
    Baud14400 = 3866624,
    #[doc = "5152768: 19200 baud."]
    Baud19200 = 5152768,
    #[doc = "7729152: 28800 baud."]
    Baud28800 = 7729152,
    #[doc = "8388608: 31250 baud."]
    Baud31250 = 8388608,
    #[doc = "10309632: 38400 baud."]
    Baud38400 = 10309632,
    #[doc = "15007744: 56000 baud."]
    Baud56000 = 15007744,
    #[doc = "15462400: 57600 baud."]
    Baud57600 = 15462400,
    #[doc = "20615168: 76800 baud."]
    Baud76800 = 20615168,
    #[doc = "30924800: 115200 baud."]
    Baud115200 = 30924800,
    #[doc = "61845504: 230400 baud."]
    Baud230400 = 61845504,
    #[doc = "67108864: 250000 baud."]
    Baud250000 = 67108864,
    #[doc = "123695104: 460800 baud."]
    Baud460800 = 123695104,
    #[doc = "247386112: 921600 baud."]
    Baud921600 = 247386112,
    #[doc = "268435456: 1M baud."]
    Baud1m = 268435456,
}
impl From<Baudrate> for u32 {
    #[inline(always)]
    fn from(variant: Baudrate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Baudrate {
    type Ux = u32;
}
impl crate::IsEnum for Baudrate {}
#[doc = "Field `BAUDRATE` reader - UART baudrate."]
pub type BaudrateR = crate::FieldReader<Baudrate>;
impl BaudrateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Baudrate> {
        match self.bits {
            323584 => Some(Baudrate::Baud1200),
            643072 => Some(Baudrate::Baud2400),
            1290240 => Some(Baudrate::Baud4800),
            2576384 => Some(Baudrate::Baud9600),
            3866624 => Some(Baudrate::Baud14400),
            5152768 => Some(Baudrate::Baud19200),
            7729152 => Some(Baudrate::Baud28800),
            8388608 => Some(Baudrate::Baud31250),
            10309632 => Some(Baudrate::Baud38400),
            15007744 => Some(Baudrate::Baud56000),
            15462400 => Some(Baudrate::Baud57600),
            20615168 => Some(Baudrate::Baud76800),
            30924800 => Some(Baudrate::Baud115200),
            61845504 => Some(Baudrate::Baud230400),
            67108864 => Some(Baudrate::Baud250000),
            123695104 => Some(Baudrate::Baud460800),
            247386112 => Some(Baudrate::Baud921600),
            268435456 => Some(Baudrate::Baud1m),
            _ => None,
        }
    }
    #[doc = "1200 baud."]
    #[inline(always)]
    pub fn is_baud1200(&self) -> bool {
        *self == Baudrate::Baud1200
    }
    #[doc = "2400 baud."]
    #[inline(always)]
    pub fn is_baud2400(&self) -> bool {
        *self == Baudrate::Baud2400
    }
    #[doc = "4800 baud."]
    #[inline(always)]
    pub fn is_baud4800(&self) -> bool {
        *self == Baudrate::Baud4800
    }
    #[doc = "9600 baud."]
    #[inline(always)]
    pub fn is_baud9600(&self) -> bool {
        *self == Baudrate::Baud9600
    }
    #[doc = "14400 baud."]
    #[inline(always)]
    pub fn is_baud14400(&self) -> bool {
        *self == Baudrate::Baud14400
    }
    #[doc = "19200 baud."]
    #[inline(always)]
    pub fn is_baud19200(&self) -> bool {
        *self == Baudrate::Baud19200
    }
    #[doc = "28800 baud."]
    #[inline(always)]
    pub fn is_baud28800(&self) -> bool {
        *self == Baudrate::Baud28800
    }
    #[doc = "31250 baud."]
    #[inline(always)]
    pub fn is_baud31250(&self) -> bool {
        *self == Baudrate::Baud31250
    }
    #[doc = "38400 baud."]
    #[inline(always)]
    pub fn is_baud38400(&self) -> bool {
        *self == Baudrate::Baud38400
    }
    #[doc = "56000 baud."]
    #[inline(always)]
    pub fn is_baud56000(&self) -> bool {
        *self == Baudrate::Baud56000
    }
    #[doc = "57600 baud."]
    #[inline(always)]
    pub fn is_baud57600(&self) -> bool {
        *self == Baudrate::Baud57600
    }
    #[doc = "76800 baud."]
    #[inline(always)]
    pub fn is_baud76800(&self) -> bool {
        *self == Baudrate::Baud76800
    }
    #[doc = "115200 baud."]
    #[inline(always)]
    pub fn is_baud115200(&self) -> bool {
        *self == Baudrate::Baud115200
    }
    #[doc = "230400 baud."]
    #[inline(always)]
    pub fn is_baud230400(&self) -> bool {
        *self == Baudrate::Baud230400
    }
    #[doc = "250000 baud."]
    #[inline(always)]
    pub fn is_baud250000(&self) -> bool {
        *self == Baudrate::Baud250000
    }
    #[doc = "460800 baud."]
    #[inline(always)]
    pub fn is_baud460800(&self) -> bool {
        *self == Baudrate::Baud460800
    }
    #[doc = "921600 baud."]
    #[inline(always)]
    pub fn is_baud921600(&self) -> bool {
        *self == Baudrate::Baud921600
    }
    #[doc = "1M baud."]
    #[inline(always)]
    pub fn is_baud1m(&self) -> bool {
        *self == Baudrate::Baud1m
    }
}
#[doc = "Field `BAUDRATE` writer - UART baudrate."]
pub type BaudrateW<'a, REG> = crate::FieldWriter<'a, REG, 32, Baudrate>;
impl<'a, REG> BaudrateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "1200 baud."]
    #[inline(always)]
    pub fn baud1200(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud1200)
    }
    #[doc = "2400 baud."]
    #[inline(always)]
    pub fn baud2400(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud2400)
    }
    #[doc = "4800 baud."]
    #[inline(always)]
    pub fn baud4800(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud4800)
    }
    #[doc = "9600 baud."]
    #[inline(always)]
    pub fn baud9600(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud9600)
    }
    #[doc = "14400 baud."]
    #[inline(always)]
    pub fn baud14400(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud14400)
    }
    #[doc = "19200 baud."]
    #[inline(always)]
    pub fn baud19200(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud19200)
    }
    #[doc = "28800 baud."]
    #[inline(always)]
    pub fn baud28800(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud28800)
    }
    #[doc = "31250 baud."]
    #[inline(always)]
    pub fn baud31250(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud31250)
    }
    #[doc = "38400 baud."]
    #[inline(always)]
    pub fn baud38400(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud38400)
    }
    #[doc = "56000 baud."]
    #[inline(always)]
    pub fn baud56000(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud56000)
    }
    #[doc = "57600 baud."]
    #[inline(always)]
    pub fn baud57600(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud57600)
    }
    #[doc = "76800 baud."]
    #[inline(always)]
    pub fn baud76800(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud76800)
    }
    #[doc = "115200 baud."]
    #[inline(always)]
    pub fn baud115200(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud115200)
    }
    #[doc = "230400 baud."]
    #[inline(always)]
    pub fn baud230400(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud230400)
    }
    #[doc = "250000 baud."]
    #[inline(always)]
    pub fn baud250000(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud250000)
    }
    #[doc = "460800 baud."]
    #[inline(always)]
    pub fn baud460800(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud460800)
    }
    #[doc = "921600 baud."]
    #[inline(always)]
    pub fn baud921600(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud921600)
    }
    #[doc = "1M baud."]
    #[inline(always)]
    pub fn baud1m(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud1m)
    }
}
impl R {
    #[doc = "Bits 0:31 - UART baudrate."]
    #[inline(always)]
    pub fn baudrate(&self) -> BaudrateR {
        BaudrateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - UART baudrate."]
    #[inline(always)]
    pub fn baudrate(&mut self) -> BaudrateW<'_, BaudrateSpec> {
        BaudrateW::new(self, 0)
    }
}
#[doc = "UART Baudrate.\n\nYou can [`read`](crate::Reg::read) this register and get [`baudrate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudrate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudrateSpec;
impl crate::RegisterSpec for BaudrateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudrate::R`](R) reader structure"]
impl crate::Readable for BaudrateSpec {}
#[doc = "`write(|w| ..)` method takes [`baudrate::W`](W) writer structure"]
impl crate::Writable for BaudrateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAUDRATE to value 0"]
impl crate::Resettable for BaudrateSpec {}
