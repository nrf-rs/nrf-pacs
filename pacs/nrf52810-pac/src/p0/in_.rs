#[doc = "Register `IN` reader"]
pub type R = crate::R<InSpec>;
#[doc = "Pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin0 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin0> for bool {
    #[inline(always)]
    fn from(variant: Pin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN0` reader - Pin 0"]
pub type Pin0R = crate::BitReader<Pin0>;
impl Pin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin0 {
        match self.bits {
            false => Pin0::Low,
            true => Pin0::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin0::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin0::High
    }
}
#[doc = "Pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin1 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin1> for bool {
    #[inline(always)]
    fn from(variant: Pin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN1` reader - Pin 1"]
pub type Pin1R = crate::BitReader<Pin1>;
impl Pin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin1 {
        match self.bits {
            false => Pin1::Low,
            true => Pin1::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin1::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin1::High
    }
}
#[doc = "Pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin2 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin2> for bool {
    #[inline(always)]
    fn from(variant: Pin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN2` reader - Pin 2"]
pub type Pin2R = crate::BitReader<Pin2>;
impl Pin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin2 {
        match self.bits {
            false => Pin2::Low,
            true => Pin2::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin2::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin2::High
    }
}
#[doc = "Pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin3 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin3> for bool {
    #[inline(always)]
    fn from(variant: Pin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN3` reader - Pin 3"]
pub type Pin3R = crate::BitReader<Pin3>;
impl Pin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin3 {
        match self.bits {
            false => Pin3::Low,
            true => Pin3::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin3::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin3::High
    }
}
#[doc = "Pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin4 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin4> for bool {
    #[inline(always)]
    fn from(variant: Pin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN4` reader - Pin 4"]
pub type Pin4R = crate::BitReader<Pin4>;
impl Pin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin4 {
        match self.bits {
            false => Pin4::Low,
            true => Pin4::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin4::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin4::High
    }
}
#[doc = "Pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin5 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin5> for bool {
    #[inline(always)]
    fn from(variant: Pin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN5` reader - Pin 5"]
pub type Pin5R = crate::BitReader<Pin5>;
impl Pin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin5 {
        match self.bits {
            false => Pin5::Low,
            true => Pin5::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin5::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin5::High
    }
}
#[doc = "Pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin6 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin6> for bool {
    #[inline(always)]
    fn from(variant: Pin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN6` reader - Pin 6"]
pub type Pin6R = crate::BitReader<Pin6>;
impl Pin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin6 {
        match self.bits {
            false => Pin6::Low,
            true => Pin6::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin6::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin6::High
    }
}
#[doc = "Pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin7 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin7> for bool {
    #[inline(always)]
    fn from(variant: Pin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN7` reader - Pin 7"]
pub type Pin7R = crate::BitReader<Pin7>;
impl Pin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin7 {
        match self.bits {
            false => Pin7::Low,
            true => Pin7::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin7::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin7::High
    }
}
#[doc = "Pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin8 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin8> for bool {
    #[inline(always)]
    fn from(variant: Pin8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN8` reader - Pin 8"]
pub type Pin8R = crate::BitReader<Pin8>;
impl Pin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin8 {
        match self.bits {
            false => Pin8::Low,
            true => Pin8::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin8::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin8::High
    }
}
#[doc = "Pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin9 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin9> for bool {
    #[inline(always)]
    fn from(variant: Pin9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN9` reader - Pin 9"]
pub type Pin9R = crate::BitReader<Pin9>;
impl Pin9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin9 {
        match self.bits {
            false => Pin9::Low,
            true => Pin9::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin9::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin9::High
    }
}
#[doc = "Pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin10 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin10> for bool {
    #[inline(always)]
    fn from(variant: Pin10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN10` reader - Pin 10"]
pub type Pin10R = crate::BitReader<Pin10>;
impl Pin10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin10 {
        match self.bits {
            false => Pin10::Low,
            true => Pin10::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin10::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin10::High
    }
}
#[doc = "Pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin11 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin11> for bool {
    #[inline(always)]
    fn from(variant: Pin11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN11` reader - Pin 11"]
pub type Pin11R = crate::BitReader<Pin11>;
impl Pin11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin11 {
        match self.bits {
            false => Pin11::Low,
            true => Pin11::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin11::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin11::High
    }
}
#[doc = "Pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin12 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin12> for bool {
    #[inline(always)]
    fn from(variant: Pin12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN12` reader - Pin 12"]
pub type Pin12R = crate::BitReader<Pin12>;
impl Pin12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin12 {
        match self.bits {
            false => Pin12::Low,
            true => Pin12::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin12::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin12::High
    }
}
#[doc = "Pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin13 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin13> for bool {
    #[inline(always)]
    fn from(variant: Pin13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN13` reader - Pin 13"]
pub type Pin13R = crate::BitReader<Pin13>;
impl Pin13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin13 {
        match self.bits {
            false => Pin13::Low,
            true => Pin13::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin13::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin13::High
    }
}
#[doc = "Pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin14 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin14> for bool {
    #[inline(always)]
    fn from(variant: Pin14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN14` reader - Pin 14"]
pub type Pin14R = crate::BitReader<Pin14>;
impl Pin14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin14 {
        match self.bits {
            false => Pin14::Low,
            true => Pin14::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin14::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin14::High
    }
}
#[doc = "Pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin15 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin15> for bool {
    #[inline(always)]
    fn from(variant: Pin15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN15` reader - Pin 15"]
pub type Pin15R = crate::BitReader<Pin15>;
impl Pin15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin15 {
        match self.bits {
            false => Pin15::Low,
            true => Pin15::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin15::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin15::High
    }
}
#[doc = "Pin 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin16 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin16> for bool {
    #[inline(always)]
    fn from(variant: Pin16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN16` reader - Pin 16"]
pub type Pin16R = crate::BitReader<Pin16>;
impl Pin16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin16 {
        match self.bits {
            false => Pin16::Low,
            true => Pin16::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin16::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin16::High
    }
}
#[doc = "Pin 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin17 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin17> for bool {
    #[inline(always)]
    fn from(variant: Pin17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN17` reader - Pin 17"]
pub type Pin17R = crate::BitReader<Pin17>;
impl Pin17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin17 {
        match self.bits {
            false => Pin17::Low,
            true => Pin17::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin17::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin17::High
    }
}
#[doc = "Pin 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin18 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin18> for bool {
    #[inline(always)]
    fn from(variant: Pin18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN18` reader - Pin 18"]
pub type Pin18R = crate::BitReader<Pin18>;
impl Pin18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin18 {
        match self.bits {
            false => Pin18::Low,
            true => Pin18::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin18::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin18::High
    }
}
#[doc = "Pin 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin19 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin19> for bool {
    #[inline(always)]
    fn from(variant: Pin19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN19` reader - Pin 19"]
pub type Pin19R = crate::BitReader<Pin19>;
impl Pin19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin19 {
        match self.bits {
            false => Pin19::Low,
            true => Pin19::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin19::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin19::High
    }
}
#[doc = "Pin 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin20 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin20> for bool {
    #[inline(always)]
    fn from(variant: Pin20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN20` reader - Pin 20"]
pub type Pin20R = crate::BitReader<Pin20>;
impl Pin20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin20 {
        match self.bits {
            false => Pin20::Low,
            true => Pin20::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin20::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin20::High
    }
}
#[doc = "Pin 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin21 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin21> for bool {
    #[inline(always)]
    fn from(variant: Pin21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN21` reader - Pin 21"]
pub type Pin21R = crate::BitReader<Pin21>;
impl Pin21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin21 {
        match self.bits {
            false => Pin21::Low,
            true => Pin21::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin21::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin21::High
    }
}
#[doc = "Pin 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin22 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin22> for bool {
    #[inline(always)]
    fn from(variant: Pin22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN22` reader - Pin 22"]
pub type Pin22R = crate::BitReader<Pin22>;
impl Pin22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin22 {
        match self.bits {
            false => Pin22::Low,
            true => Pin22::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin22::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin22::High
    }
}
#[doc = "Pin 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin23 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin23> for bool {
    #[inline(always)]
    fn from(variant: Pin23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN23` reader - Pin 23"]
pub type Pin23R = crate::BitReader<Pin23>;
impl Pin23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin23 {
        match self.bits {
            false => Pin23::Low,
            true => Pin23::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin23::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin23::High
    }
}
#[doc = "Pin 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin24 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin24> for bool {
    #[inline(always)]
    fn from(variant: Pin24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN24` reader - Pin 24"]
pub type Pin24R = crate::BitReader<Pin24>;
impl Pin24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin24 {
        match self.bits {
            false => Pin24::Low,
            true => Pin24::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin24::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin24::High
    }
}
#[doc = "Pin 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin25 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin25> for bool {
    #[inline(always)]
    fn from(variant: Pin25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN25` reader - Pin 25"]
pub type Pin25R = crate::BitReader<Pin25>;
impl Pin25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin25 {
        match self.bits {
            false => Pin25::Low,
            true => Pin25::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin25::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin25::High
    }
}
#[doc = "Pin 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin26 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin26> for bool {
    #[inline(always)]
    fn from(variant: Pin26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN26` reader - Pin 26"]
pub type Pin26R = crate::BitReader<Pin26>;
impl Pin26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin26 {
        match self.bits {
            false => Pin26::Low,
            true => Pin26::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin26::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin26::High
    }
}
#[doc = "Pin 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin27 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin27> for bool {
    #[inline(always)]
    fn from(variant: Pin27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN27` reader - Pin 27"]
pub type Pin27R = crate::BitReader<Pin27>;
impl Pin27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin27 {
        match self.bits {
            false => Pin27::Low,
            true => Pin27::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin27::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin27::High
    }
}
#[doc = "Pin 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin28 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin28> for bool {
    #[inline(always)]
    fn from(variant: Pin28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN28` reader - Pin 28"]
pub type Pin28R = crate::BitReader<Pin28>;
impl Pin28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin28 {
        match self.bits {
            false => Pin28::Low,
            true => Pin28::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin28::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin28::High
    }
}
#[doc = "Pin 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin29 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin29> for bool {
    #[inline(always)]
    fn from(variant: Pin29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN29` reader - Pin 29"]
pub type Pin29R = crate::BitReader<Pin29>;
impl Pin29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin29 {
        match self.bits {
            false => Pin29::Low,
            true => Pin29::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin29::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin29::High
    }
}
#[doc = "Pin 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin30 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin30> for bool {
    #[inline(always)]
    fn from(variant: Pin30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN30` reader - Pin 30"]
pub type Pin30R = crate::BitReader<Pin30>;
impl Pin30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin30 {
        match self.bits {
            false => Pin30::Low,
            true => Pin30::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin30::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin30::High
    }
}
#[doc = "Pin 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin31 {
    #[doc = "0: Pin input is low"]
    Low = 0,
    #[doc = "1: Pin input is high"]
    High = 1,
}
impl From<Pin31> for bool {
    #[inline(always)]
    fn from(variant: Pin31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN31` reader - Pin 31"]
pub type Pin31R = crate::BitReader<Pin31>;
impl Pin31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin31 {
        match self.bits {
            false => Pin31::Low,
            true => Pin31::High,
        }
    }
    #[doc = "Pin input is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pin31::Low
    }
    #[doc = "Pin input is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pin31::High
    }
}
impl R {
    #[doc = "Bit 0 - Pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pin 16"]
    #[inline(always)]
    pub fn pin16(&self) -> Pin16R {
        Pin16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pin 17"]
    #[inline(always)]
    pub fn pin17(&self) -> Pin17R {
        Pin17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pin 18"]
    #[inline(always)]
    pub fn pin18(&self) -> Pin18R {
        Pin18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pin 19"]
    #[inline(always)]
    pub fn pin19(&self) -> Pin19R {
        Pin19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pin 20"]
    #[inline(always)]
    pub fn pin20(&self) -> Pin20R {
        Pin20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pin 21"]
    #[inline(always)]
    pub fn pin21(&self) -> Pin21R {
        Pin21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pin 22"]
    #[inline(always)]
    pub fn pin22(&self) -> Pin22R {
        Pin22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Pin 23"]
    #[inline(always)]
    pub fn pin23(&self) -> Pin23R {
        Pin23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Pin 24"]
    #[inline(always)]
    pub fn pin24(&self) -> Pin24R {
        Pin24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pin 25"]
    #[inline(always)]
    pub fn pin25(&self) -> Pin25R {
        Pin25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin 26"]
    #[inline(always)]
    pub fn pin26(&self) -> Pin26R {
        Pin26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Pin 27"]
    #[inline(always)]
    pub fn pin27(&self) -> Pin27R {
        Pin27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pin 28"]
    #[inline(always)]
    pub fn pin28(&self) -> Pin28R {
        Pin28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Pin 29"]
    #[inline(always)]
    pub fn pin29(&self) -> Pin29R {
        Pin29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Pin 30"]
    #[inline(always)]
    pub fn pin30(&self) -> Pin30R {
        Pin30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Pin 31"]
    #[inline(always)]
    pub fn pin31(&self) -> Pin31R {
        Pin31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Read GPIO port\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InSpec;
impl crate::RegisterSpec for InSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for InSpec {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for InSpec {}
