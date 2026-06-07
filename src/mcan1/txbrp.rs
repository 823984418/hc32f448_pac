#[doc = "Register `TXBRP` reader"]
pub type R = crate::R<TxbrpSpec>;
#[doc = "Field `TRP0` reader - desc TRP0"]
pub type Trp0R = crate::BitReader;
#[doc = "Field `TRP1` reader - desc TRP1"]
pub type Trp1R = crate::BitReader;
#[doc = "Field `TRP2` reader - desc TRP2"]
pub type Trp2R = crate::BitReader;
#[doc = "Field `TRP3` reader - desc TRP3"]
pub type Trp3R = crate::BitReader;
#[doc = "Field `TRP4` reader - desc TRP4"]
pub type Trp4R = crate::BitReader;
#[doc = "Field `TRP5` reader - desc TRP5"]
pub type Trp5R = crate::BitReader;
#[doc = "Field `TRP6` reader - desc TRP6"]
pub type Trp6R = crate::BitReader;
#[doc = "Field `TRP7` reader - desc TRP7"]
pub type Trp7R = crate::BitReader;
#[doc = "Field `TRP8` reader - desc TRP8"]
pub type Trp8R = crate::BitReader;
#[doc = "Field `TRP9` reader - desc TRP9"]
pub type Trp9R = crate::BitReader;
#[doc = "Field `TRP10` reader - desc TRP10"]
pub type Trp10R = crate::BitReader;
#[doc = "Field `TRP11` reader - desc TRP11"]
pub type Trp11R = crate::BitReader;
#[doc = "Field `TRP12` reader - desc TRP12"]
pub type Trp12R = crate::BitReader;
#[doc = "Field `TRP13` reader - desc TRP13"]
pub type Trp13R = crate::BitReader;
#[doc = "Field `TRP14` reader - desc TRP14"]
pub type Trp14R = crate::BitReader;
#[doc = "Field `TRP15` reader - desc TRP15"]
pub type Trp15R = crate::BitReader;
#[doc = "Field `TRP16` reader - desc TRP16"]
pub type Trp16R = crate::BitReader;
#[doc = "Field `TRP17` reader - desc TRP17"]
pub type Trp17R = crate::BitReader;
#[doc = "Field `TRP18` reader - desc TRP18"]
pub type Trp18R = crate::BitReader;
#[doc = "Field `TRP19` reader - desc TRP19"]
pub type Trp19R = crate::BitReader;
#[doc = "Field `TRP20` reader - desc TRP20"]
pub type Trp20R = crate::BitReader;
#[doc = "Field `TRP21` reader - desc TRP21"]
pub type Trp21R = crate::BitReader;
#[doc = "Field `TRP22` reader - desc TRP22"]
pub type Trp22R = crate::BitReader;
#[doc = "Field `TRP23` reader - desc TRP23"]
pub type Trp23R = crate::BitReader;
#[doc = "Field `TRP24` reader - desc TRP24"]
pub type Trp24R = crate::BitReader;
#[doc = "Field `TRP25` reader - desc TRP25"]
pub type Trp25R = crate::BitReader;
#[doc = "Field `TRP26` reader - desc TRP26"]
pub type Trp26R = crate::BitReader;
#[doc = "Field `TRP27` reader - desc TRP27"]
pub type Trp27R = crate::BitReader;
#[doc = "Field `TRP28` reader - desc TRP28"]
pub type Trp28R = crate::BitReader;
#[doc = "Field `TRP29` reader - desc TRP29"]
pub type Trp29R = crate::BitReader;
#[doc = "Field `TRP30` reader - desc TRP30"]
pub type Trp30R = crate::BitReader;
#[doc = "Field `TRP31` reader - desc TRP31"]
pub type Trp31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc TRP0"]
    #[inline(always)]
    pub fn trp0(&self) -> Trp0R {
        Trp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TRP1"]
    #[inline(always)]
    pub fn trp1(&self) -> Trp1R {
        Trp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TRP2"]
    #[inline(always)]
    pub fn trp2(&self) -> Trp2R {
        Trp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TRP3"]
    #[inline(always)]
    pub fn trp3(&self) -> Trp3R {
        Trp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TRP4"]
    #[inline(always)]
    pub fn trp4(&self) -> Trp4R {
        Trp4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TRP5"]
    #[inline(always)]
    pub fn trp5(&self) -> Trp5R {
        Trp5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TRP6"]
    #[inline(always)]
    pub fn trp6(&self) -> Trp6R {
        Trp6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TRP7"]
    #[inline(always)]
    pub fn trp7(&self) -> Trp7R {
        Trp7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TRP8"]
    #[inline(always)]
    pub fn trp8(&self) -> Trp8R {
        Trp8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc TRP9"]
    #[inline(always)]
    pub fn trp9(&self) -> Trp9R {
        Trp9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc TRP10"]
    #[inline(always)]
    pub fn trp10(&self) -> Trp10R {
        Trp10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc TRP11"]
    #[inline(always)]
    pub fn trp11(&self) -> Trp11R {
        Trp11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TRP12"]
    #[inline(always)]
    pub fn trp12(&self) -> Trp12R {
        Trp12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc TRP13"]
    #[inline(always)]
    pub fn trp13(&self) -> Trp13R {
        Trp13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TRP14"]
    #[inline(always)]
    pub fn trp14(&self) -> Trp14R {
        Trp14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TRP15"]
    #[inline(always)]
    pub fn trp15(&self) -> Trp15R {
        Trp15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc TRP16"]
    #[inline(always)]
    pub fn trp16(&self) -> Trp16R {
        Trp16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc TRP17"]
    #[inline(always)]
    pub fn trp17(&self) -> Trp17R {
        Trp17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc TRP18"]
    #[inline(always)]
    pub fn trp18(&self) -> Trp18R {
        Trp18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc TRP19"]
    #[inline(always)]
    pub fn trp19(&self) -> Trp19R {
        Trp19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc TRP20"]
    #[inline(always)]
    pub fn trp20(&self) -> Trp20R {
        Trp20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc TRP21"]
    #[inline(always)]
    pub fn trp21(&self) -> Trp21R {
        Trp21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc TRP22"]
    #[inline(always)]
    pub fn trp22(&self) -> Trp22R {
        Trp22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TRP23"]
    #[inline(always)]
    pub fn trp23(&self) -> Trp23R {
        Trp23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc TRP24"]
    #[inline(always)]
    pub fn trp24(&self) -> Trp24R {
        Trp24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc TRP25"]
    #[inline(always)]
    pub fn trp25(&self) -> Trp25R {
        Trp25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc TRP26"]
    #[inline(always)]
    pub fn trp26(&self) -> Trp26R {
        Trp26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc TRP27"]
    #[inline(always)]
    pub fn trp27(&self) -> Trp27R {
        Trp27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc TRP28"]
    #[inline(always)]
    pub fn trp28(&self) -> Trp28R {
        Trp28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc TRP29"]
    #[inline(always)]
    pub fn trp29(&self) -> Trp29R {
        Trp29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc TRP30"]
    #[inline(always)]
    pub fn trp30(&self) -> Trp30R {
        Trp30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc TRP31"]
    #[inline(always)]
    pub fn trp31(&self) -> Trp31R {
        Trp31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBRP")
            .field("trp0", &self.trp0())
            .field("trp1", &self.trp1())
            .field("trp2", &self.trp2())
            .field("trp3", &self.trp3())
            .field("trp4", &self.trp4())
            .field("trp5", &self.trp5())
            .field("trp6", &self.trp6())
            .field("trp7", &self.trp7())
            .field("trp8", &self.trp8())
            .field("trp9", &self.trp9())
            .field("trp10", &self.trp10())
            .field("trp11", &self.trp11())
            .field("trp12", &self.trp12())
            .field("trp13", &self.trp13())
            .field("trp14", &self.trp14())
            .field("trp15", &self.trp15())
            .field("trp16", &self.trp16())
            .field("trp17", &self.trp17())
            .field("trp18", &self.trp18())
            .field("trp19", &self.trp19())
            .field("trp20", &self.trp20())
            .field("trp21", &self.trp21())
            .field("trp22", &self.trp22())
            .field("trp23", &self.trp23())
            .field("trp24", &self.trp24())
            .field("trp25", &self.trp25())
            .field("trp26", &self.trp26())
            .field("trp27", &self.trp27())
            .field("trp28", &self.trp28())
            .field("trp29", &self.trp29())
            .field("trp30", &self.trp30())
            .field("trp31", &self.trp31())
            .finish()
    }
}
#[doc = "desc TXBRP\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbrpSpec;
impl crate::RegisterSpec for TxbrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbrp::R`](R) reader structure"]
impl crate::Readable for TxbrpSpec {}
#[doc = "`reset()` method sets TXBRP to value 0"]
impl crate::Resettable for TxbrpSpec {}
