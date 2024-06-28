#[doc = "Register `BAUDRATE` reader"]
pub type R = crate::R<BaudrateSpec>;
#[doc = "Register `BAUDRATE` writer"]
pub type W = crate::W<BaudrateSpec>;
#[doc = "Baud rate\n\nValue on reset: 67108864"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Baudrate {
    #[doc = "323584: 1200 baud (actual rate: 1205)"]
    Baud1200 = 323584,
    #[doc = "643072: 2400 baud (actual rate: 2396)"]
    Baud2400 = 643072,
    #[doc = "1290240: 4800 baud (actual rate: 4808)"]
    Baud4800 = 1290240,
    #[doc = "2576384: 9600 baud (actual rate: 9598)"]
    Baud9600 = 2576384,
    #[doc = "3862528: 14400 baud (actual rate: 14401)"]
    Baud14400 = 3862528,
    #[doc = "5152768: 19200 baud (actual rate: 19208)"]
    Baud19200 = 5152768,
    #[doc = "7716864: 28800 baud (actual rate: 28777)"]
    Baud28800 = 7716864,
    #[doc = "8388608: 31250 baud"]
    Baud31250 = 8388608,
    #[doc = "10289152: 38400 baud (actual rate: 38369)"]
    Baud38400 = 10289152,
    #[doc = "15007744: 56000 baud (actual rate: 55944)"]
    Baud56000 = 15007744,
    #[doc = "15400960: 57600 baud (actual rate: 57554)"]
    Baud57600 = 15400960,
    #[doc = "20615168: 76800 baud (actual rate: 76923)"]
    Baud76800 = 20615168,
    #[doc = "30801920: 115200 baud (actual rate: 115108)"]
    Baud115200 = 30801920,
    #[doc = "61865984: 230400 baud (actual rate: 231884)"]
    Baud230400 = 61865984,
    #[doc = "67108864: 250000 baud"]
    Baud250000 = 67108864,
    #[doc = "121634816: 460800 baud (actual rate: 457143)"]
    Baud460800 = 121634816,
    #[doc = "251658240: 921600 baud (actual rate: 941176)"]
    Baud921600 = 251658240,
    #[doc = "268435456: 1 megabaud"]
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
#[doc = "Field `BAUDRATE` reader - Baud rate"]
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
            3862528 => Some(Baudrate::Baud14400),
            5152768 => Some(Baudrate::Baud19200),
            7716864 => Some(Baudrate::Baud28800),
            8388608 => Some(Baudrate::Baud31250),
            10289152 => Some(Baudrate::Baud38400),
            15007744 => Some(Baudrate::Baud56000),
            15400960 => Some(Baudrate::Baud57600),
            20615168 => Some(Baudrate::Baud76800),
            30801920 => Some(Baudrate::Baud115200),
            61865984 => Some(Baudrate::Baud230400),
            67108864 => Some(Baudrate::Baud250000),
            121634816 => Some(Baudrate::Baud460800),
            251658240 => Some(Baudrate::Baud921600),
            268435456 => Some(Baudrate::Baud1m),
            _ => None,
        }
    }
    #[doc = "1200 baud (actual rate: 1205)"]
    #[inline(always)]
    pub fn is_baud1200(&self) -> bool {
        *self == Baudrate::Baud1200
    }
    #[doc = "2400 baud (actual rate: 2396)"]
    #[inline(always)]
    pub fn is_baud2400(&self) -> bool {
        *self == Baudrate::Baud2400
    }
    #[doc = "4800 baud (actual rate: 4808)"]
    #[inline(always)]
    pub fn is_baud4800(&self) -> bool {
        *self == Baudrate::Baud4800
    }
    #[doc = "9600 baud (actual rate: 9598)"]
    #[inline(always)]
    pub fn is_baud9600(&self) -> bool {
        *self == Baudrate::Baud9600
    }
    #[doc = "14400 baud (actual rate: 14401)"]
    #[inline(always)]
    pub fn is_baud14400(&self) -> bool {
        *self == Baudrate::Baud14400
    }
    #[doc = "19200 baud (actual rate: 19208)"]
    #[inline(always)]
    pub fn is_baud19200(&self) -> bool {
        *self == Baudrate::Baud19200
    }
    #[doc = "28800 baud (actual rate: 28777)"]
    #[inline(always)]
    pub fn is_baud28800(&self) -> bool {
        *self == Baudrate::Baud28800
    }
    #[doc = "31250 baud"]
    #[inline(always)]
    pub fn is_baud31250(&self) -> bool {
        *self == Baudrate::Baud31250
    }
    #[doc = "38400 baud (actual rate: 38369)"]
    #[inline(always)]
    pub fn is_baud38400(&self) -> bool {
        *self == Baudrate::Baud38400
    }
    #[doc = "56000 baud (actual rate: 55944)"]
    #[inline(always)]
    pub fn is_baud56000(&self) -> bool {
        *self == Baudrate::Baud56000
    }
    #[doc = "57600 baud (actual rate: 57554)"]
    #[inline(always)]
    pub fn is_baud57600(&self) -> bool {
        *self == Baudrate::Baud57600
    }
    #[doc = "76800 baud (actual rate: 76923)"]
    #[inline(always)]
    pub fn is_baud76800(&self) -> bool {
        *self == Baudrate::Baud76800
    }
    #[doc = "115200 baud (actual rate: 115108)"]
    #[inline(always)]
    pub fn is_baud115200(&self) -> bool {
        *self == Baudrate::Baud115200
    }
    #[doc = "230400 baud (actual rate: 231884)"]
    #[inline(always)]
    pub fn is_baud230400(&self) -> bool {
        *self == Baudrate::Baud230400
    }
    #[doc = "250000 baud"]
    #[inline(always)]
    pub fn is_baud250000(&self) -> bool {
        *self == Baudrate::Baud250000
    }
    #[doc = "460800 baud (actual rate: 457143)"]
    #[inline(always)]
    pub fn is_baud460800(&self) -> bool {
        *self == Baudrate::Baud460800
    }
    #[doc = "921600 baud (actual rate: 941176)"]
    #[inline(always)]
    pub fn is_baud921600(&self) -> bool {
        *self == Baudrate::Baud921600
    }
    #[doc = "1 megabaud"]
    #[inline(always)]
    pub fn is_baud1m(&self) -> bool {
        *self == Baudrate::Baud1m
    }
}
#[doc = "Field `BAUDRATE` writer - Baud rate"]
pub type BaudrateW<'a, REG> = crate::FieldWriter<'a, REG, 32, Baudrate>;
impl<'a, REG> BaudrateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "1200 baud (actual rate: 1205)"]
    #[inline(always)]
    pub fn baud1200(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud1200)
    }
    #[doc = "2400 baud (actual rate: 2396)"]
    #[inline(always)]
    pub fn baud2400(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud2400)
    }
    #[doc = "4800 baud (actual rate: 4808)"]
    #[inline(always)]
    pub fn baud4800(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud4800)
    }
    #[doc = "9600 baud (actual rate: 9598)"]
    #[inline(always)]
    pub fn baud9600(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud9600)
    }
    #[doc = "14400 baud (actual rate: 14401)"]
    #[inline(always)]
    pub fn baud14400(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud14400)
    }
    #[doc = "19200 baud (actual rate: 19208)"]
    #[inline(always)]
    pub fn baud19200(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud19200)
    }
    #[doc = "28800 baud (actual rate: 28777)"]
    #[inline(always)]
    pub fn baud28800(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud28800)
    }
    #[doc = "31250 baud"]
    #[inline(always)]
    pub fn baud31250(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud31250)
    }
    #[doc = "38400 baud (actual rate: 38369)"]
    #[inline(always)]
    pub fn baud38400(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud38400)
    }
    #[doc = "56000 baud (actual rate: 55944)"]
    #[inline(always)]
    pub fn baud56000(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud56000)
    }
    #[doc = "57600 baud (actual rate: 57554)"]
    #[inline(always)]
    pub fn baud57600(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud57600)
    }
    #[doc = "76800 baud (actual rate: 76923)"]
    #[inline(always)]
    pub fn baud76800(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud76800)
    }
    #[doc = "115200 baud (actual rate: 115108)"]
    #[inline(always)]
    pub fn baud115200(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud115200)
    }
    #[doc = "230400 baud (actual rate: 231884)"]
    #[inline(always)]
    pub fn baud230400(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud230400)
    }
    #[doc = "250000 baud"]
    #[inline(always)]
    pub fn baud250000(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud250000)
    }
    #[doc = "460800 baud (actual rate: 457143)"]
    #[inline(always)]
    pub fn baud460800(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud460800)
    }
    #[doc = "921600 baud (actual rate: 941176)"]
    #[inline(always)]
    pub fn baud921600(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud921600)
    }
    #[doc = "1 megabaud"]
    #[inline(always)]
    pub fn baud1m(self) -> &'a mut crate::W<REG> {
        self.variant(Baudrate::Baud1m)
    }
}
impl R {
    #[doc = "Bits 0:31 - Baud rate"]
    #[inline(always)]
    pub fn baudrate(&self) -> BaudrateR {
        BaudrateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Baud rate"]
    #[inline(always)]
    #[must_use]
    pub fn baudrate(&mut self) -> BaudrateW<BaudrateSpec> {
        BaudrateW::new(self, 0)
    }
}
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baudrate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudrate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudrateSpec;
impl crate::RegisterSpec for BaudrateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudrate::R`](R) reader structure"]
impl crate::Readable for BaudrateSpec {}
#[doc = "`write(|w| ..)` method takes [`baudrate::W`](W) writer structure"]
impl crate::Writable for BaudrateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BAUDRATE to value 0x0400_0000"]
impl crate::Resettable for BaudrateSpec {
    const RESET_VALUE: u32 = 0x0400_0000;
}
