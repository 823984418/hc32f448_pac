#[doc = "Register `HCPBR` reader"]
pub type R = crate::R<HcpbrSpec>;
#[doc = "Register `HCPBR` writer"]
pub type W = crate::W<HcpbrSpec>;
#[doc = "Field `HCPB0` reader - desc HCPB0"]
pub type Hcpb0R = crate::BitReader;
#[doc = "Field `HCPB0` writer - desc HCPB0"]
pub type Hcpb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB1` reader - desc HCPB1"]
pub type Hcpb1R = crate::BitReader;
#[doc = "Field `HCPB1` writer - desc HCPB1"]
pub type Hcpb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB2` reader - desc HCPB2"]
pub type Hcpb2R = crate::BitReader;
#[doc = "Field `HCPB2` writer - desc HCPB2"]
pub type Hcpb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB3` reader - desc HCPB3"]
pub type Hcpb3R = crate::BitReader;
#[doc = "Field `HCPB3` writer - desc HCPB3"]
pub type Hcpb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB8` reader - desc HCPB8"]
pub type Hcpb8R = crate::BitReader;
#[doc = "Field `HCPB8` writer - desc HCPB8"]
pub type Hcpb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB9` reader - desc HCPB9"]
pub type Hcpb9R = crate::BitReader;
#[doc = "Field `HCPB9` writer - desc HCPB9"]
pub type Hcpb9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB16` reader - desc HCPB16"]
pub type Hcpb16R = crate::BitReader;
#[doc = "Field `HCPB16` writer - desc HCPB16"]
pub type Hcpb16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB17` reader - desc HCPB17"]
pub type Hcpb17R = crate::BitReader;
#[doc = "Field `HCPB17` writer - desc HCPB17"]
pub type Hcpb17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB18` reader - desc HCPB18"]
pub type Hcpb18R = crate::BitReader;
#[doc = "Field `HCPB18` writer - desc HCPB18"]
pub type Hcpb18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB19` reader - desc HCPB19"]
pub type Hcpb19R = crate::BitReader;
#[doc = "Field `HCPB19` writer - desc HCPB19"]
pub type Hcpb19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB24` reader - desc HCPB24"]
pub type Hcpb24R = crate::BitReader;
#[doc = "Field `HCPB24` writer - desc HCPB24"]
pub type Hcpb24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCPB25` reader - desc HCPB25"]
pub type Hcpb25R = crate::BitReader;
#[doc = "Field `HCPB25` writer - desc HCPB25"]
pub type Hcpb25W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HCPB0"]
    #[inline(always)]
    pub fn hcpb0(&self) -> Hcpb0R {
        Hcpb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCPB1"]
    #[inline(always)]
    pub fn hcpb1(&self) -> Hcpb1R {
        Hcpb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HCPB2"]
    #[inline(always)]
    pub fn hcpb2(&self) -> Hcpb2R {
        Hcpb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HCPB3"]
    #[inline(always)]
    pub fn hcpb3(&self) -> Hcpb3R {
        Hcpb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCPB8"]
    #[inline(always)]
    pub fn hcpb8(&self) -> Hcpb8R {
        Hcpb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCPB9"]
    #[inline(always)]
    pub fn hcpb9(&self) -> Hcpb9R {
        Hcpb9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HCPB16"]
    #[inline(always)]
    pub fn hcpb16(&self) -> Hcpb16R {
        Hcpb16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc HCPB17"]
    #[inline(always)]
    pub fn hcpb17(&self) -> Hcpb17R {
        Hcpb17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc HCPB18"]
    #[inline(always)]
    pub fn hcpb18(&self) -> Hcpb18R {
        Hcpb18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc HCPB19"]
    #[inline(always)]
    pub fn hcpb19(&self) -> Hcpb19R {
        Hcpb19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - desc HCPB24"]
    #[inline(always)]
    pub fn hcpb24(&self) -> Hcpb24R {
        Hcpb24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc HCPB25"]
    #[inline(always)]
    pub fn hcpb25(&self) -> Hcpb25R {
        Hcpb25R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCPBR")
            .field("hcpb0", &self.hcpb0())
            .field("hcpb1", &self.hcpb1())
            .field("hcpb2", &self.hcpb2())
            .field("hcpb3", &self.hcpb3())
            .field("hcpb8", &self.hcpb8())
            .field("hcpb9", &self.hcpb9())
            .field("hcpb16", &self.hcpb16())
            .field("hcpb17", &self.hcpb17())
            .field("hcpb18", &self.hcpb18())
            .field("hcpb19", &self.hcpb19())
            .field("hcpb24", &self.hcpb24())
            .field("hcpb25", &self.hcpb25())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc HCPB0"]
    #[inline(always)]
    pub fn hcpb0(&mut self) -> Hcpb0W<'_, HcpbrSpec> {
        Hcpb0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HCPB1"]
    #[inline(always)]
    pub fn hcpb1(&mut self) -> Hcpb1W<'_, HcpbrSpec> {
        Hcpb1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc HCPB2"]
    #[inline(always)]
    pub fn hcpb2(&mut self) -> Hcpb2W<'_, HcpbrSpec> {
        Hcpb2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc HCPB3"]
    #[inline(always)]
    pub fn hcpb3(&mut self) -> Hcpb3W<'_, HcpbrSpec> {
        Hcpb3W::new(self, 3)
    }
    #[doc = "Bit 8 - desc HCPB8"]
    #[inline(always)]
    pub fn hcpb8(&mut self) -> Hcpb8W<'_, HcpbrSpec> {
        Hcpb8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HCPB9"]
    #[inline(always)]
    pub fn hcpb9(&mut self) -> Hcpb9W<'_, HcpbrSpec> {
        Hcpb9W::new(self, 9)
    }
    #[doc = "Bit 16 - desc HCPB16"]
    #[inline(always)]
    pub fn hcpb16(&mut self) -> Hcpb16W<'_, HcpbrSpec> {
        Hcpb16W::new(self, 16)
    }
    #[doc = "Bit 17 - desc HCPB17"]
    #[inline(always)]
    pub fn hcpb17(&mut self) -> Hcpb17W<'_, HcpbrSpec> {
        Hcpb17W::new(self, 17)
    }
    #[doc = "Bit 18 - desc HCPB18"]
    #[inline(always)]
    pub fn hcpb18(&mut self) -> Hcpb18W<'_, HcpbrSpec> {
        Hcpb18W::new(self, 18)
    }
    #[doc = "Bit 19 - desc HCPB19"]
    #[inline(always)]
    pub fn hcpb19(&mut self) -> Hcpb19W<'_, HcpbrSpec> {
        Hcpb19W::new(self, 19)
    }
    #[doc = "Bit 24 - desc HCPB24"]
    #[inline(always)]
    pub fn hcpb24(&mut self) -> Hcpb24W<'_, HcpbrSpec> {
        Hcpb24W::new(self, 24)
    }
    #[doc = "Bit 25 - desc HCPB25"]
    #[inline(always)]
    pub fn hcpb25(&mut self) -> Hcpb25W<'_, HcpbrSpec> {
        Hcpb25W::new(self, 25)
    }
}
#[doc = "desc HCPBR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcpbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcpbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcpbrSpec;
impl crate::RegisterSpec for HcpbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcpbr::R`](R) reader structure"]
impl crate::Readable for HcpbrSpec {}
#[doc = "`write(|w| ..)` method takes [`hcpbr::W`](W) writer structure"]
impl crate::Writable for HcpbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCPBR to value 0"]
impl crate::Resettable for HcpbrSpec {}
