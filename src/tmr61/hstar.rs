#[doc = "Register `HSTAR` reader"]
pub type R = crate::R<HstarSpec>;
#[doc = "Register `HSTAR` writer"]
pub type W = crate::W<HstarSpec>;
#[doc = "Field `HSTA0` reader - desc HSTA0"]
pub type Hsta0R = crate::BitReader;
#[doc = "Field `HSTA0` writer - desc HSTA0"]
pub type Hsta0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA1` reader - desc HSTA1"]
pub type Hsta1R = crate::BitReader;
#[doc = "Field `HSTA1` writer - desc HSTA1"]
pub type Hsta1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA2` reader - desc HSTA2"]
pub type Hsta2R = crate::BitReader;
#[doc = "Field `HSTA2` writer - desc HSTA2"]
pub type Hsta2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA3` reader - desc HSTA3"]
pub type Hsta3R = crate::BitReader;
#[doc = "Field `HSTA3` writer - desc HSTA3"]
pub type Hsta3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAS` reader - desc STAS"]
pub type StasR = crate::BitReader;
#[doc = "Field `STAS` writer - desc STAS"]
pub type StasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA8` reader - desc HSTA8"]
pub type Hsta8R = crate::BitReader;
#[doc = "Field `HSTA8` writer - desc HSTA8"]
pub type Hsta8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA9` reader - desc HSTA9"]
pub type Hsta9R = crate::BitReader;
#[doc = "Field `HSTA9` writer - desc HSTA9"]
pub type Hsta9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA16` reader - desc HSTA16"]
pub type Hsta16R = crate::BitReader;
#[doc = "Field `HSTA16` writer - desc HSTA16"]
pub type Hsta16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA17` reader - desc HSTA17"]
pub type Hsta17R = crate::BitReader;
#[doc = "Field `HSTA17` writer - desc HSTA17"]
pub type Hsta17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA18` reader - desc HSTA18"]
pub type Hsta18R = crate::BitReader;
#[doc = "Field `HSTA18` writer - desc HSTA18"]
pub type Hsta18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA19` reader - desc HSTA19"]
pub type Hsta19R = crate::BitReader;
#[doc = "Field `HSTA19` writer - desc HSTA19"]
pub type Hsta19W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HSTA0"]
    #[inline(always)]
    pub fn hsta0(&self) -> Hsta0R {
        Hsta0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSTA1"]
    #[inline(always)]
    pub fn hsta1(&self) -> Hsta1R {
        Hsta1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HSTA2"]
    #[inline(always)]
    pub fn hsta2(&self) -> Hsta2R {
        Hsta2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HSTA3"]
    #[inline(always)]
    pub fn hsta3(&self) -> Hsta3R {
        Hsta3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - desc STAS"]
    #[inline(always)]
    pub fn stas(&self) -> StasR {
        StasR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HSTA8"]
    #[inline(always)]
    pub fn hsta8(&self) -> Hsta8R {
        Hsta8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HSTA9"]
    #[inline(always)]
    pub fn hsta9(&self) -> Hsta9R {
        Hsta9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HSTA16"]
    #[inline(always)]
    pub fn hsta16(&self) -> Hsta16R {
        Hsta16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc HSTA17"]
    #[inline(always)]
    pub fn hsta17(&self) -> Hsta17R {
        Hsta17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc HSTA18"]
    #[inline(always)]
    pub fn hsta18(&self) -> Hsta18R {
        Hsta18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc HSTA19"]
    #[inline(always)]
    pub fn hsta19(&self) -> Hsta19R {
        Hsta19R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSTAR")
            .field("hsta0", &self.hsta0())
            .field("hsta1", &self.hsta1())
            .field("hsta2", &self.hsta2())
            .field("hsta3", &self.hsta3())
            .field("stas", &self.stas())
            .field("hsta8", &self.hsta8())
            .field("hsta9", &self.hsta9())
            .field("hsta16", &self.hsta16())
            .field("hsta17", &self.hsta17())
            .field("hsta18", &self.hsta18())
            .field("hsta19", &self.hsta19())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc HSTA0"]
    #[inline(always)]
    pub fn hsta0(&mut self) -> Hsta0W<'_, HstarSpec> {
        Hsta0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HSTA1"]
    #[inline(always)]
    pub fn hsta1(&mut self) -> Hsta1W<'_, HstarSpec> {
        Hsta1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc HSTA2"]
    #[inline(always)]
    pub fn hsta2(&mut self) -> Hsta2W<'_, HstarSpec> {
        Hsta2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc HSTA3"]
    #[inline(always)]
    pub fn hsta3(&mut self) -> Hsta3W<'_, HstarSpec> {
        Hsta3W::new(self, 3)
    }
    #[doc = "Bit 7 - desc STAS"]
    #[inline(always)]
    pub fn stas(&mut self) -> StasW<'_, HstarSpec> {
        StasW::new(self, 7)
    }
    #[doc = "Bit 8 - desc HSTA8"]
    #[inline(always)]
    pub fn hsta8(&mut self) -> Hsta8W<'_, HstarSpec> {
        Hsta8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HSTA9"]
    #[inline(always)]
    pub fn hsta9(&mut self) -> Hsta9W<'_, HstarSpec> {
        Hsta9W::new(self, 9)
    }
    #[doc = "Bit 16 - desc HSTA16"]
    #[inline(always)]
    pub fn hsta16(&mut self) -> Hsta16W<'_, HstarSpec> {
        Hsta16W::new(self, 16)
    }
    #[doc = "Bit 17 - desc HSTA17"]
    #[inline(always)]
    pub fn hsta17(&mut self) -> Hsta17W<'_, HstarSpec> {
        Hsta17W::new(self, 17)
    }
    #[doc = "Bit 18 - desc HSTA18"]
    #[inline(always)]
    pub fn hsta18(&mut self) -> Hsta18W<'_, HstarSpec> {
        Hsta18W::new(self, 18)
    }
    #[doc = "Bit 19 - desc HSTA19"]
    #[inline(always)]
    pub fn hsta19(&mut self) -> Hsta19W<'_, HstarSpec> {
        Hsta19W::new(self, 19)
    }
}
#[doc = "desc HSTAR\n\nYou can [`read`](crate::Reg::read) this register and get [`hstar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstarSpec;
impl crate::RegisterSpec for HstarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstar::R`](R) reader structure"]
impl crate::Readable for HstarSpec {}
#[doc = "`write(|w| ..)` method takes [`hstar::W`](W) writer structure"]
impl crate::Writable for HstarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTAR to value 0"]
impl crate::Resettable for HstarSpec {}
