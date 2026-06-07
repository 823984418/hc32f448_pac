#[doc = "Register `TXBTO` reader"]
pub type R = crate::R<TxbtoSpec>;
#[doc = "Field `TO0` reader - desc TO0"]
pub type To0R = crate::BitReader;
#[doc = "Field `TO1` reader - desc TO1"]
pub type To1R = crate::BitReader;
#[doc = "Field `TO2` reader - desc TO2"]
pub type To2R = crate::BitReader;
#[doc = "Field `TO3` reader - desc TO3"]
pub type To3R = crate::BitReader;
#[doc = "Field `TO4` reader - desc TO4"]
pub type To4R = crate::BitReader;
#[doc = "Field `TO5` reader - desc TO5"]
pub type To5R = crate::BitReader;
#[doc = "Field `TO6` reader - desc TO6"]
pub type To6R = crate::BitReader;
#[doc = "Field `TO7` reader - desc TO7"]
pub type To7R = crate::BitReader;
#[doc = "Field `TO8` reader - desc TO8"]
pub type To8R = crate::BitReader;
#[doc = "Field `TO9` reader - desc TO9"]
pub type To9R = crate::BitReader;
#[doc = "Field `TO10` reader - desc TO10"]
pub type To10R = crate::BitReader;
#[doc = "Field `TO11` reader - desc TO11"]
pub type To11R = crate::BitReader;
#[doc = "Field `TO12` reader - desc TO12"]
pub type To12R = crate::BitReader;
#[doc = "Field `TO13` reader - desc TO13"]
pub type To13R = crate::BitReader;
#[doc = "Field `TO14` reader - desc TO14"]
pub type To14R = crate::BitReader;
#[doc = "Field `TO15` reader - desc TO15"]
pub type To15R = crate::BitReader;
#[doc = "Field `TO16` reader - desc TO16"]
pub type To16R = crate::BitReader;
#[doc = "Field `TO17` reader - desc TO17"]
pub type To17R = crate::BitReader;
#[doc = "Field `TO18` reader - desc TO18"]
pub type To18R = crate::BitReader;
#[doc = "Field `TO19` reader - desc TO19"]
pub type To19R = crate::BitReader;
#[doc = "Field `TO20` reader - desc TO20"]
pub type To20R = crate::BitReader;
#[doc = "Field `TO21` reader - desc TO21"]
pub type To21R = crate::BitReader;
#[doc = "Field `TO22` reader - desc TO22"]
pub type To22R = crate::BitReader;
#[doc = "Field `TO23` reader - desc TO23"]
pub type To23R = crate::BitReader;
#[doc = "Field `TO24` reader - desc TO24"]
pub type To24R = crate::BitReader;
#[doc = "Field `TO25` reader - desc TO25"]
pub type To25R = crate::BitReader;
#[doc = "Field `TO26` reader - desc TO26"]
pub type To26R = crate::BitReader;
#[doc = "Field `TO27` reader - desc TO27"]
pub type To27R = crate::BitReader;
#[doc = "Field `TO28` reader - desc TO28"]
pub type To28R = crate::BitReader;
#[doc = "Field `TO29` reader - desc TO29"]
pub type To29R = crate::BitReader;
#[doc = "Field `TO30` reader - desc TO30"]
pub type To30R = crate::BitReader;
#[doc = "Field `TO31` reader - desc TO31"]
pub type To31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc TO0"]
    #[inline(always)]
    pub fn to0(&self) -> To0R {
        To0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TO1"]
    #[inline(always)]
    pub fn to1(&self) -> To1R {
        To1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TO2"]
    #[inline(always)]
    pub fn to2(&self) -> To2R {
        To2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TO3"]
    #[inline(always)]
    pub fn to3(&self) -> To3R {
        To3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TO4"]
    #[inline(always)]
    pub fn to4(&self) -> To4R {
        To4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TO5"]
    #[inline(always)]
    pub fn to5(&self) -> To5R {
        To5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TO6"]
    #[inline(always)]
    pub fn to6(&self) -> To6R {
        To6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TO7"]
    #[inline(always)]
    pub fn to7(&self) -> To7R {
        To7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TO8"]
    #[inline(always)]
    pub fn to8(&self) -> To8R {
        To8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc TO9"]
    #[inline(always)]
    pub fn to9(&self) -> To9R {
        To9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc TO10"]
    #[inline(always)]
    pub fn to10(&self) -> To10R {
        To10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc TO11"]
    #[inline(always)]
    pub fn to11(&self) -> To11R {
        To11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TO12"]
    #[inline(always)]
    pub fn to12(&self) -> To12R {
        To12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc TO13"]
    #[inline(always)]
    pub fn to13(&self) -> To13R {
        To13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TO14"]
    #[inline(always)]
    pub fn to14(&self) -> To14R {
        To14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TO15"]
    #[inline(always)]
    pub fn to15(&self) -> To15R {
        To15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc TO16"]
    #[inline(always)]
    pub fn to16(&self) -> To16R {
        To16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc TO17"]
    #[inline(always)]
    pub fn to17(&self) -> To17R {
        To17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc TO18"]
    #[inline(always)]
    pub fn to18(&self) -> To18R {
        To18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc TO19"]
    #[inline(always)]
    pub fn to19(&self) -> To19R {
        To19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc TO20"]
    #[inline(always)]
    pub fn to20(&self) -> To20R {
        To20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc TO21"]
    #[inline(always)]
    pub fn to21(&self) -> To21R {
        To21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc TO22"]
    #[inline(always)]
    pub fn to22(&self) -> To22R {
        To22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TO23"]
    #[inline(always)]
    pub fn to23(&self) -> To23R {
        To23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc TO24"]
    #[inline(always)]
    pub fn to24(&self) -> To24R {
        To24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc TO25"]
    #[inline(always)]
    pub fn to25(&self) -> To25R {
        To25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc TO26"]
    #[inline(always)]
    pub fn to26(&self) -> To26R {
        To26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc TO27"]
    #[inline(always)]
    pub fn to27(&self) -> To27R {
        To27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc TO28"]
    #[inline(always)]
    pub fn to28(&self) -> To28R {
        To28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc TO29"]
    #[inline(always)]
    pub fn to29(&self) -> To29R {
        To29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc TO30"]
    #[inline(always)]
    pub fn to30(&self) -> To30R {
        To30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc TO31"]
    #[inline(always)]
    pub fn to31(&self) -> To31R {
        To31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBTO")
            .field("to0", &self.to0())
            .field("to1", &self.to1())
            .field("to2", &self.to2())
            .field("to3", &self.to3())
            .field("to4", &self.to4())
            .field("to5", &self.to5())
            .field("to6", &self.to6())
            .field("to7", &self.to7())
            .field("to8", &self.to8())
            .field("to9", &self.to9())
            .field("to10", &self.to10())
            .field("to11", &self.to11())
            .field("to12", &self.to12())
            .field("to13", &self.to13())
            .field("to14", &self.to14())
            .field("to15", &self.to15())
            .field("to16", &self.to16())
            .field("to17", &self.to17())
            .field("to18", &self.to18())
            .field("to19", &self.to19())
            .field("to20", &self.to20())
            .field("to21", &self.to21())
            .field("to22", &self.to22())
            .field("to23", &self.to23())
            .field("to24", &self.to24())
            .field("to25", &self.to25())
            .field("to26", &self.to26())
            .field("to27", &self.to27())
            .field("to28", &self.to28())
            .field("to29", &self.to29())
            .field("to30", &self.to30())
            .field("to31", &self.to31())
            .finish()
    }
}
#[doc = "desc TXBTO\n\nYou can [`read`](crate::Reg::read) this register and get [`txbto::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbtoSpec;
impl crate::RegisterSpec for TxbtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbto::R`](R) reader structure"]
impl crate::Readable for TxbtoSpec {}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TxbtoSpec {}
