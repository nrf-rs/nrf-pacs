#[doc = "Register `ITATBDATA0` reader"]
pub type R = crate::R<Itatbdata0Spec>;
#[doc = "Register `ITATBDATA0` writer"]
pub type W = crate::W<Itatbdata0Spec>;
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata0 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata0> for bool {
    #[inline(always)]
    fn from(variant: Atdata0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_0` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata0R = crate::BitReader<Atdata0>;
impl Atdata0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata0 {
        match self.bits {
            false => Atdata0::Low,
            true => Atdata0::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata0::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata0::High
    }
}
#[doc = "Field `ATDATA_0` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata0W<'a, REG> = crate::BitWriter<'a, REG, Atdata0>;
impl<'a, REG> Atdata0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata0::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata0::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata1 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata1> for bool {
    #[inline(always)]
    fn from(variant: Atdata1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_1` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata1R = crate::BitReader<Atdata1>;
impl Atdata1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata1 {
        match self.bits {
            false => Atdata1::Low,
            true => Atdata1::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata1::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata1::High
    }
}
#[doc = "Field `ATDATA_1` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata1W<'a, REG> = crate::BitWriter<'a, REG, Atdata1>;
impl<'a, REG> Atdata1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata1::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata1::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata2 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata2> for bool {
    #[inline(always)]
    fn from(variant: Atdata2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_2` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata2R = crate::BitReader<Atdata2>;
impl Atdata2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata2 {
        match self.bits {
            false => Atdata2::Low,
            true => Atdata2::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata2::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata2::High
    }
}
#[doc = "Field `ATDATA_2` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata2W<'a, REG> = crate::BitWriter<'a, REG, Atdata2>;
impl<'a, REG> Atdata2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata2::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata2::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata3 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata3> for bool {
    #[inline(always)]
    fn from(variant: Atdata3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_3` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata3R = crate::BitReader<Atdata3>;
impl Atdata3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata3 {
        match self.bits {
            false => Atdata3::Low,
            true => Atdata3::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata3::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata3::High
    }
}
#[doc = "Field `ATDATA_3` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata3W<'a, REG> = crate::BitWriter<'a, REG, Atdata3>;
impl<'a, REG> Atdata3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata3::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata3::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata4 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata4> for bool {
    #[inline(always)]
    fn from(variant: Atdata4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_4` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata4R = crate::BitReader<Atdata4>;
impl Atdata4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata4 {
        match self.bits {
            false => Atdata4::Low,
            true => Atdata4::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata4::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata4::High
    }
}
#[doc = "Field `ATDATA_4` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata4W<'a, REG> = crate::BitWriter<'a, REG, Atdata4>;
impl<'a, REG> Atdata4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata4::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata4::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata5 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata5> for bool {
    #[inline(always)]
    fn from(variant: Atdata5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_5` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata5R = crate::BitReader<Atdata5>;
impl Atdata5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata5 {
        match self.bits {
            false => Atdata5::Low,
            true => Atdata5::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata5::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata5::High
    }
}
#[doc = "Field `ATDATA_5` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata5W<'a, REG> = crate::BitWriter<'a, REG, Atdata5>;
impl<'a, REG> Atdata5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata5::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata5::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata6 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata6> for bool {
    #[inline(always)]
    fn from(variant: Atdata6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_6` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata6R = crate::BitReader<Atdata6>;
impl Atdata6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata6 {
        match self.bits {
            false => Atdata6::Low,
            true => Atdata6::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata6::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata6::High
    }
}
#[doc = "Field `ATDATA_6` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata6W<'a, REG> = crate::BitWriter<'a, REG, Atdata6>;
impl<'a, REG> Atdata6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata6::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata6::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata7 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata7> for bool {
    #[inline(always)]
    fn from(variant: Atdata7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_7` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata7R = crate::BitReader<Atdata7>;
impl Atdata7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata7 {
        match self.bits {
            false => Atdata7::Low,
            true => Atdata7::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata7::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata7::High
    }
}
#[doc = "Field `ATDATA_7` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata7W<'a, REG> = crate::BitWriter<'a, REG, Atdata7>;
impl<'a, REG> Atdata7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata7::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata7::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata8 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata8> for bool {
    #[inline(always)]
    fn from(variant: Atdata8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_8` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata8R = crate::BitReader<Atdata8>;
impl Atdata8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata8 {
        match self.bits {
            false => Atdata8::Low,
            true => Atdata8::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata8::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata8::High
    }
}
#[doc = "Field `ATDATA_8` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata8W<'a, REG> = crate::BitWriter<'a, REG, Atdata8>;
impl<'a, REG> Atdata8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata8::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata8::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata9 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata9> for bool {
    #[inline(always)]
    fn from(variant: Atdata9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_9` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata9R = crate::BitReader<Atdata9>;
impl Atdata9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata9 {
        match self.bits {
            false => Atdata9::Low,
            true => Atdata9::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata9::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata9::High
    }
}
#[doc = "Field `ATDATA_9` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata9W<'a, REG> = crate::BitWriter<'a, REG, Atdata9>;
impl<'a, REG> Atdata9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata9::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata9::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata10 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata10> for bool {
    #[inline(always)]
    fn from(variant: Atdata10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_10` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata10R = crate::BitReader<Atdata10>;
impl Atdata10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata10 {
        match self.bits {
            false => Atdata10::Low,
            true => Atdata10::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata10::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata10::High
    }
}
#[doc = "Field `ATDATA_10` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata10W<'a, REG> = crate::BitWriter<'a, REG, Atdata10>;
impl<'a, REG> Atdata10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata10::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata10::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata11 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata11> for bool {
    #[inline(always)]
    fn from(variant: Atdata11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_11` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata11R = crate::BitReader<Atdata11>;
impl Atdata11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata11 {
        match self.bits {
            false => Atdata11::Low,
            true => Atdata11::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata11::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata11::High
    }
}
#[doc = "Field `ATDATA_11` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata11W<'a, REG> = crate::BitWriter<'a, REG, Atdata11>;
impl<'a, REG> Atdata11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata11::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata11::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata12 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata12> for bool {
    #[inline(always)]
    fn from(variant: Atdata12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_12` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata12R = crate::BitReader<Atdata12>;
impl Atdata12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata12 {
        match self.bits {
            false => Atdata12::Low,
            true => Atdata12::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata12::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata12::High
    }
}
#[doc = "Field `ATDATA_12` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata12W<'a, REG> = crate::BitWriter<'a, REG, Atdata12>;
impl<'a, REG> Atdata12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata12::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata12::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata13 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata13> for bool {
    #[inline(always)]
    fn from(variant: Atdata13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_13` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata13R = crate::BitReader<Atdata13>;
impl Atdata13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata13 {
        match self.bits {
            false => Atdata13::Low,
            true => Atdata13::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata13::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata13::High
    }
}
#[doc = "Field `ATDATA_13` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata13W<'a, REG> = crate::BitWriter<'a, REG, Atdata13>;
impl<'a, REG> Atdata13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata13::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata13::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata14 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata14> for bool {
    #[inline(always)]
    fn from(variant: Atdata14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_14` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata14R = crate::BitReader<Atdata14>;
impl Atdata14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata14 {
        match self.bits {
            false => Atdata14::Low,
            true => Atdata14::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata14::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata14::High
    }
}
#[doc = "Field `ATDATA_14` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata14W<'a, REG> = crate::BitWriter<'a, REG, Atdata14>;
impl<'a, REG> Atdata14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata14::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata14::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata15 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata15> for bool {
    #[inline(always)]
    fn from(variant: Atdata15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_15` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata15R = crate::BitReader<Atdata15>;
impl Atdata15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata15 {
        match self.bits {
            false => Atdata15::Low,
            true => Atdata15::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata15::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata15::High
    }
}
#[doc = "Field `ATDATA_15` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata15W<'a, REG> = crate::BitWriter<'a, REG, Atdata15>;
impl<'a, REG> Atdata15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata15::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata15::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata16 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata16> for bool {
    #[inline(always)]
    fn from(variant: Atdata16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_16` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata16R = crate::BitReader<Atdata16>;
impl Atdata16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata16 {
        match self.bits {
            false => Atdata16::Low,
            true => Atdata16::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata16::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata16::High
    }
}
#[doc = "Field `ATDATA_16` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata16W<'a, REG> = crate::BitWriter<'a, REG, Atdata16>;
impl<'a, REG> Atdata16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata16::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata16::High)
    }
}
impl R {
    #[doc = "Bit 0 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_0(&self) -> Atdata0R {
        Atdata0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_1(&self) -> Atdata1R {
        Atdata1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_2(&self) -> Atdata2R {
        Atdata2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_3(&self) -> Atdata3R {
        Atdata3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_4(&self) -> Atdata4R {
        Atdata4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_5(&self) -> Atdata5R {
        Atdata5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_6(&self) -> Atdata6R {
        Atdata6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_7(&self) -> Atdata7R {
        Atdata7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_8(&self) -> Atdata8R {
        Atdata8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_9(&self) -> Atdata9R {
        Atdata9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_10(&self) -> Atdata10R {
        Atdata10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_11(&self) -> Atdata11R {
        Atdata11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_12(&self) -> Atdata12R {
        Atdata12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_13(&self) -> Atdata13R {
        Atdata13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_14(&self) -> Atdata14R {
        Atdata14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_15(&self) -> Atdata15R {
        Atdata15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_16(&self) -> Atdata16R {
        Atdata16R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_0(&mut self) -> Atdata0W<'_, Itatbdata0Spec> {
        Atdata0W::new(self, 0)
    }
    #[doc = "Bit 1 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_1(&mut self) -> Atdata1W<'_, Itatbdata0Spec> {
        Atdata1W::new(self, 1)
    }
    #[doc = "Bit 2 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_2(&mut self) -> Atdata2W<'_, Itatbdata0Spec> {
        Atdata2W::new(self, 2)
    }
    #[doc = "Bit 3 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_3(&mut self) -> Atdata3W<'_, Itatbdata0Spec> {
        Atdata3W::new(self, 3)
    }
    #[doc = "Bit 4 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_4(&mut self) -> Atdata4W<'_, Itatbdata0Spec> {
        Atdata4W::new(self, 4)
    }
    #[doc = "Bit 5 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_5(&mut self) -> Atdata5W<'_, Itatbdata0Spec> {
        Atdata5W::new(self, 5)
    }
    #[doc = "Bit 6 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_6(&mut self) -> Atdata6W<'_, Itatbdata0Spec> {
        Atdata6W::new(self, 6)
    }
    #[doc = "Bit 7 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_7(&mut self) -> Atdata7W<'_, Itatbdata0Spec> {
        Atdata7W::new(self, 7)
    }
    #[doc = "Bit 8 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_8(&mut self) -> Atdata8W<'_, Itatbdata0Spec> {
        Atdata8W::new(self, 8)
    }
    #[doc = "Bit 9 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_9(&mut self) -> Atdata9W<'_, Itatbdata0Spec> {
        Atdata9W::new(self, 9)
    }
    #[doc = "Bit 10 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_10(&mut self) -> Atdata10W<'_, Itatbdata0Spec> {
        Atdata10W::new(self, 10)
    }
    #[doc = "Bit 11 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_11(&mut self) -> Atdata11W<'_, Itatbdata0Spec> {
        Atdata11W::new(self, 11)
    }
    #[doc = "Bit 12 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_12(&mut self) -> Atdata12W<'_, Itatbdata0Spec> {
        Atdata12W::new(self, 12)
    }
    #[doc = "Bit 13 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_13(&mut self) -> Atdata13W<'_, Itatbdata0Spec> {
        Atdata13W::new(self, 13)
    }
    #[doc = "Bit 14 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_14(&mut self) -> Atdata14W<'_, Itatbdata0Spec> {
        Atdata14W::new(self, 14)
    }
    #[doc = "Bit 15 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_15(&mut self) -> Atdata15W<'_, Itatbdata0Spec> {
        Atdata15W::new(self, 15)
    }
    #[doc = "Bit 16 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_16(&mut self) -> Atdata16W<'_, Itatbdata0Spec> {
        Atdata16W::new(self, 16)
    }
}
#[doc = "The ITATBDATA0 register performs different functions depending on whether the access is a read or a write.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbdata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbdata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbdata0Spec;
impl crate::RegisterSpec for Itatbdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbdata0::R`](R) reader structure"]
impl crate::Readable for Itatbdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`itatbdata0::W`](W) writer structure"]
impl crate::Writable for Itatbdata0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITATBDATA0 to value 0"]
impl crate::Resettable for Itatbdata0Spec {}
