#[doc = "Register `SCFR` reader"]
pub type R = crate::R<ScfrSpec>;
#[doc = "Register `SCFR` writer"]
pub type W = crate::W<ScfrSpec>;
#[doc = "Field `SF0` reader - desc SF0"]
pub type Sf0R = crate::BitReader;
#[doc = "Field `SF0` writer - desc SF0"]
pub type Sf0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SF1` reader - desc SF1"]
pub type Sf1R = crate::BitReader;
#[doc = "Field `SF1` writer - desc SF1"]
pub type Sf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SF2` reader - desc SF2"]
pub type Sf2R = crate::BitReader;
#[doc = "Field `SF2` writer - desc SF2"]
pub type Sf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SF3` reader - desc SF3"]
pub type Sf3R = crate::BitReader;
#[doc = "Field `SF3` writer - desc SF3"]
pub type Sf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SF4` reader - desc SF4"]
pub type Sf4R = crate::BitReader;
#[doc = "Field `SF4` writer - desc SF4"]
pub type Sf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SF5` reader - desc SF5"]
pub type Sf5R = crate::BitReader;
#[doc = "Field `SF5` writer - desc SF5"]
pub type Sf5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SF6` reader - desc SF6"]
pub type Sf6R = crate::BitReader;
#[doc = "Field `SF6` writer - desc SF6"]
pub type Sf6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SF7` reader - desc SF7"]
pub type Sf7R = crate::BitReader;
#[doc = "Field `SF7` writer - desc SF7"]
pub type Sf7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SF0"]
    #[inline(always)]
    pub fn sf0(&self) -> Sf0R {
        Sf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SF1"]
    #[inline(always)]
    pub fn sf1(&self) -> Sf1R {
        Sf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SF2"]
    #[inline(always)]
    pub fn sf2(&self) -> Sf2R {
        Sf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SF3"]
    #[inline(always)]
    pub fn sf3(&self) -> Sf3R {
        Sf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SF4"]
    #[inline(always)]
    pub fn sf4(&self) -> Sf4R {
        Sf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc SF5"]
    #[inline(always)]
    pub fn sf5(&self) -> Sf5R {
        Sf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SF6"]
    #[inline(always)]
    pub fn sf6(&self) -> Sf6R {
        Sf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SF7"]
    #[inline(always)]
    pub fn sf7(&self) -> Sf7R {
        Sf7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCFR")
            .field("sf0", &self.sf0())
            .field("sf1", &self.sf1())
            .field("sf2", &self.sf2())
            .field("sf3", &self.sf3())
            .field("sf4", &self.sf4())
            .field("sf5", &self.sf5())
            .field("sf6", &self.sf6())
            .field("sf7", &self.sf7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc SF0"]
    #[inline(always)]
    pub fn sf0(&mut self) -> Sf0W<'_, ScfrSpec> {
        Sf0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc SF1"]
    #[inline(always)]
    pub fn sf1(&mut self) -> Sf1W<'_, ScfrSpec> {
        Sf1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc SF2"]
    #[inline(always)]
    pub fn sf2(&mut self) -> Sf2W<'_, ScfrSpec> {
        Sf2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc SF3"]
    #[inline(always)]
    pub fn sf3(&mut self) -> Sf3W<'_, ScfrSpec> {
        Sf3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc SF4"]
    #[inline(always)]
    pub fn sf4(&mut self) -> Sf4W<'_, ScfrSpec> {
        Sf4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc SF5"]
    #[inline(always)]
    pub fn sf5(&mut self) -> Sf5W<'_, ScfrSpec> {
        Sf5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc SF6"]
    #[inline(always)]
    pub fn sf6(&mut self) -> Sf6W<'_, ScfrSpec> {
        Sf6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc SF7"]
    #[inline(always)]
    pub fn sf7(&mut self) -> Sf7W<'_, ScfrSpec> {
        Sf7W::new(self, 7)
    }
}
#[doc = "desc SCFR\n\nYou can [`read`](crate::Reg::read) this register and get [`scfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScfrSpec;
impl crate::RegisterSpec for ScfrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`scfr::R`](R) reader structure"]
impl crate::Readable for ScfrSpec {}
#[doc = "`write(|w| ..)` method takes [`scfr::W`](W) writer structure"]
impl crate::Writable for ScfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCFR to value 0xff00"]
impl crate::Resettable for ScfrSpec {
    const RESET_VALUE: u16 = 0xff00;
}
