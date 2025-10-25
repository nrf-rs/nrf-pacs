#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive0 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive0> for bool {
    #[inline(always)]
    fn from(variant: Receive0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE0` reader - Read pending status of interrupt for event RECEIVE\\[0\\]"]
pub type Receive0R = crate::BitReader<Receive0>;
impl Receive0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive0 {
        match self.bits {
            false => Receive0::NotPending,
            true => Receive0::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive0::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive0::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive1 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive1> for bool {
    #[inline(always)]
    fn from(variant: Receive1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE1` reader - Read pending status of interrupt for event RECEIVE\\[1\\]"]
pub type Receive1R = crate::BitReader<Receive1>;
impl Receive1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive1 {
        match self.bits {
            false => Receive1::NotPending,
            true => Receive1::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive1::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive1::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive2 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive2> for bool {
    #[inline(always)]
    fn from(variant: Receive2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE2` reader - Read pending status of interrupt for event RECEIVE\\[2\\]"]
pub type Receive2R = crate::BitReader<Receive2>;
impl Receive2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive2 {
        match self.bits {
            false => Receive2::NotPending,
            true => Receive2::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive2::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive2::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive3 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive3> for bool {
    #[inline(always)]
    fn from(variant: Receive3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE3` reader - Read pending status of interrupt for event RECEIVE\\[3\\]"]
pub type Receive3R = crate::BitReader<Receive3>;
impl Receive3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive3 {
        match self.bits {
            false => Receive3::NotPending,
            true => Receive3::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive3::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive3::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive4 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive4> for bool {
    #[inline(always)]
    fn from(variant: Receive4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE4` reader - Read pending status of interrupt for event RECEIVE\\[4\\]"]
pub type Receive4R = crate::BitReader<Receive4>;
impl Receive4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive4 {
        match self.bits {
            false => Receive4::NotPending,
            true => Receive4::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive4::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive4::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive5 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive5> for bool {
    #[inline(always)]
    fn from(variant: Receive5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE5` reader - Read pending status of interrupt for event RECEIVE\\[5\\]"]
pub type Receive5R = crate::BitReader<Receive5>;
impl Receive5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive5 {
        match self.bits {
            false => Receive5::NotPending,
            true => Receive5::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive5::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive5::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive6 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive6> for bool {
    #[inline(always)]
    fn from(variant: Receive6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE6` reader - Read pending status of interrupt for event RECEIVE\\[6\\]"]
pub type Receive6R = crate::BitReader<Receive6>;
impl Receive6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive6 {
        match self.bits {
            false => Receive6::NotPending,
            true => Receive6::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive6::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive6::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive7 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive7> for bool {
    #[inline(always)]
    fn from(variant: Receive7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE7` reader - Read pending status of interrupt for event RECEIVE\\[7\\]"]
pub type Receive7R = crate::BitReader<Receive7>;
impl Receive7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive7 {
        match self.bits {
            false => Receive7::NotPending,
            true => Receive7::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive7::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive7::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive8 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive8> for bool {
    #[inline(always)]
    fn from(variant: Receive8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE8` reader - Read pending status of interrupt for event RECEIVE\\[8\\]"]
pub type Receive8R = crate::BitReader<Receive8>;
impl Receive8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive8 {
        match self.bits {
            false => Receive8::NotPending,
            true => Receive8::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive8::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive8::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive9 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive9> for bool {
    #[inline(always)]
    fn from(variant: Receive9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE9` reader - Read pending status of interrupt for event RECEIVE\\[9\\]"]
pub type Receive9R = crate::BitReader<Receive9>;
impl Receive9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive9 {
        match self.bits {
            false => Receive9::NotPending,
            true => Receive9::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive9::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive9::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive10 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive10> for bool {
    #[inline(always)]
    fn from(variant: Receive10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE10` reader - Read pending status of interrupt for event RECEIVE\\[10\\]"]
pub type Receive10R = crate::BitReader<Receive10>;
impl Receive10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive10 {
        match self.bits {
            false => Receive10::NotPending,
            true => Receive10::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive10::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive10::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive11 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive11> for bool {
    #[inline(always)]
    fn from(variant: Receive11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE11` reader - Read pending status of interrupt for event RECEIVE\\[11\\]"]
pub type Receive11R = crate::BitReader<Receive11>;
impl Receive11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive11 {
        match self.bits {
            false => Receive11::NotPending,
            true => Receive11::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive11::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive11::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[12\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive12 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive12> for bool {
    #[inline(always)]
    fn from(variant: Receive12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE12` reader - Read pending status of interrupt for event RECEIVE\\[12\\]"]
pub type Receive12R = crate::BitReader<Receive12>;
impl Receive12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive12 {
        match self.bits {
            false => Receive12::NotPending,
            true => Receive12::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive12::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive12::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[13\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive13 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive13> for bool {
    #[inline(always)]
    fn from(variant: Receive13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE13` reader - Read pending status of interrupt for event RECEIVE\\[13\\]"]
pub type Receive13R = crate::BitReader<Receive13>;
impl Receive13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive13 {
        match self.bits {
            false => Receive13::NotPending,
            true => Receive13::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive13::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive13::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[14\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive14 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive14> for bool {
    #[inline(always)]
    fn from(variant: Receive14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE14` reader - Read pending status of interrupt for event RECEIVE\\[14\\]"]
pub type Receive14R = crate::BitReader<Receive14>;
impl Receive14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive14 {
        match self.bits {
            false => Receive14::NotPending,
            true => Receive14::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive14::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive14::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[15\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive15 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive15> for bool {
    #[inline(always)]
    fn from(variant: Receive15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE15` reader - Read pending status of interrupt for event RECEIVE\\[15\\]"]
pub type Receive15R = crate::BitReader<Receive15>;
impl Receive15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive15 {
        match self.bits {
            false => Receive15::NotPending,
            true => Receive15::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive15::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive15::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&self) -> Receive0R {
        Receive0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&self) -> Receive1R {
        Receive1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&self) -> Receive2R {
        Receive2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read pending status of interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&self) -> Receive3R {
        Receive3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read pending status of interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&self) -> Receive4R {
        Receive4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read pending status of interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&self) -> Receive5R {
        Receive5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read pending status of interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&self) -> Receive6R {
        Receive6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read pending status of interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&self) -> Receive7R {
        Receive7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read pending status of interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn receive8(&self) -> Receive8R {
        Receive8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read pending status of interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn receive9(&self) -> Receive9R {
        Receive9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read pending status of interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn receive10(&self) -> Receive10R {
        Receive10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Read pending status of interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn receive11(&self) -> Receive11R {
        Receive11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Read pending status of interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn receive12(&self) -> Receive12R {
        Receive12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Read pending status of interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn receive13(&self) -> Receive13R {
        Receive13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Read pending status of interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn receive14(&self) -> Receive14R {
        Receive14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Read pending status of interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn receive15(&self) -> Receive15R {
        Receive15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Pending interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpendSpec;
impl crate::RegisterSpec for IntpendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpend::R`](R) reader structure"]
impl crate::Readable for IntpendSpec {}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for IntpendSpec {}
