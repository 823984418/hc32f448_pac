#[doc = "Register `TXBCF` reader"]
pub type R = crate::R<TxbcfSpec>;
#[doc = "Field `CF0` reader - desc CF0"]
pub type Cf0R = crate::BitReader;
#[doc = "Field `CF1` reader - desc CF1"]
pub type Cf1R = crate::BitReader;
#[doc = "Field `CF2` reader - desc CF2"]
pub type Cf2R = crate::BitReader;
#[doc = "Field `CF3` reader - desc CF3"]
pub type Cf3R = crate::BitReader;
#[doc = "Field `CF4` reader - desc CF4"]
pub type Cf4R = crate::BitReader;
#[doc = "Field `CF5` reader - desc CF5"]
pub type Cf5R = crate::BitReader;
#[doc = "Field `CF6` reader - desc CF6"]
pub type Cf6R = crate::BitReader;
#[doc = "Field `CF7` reader - desc CF7"]
pub type Cf7R = crate::BitReader;
#[doc = "Field `CF8` reader - desc CF8"]
pub type Cf8R = crate::BitReader;
#[doc = "Field `CF9` reader - desc CF9"]
pub type Cf9R = crate::BitReader;
#[doc = "Field `CF10` reader - desc CF10"]
pub type Cf10R = crate::BitReader;
#[doc = "Field `CF11` reader - desc CF11"]
pub type Cf11R = crate::BitReader;
#[doc = "Field `CF12` reader - desc CF12"]
pub type Cf12R = crate::BitReader;
#[doc = "Field `CF13` reader - desc CF13"]
pub type Cf13R = crate::BitReader;
#[doc = "Field `CF14` reader - desc CF14"]
pub type Cf14R = crate::BitReader;
#[doc = "Field `CF15` reader - desc CF15"]
pub type Cf15R = crate::BitReader;
#[doc = "Field `CF16` reader - desc CF16"]
pub type Cf16R = crate::BitReader;
#[doc = "Field `CF17` reader - desc CF17"]
pub type Cf17R = crate::BitReader;
#[doc = "Field `CF18` reader - desc CF18"]
pub type Cf18R = crate::BitReader;
#[doc = "Field `CF19` reader - desc CF19"]
pub type Cf19R = crate::BitReader;
#[doc = "Field `CF20` reader - desc CF20"]
pub type Cf20R = crate::BitReader;
#[doc = "Field `CF21` reader - desc CF21"]
pub type Cf21R = crate::BitReader;
#[doc = "Field `CF22` reader - desc CF22"]
pub type Cf22R = crate::BitReader;
#[doc = "Field `CF23` reader - desc CF23"]
pub type Cf23R = crate::BitReader;
#[doc = "Field `CF24` reader - desc CF24"]
pub type Cf24R = crate::BitReader;
#[doc = "Field `CF25` reader - desc CF25"]
pub type Cf25R = crate::BitReader;
#[doc = "Field `CF26` reader - desc CF26"]
pub type Cf26R = crate::BitReader;
#[doc = "Field `CF27` reader - desc CF27"]
pub type Cf27R = crate::BitReader;
#[doc = "Field `CF28` reader - desc CF28"]
pub type Cf28R = crate::BitReader;
#[doc = "Field `CF29` reader - desc CF29"]
pub type Cf29R = crate::BitReader;
#[doc = "Field `CF30` reader - desc CF30"]
pub type Cf30R = crate::BitReader;
#[doc = "Field `CF31` reader - desc CF31"]
pub type Cf31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CF0"]
    #[inline(always)]
    pub fn cf0(&self) -> Cf0R {
        Cf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CF1"]
    #[inline(always)]
    pub fn cf1(&self) -> Cf1R {
        Cf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CF2"]
    #[inline(always)]
    pub fn cf2(&self) -> Cf2R {
        Cf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CF3"]
    #[inline(always)]
    pub fn cf3(&self) -> Cf3R {
        Cf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CF4"]
    #[inline(always)]
    pub fn cf4(&self) -> Cf4R {
        Cf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CF5"]
    #[inline(always)]
    pub fn cf5(&self) -> Cf5R {
        Cf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CF6"]
    #[inline(always)]
    pub fn cf6(&self) -> Cf6R {
        Cf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CF7"]
    #[inline(always)]
    pub fn cf7(&self) -> Cf7R {
        Cf7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CF8"]
    #[inline(always)]
    pub fn cf8(&self) -> Cf8R {
        Cf8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CF9"]
    #[inline(always)]
    pub fn cf9(&self) -> Cf9R {
        Cf9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CF10"]
    #[inline(always)]
    pub fn cf10(&self) -> Cf10R {
        Cf10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CF11"]
    #[inline(always)]
    pub fn cf11(&self) -> Cf11R {
        Cf11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CF12"]
    #[inline(always)]
    pub fn cf12(&self) -> Cf12R {
        Cf12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc CF13"]
    #[inline(always)]
    pub fn cf13(&self) -> Cf13R {
        Cf13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc CF14"]
    #[inline(always)]
    pub fn cf14(&self) -> Cf14R {
        Cf14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc CF15"]
    #[inline(always)]
    pub fn cf15(&self) -> Cf15R {
        Cf15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc CF16"]
    #[inline(always)]
    pub fn cf16(&self) -> Cf16R {
        Cf16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc CF17"]
    #[inline(always)]
    pub fn cf17(&self) -> Cf17R {
        Cf17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc CF18"]
    #[inline(always)]
    pub fn cf18(&self) -> Cf18R {
        Cf18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc CF19"]
    #[inline(always)]
    pub fn cf19(&self) -> Cf19R {
        Cf19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc CF20"]
    #[inline(always)]
    pub fn cf20(&self) -> Cf20R {
        Cf20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc CF21"]
    #[inline(always)]
    pub fn cf21(&self) -> Cf21R {
        Cf21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc CF22"]
    #[inline(always)]
    pub fn cf22(&self) -> Cf22R {
        Cf22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc CF23"]
    #[inline(always)]
    pub fn cf23(&self) -> Cf23R {
        Cf23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc CF24"]
    #[inline(always)]
    pub fn cf24(&self) -> Cf24R {
        Cf24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc CF25"]
    #[inline(always)]
    pub fn cf25(&self) -> Cf25R {
        Cf25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc CF26"]
    #[inline(always)]
    pub fn cf26(&self) -> Cf26R {
        Cf26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc CF27"]
    #[inline(always)]
    pub fn cf27(&self) -> Cf27R {
        Cf27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc CF28"]
    #[inline(always)]
    pub fn cf28(&self) -> Cf28R {
        Cf28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc CF29"]
    #[inline(always)]
    pub fn cf29(&self) -> Cf29R {
        Cf29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc CF30"]
    #[inline(always)]
    pub fn cf30(&self) -> Cf30R {
        Cf30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc CF31"]
    #[inline(always)]
    pub fn cf31(&self) -> Cf31R {
        Cf31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCF")
            .field("cf0", &self.cf0())
            .field("cf1", &self.cf1())
            .field("cf2", &self.cf2())
            .field("cf3", &self.cf3())
            .field("cf4", &self.cf4())
            .field("cf5", &self.cf5())
            .field("cf6", &self.cf6())
            .field("cf7", &self.cf7())
            .field("cf8", &self.cf8())
            .field("cf9", &self.cf9())
            .field("cf10", &self.cf10())
            .field("cf11", &self.cf11())
            .field("cf12", &self.cf12())
            .field("cf13", &self.cf13())
            .field("cf14", &self.cf14())
            .field("cf15", &self.cf15())
            .field("cf16", &self.cf16())
            .field("cf17", &self.cf17())
            .field("cf18", &self.cf18())
            .field("cf19", &self.cf19())
            .field("cf20", &self.cf20())
            .field("cf21", &self.cf21())
            .field("cf22", &self.cf22())
            .field("cf23", &self.cf23())
            .field("cf24", &self.cf24())
            .field("cf25", &self.cf25())
            .field("cf26", &self.cf26())
            .field("cf27", &self.cf27())
            .field("cf28", &self.cf28())
            .field("cf29", &self.cf29())
            .field("cf30", &self.cf30())
            .field("cf31", &self.cf31())
            .finish()
    }
}
#[doc = "desc TXBCF\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcfSpec;
impl crate::RegisterSpec for TxbcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcf::R`](R) reader structure"]
impl crate::Readable for TxbcfSpec {}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TxbcfSpec {}
