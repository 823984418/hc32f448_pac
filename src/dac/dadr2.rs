#[doc = "Register `DADR2` reader"]
pub type R = crate::R<Dadr2Spec>;
#[doc = "Register `DADR2` writer"]
pub type W = crate::W<Dadr2Spec>;
#[doc = "Field `DR0` reader - desc DR0"]
pub type Dr0R = crate::BitReader;
#[doc = "Field `DR0` writer - desc DR0"]
pub type Dr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR1` reader - desc DR1"]
pub type Dr1R = crate::BitReader;
#[doc = "Field `DR1` writer - desc DR1"]
pub type Dr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR2` reader - desc DR2"]
pub type Dr2R = crate::BitReader;
#[doc = "Field `DR2` writer - desc DR2"]
pub type Dr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR3` reader - desc DR3"]
pub type Dr3R = crate::BitReader;
#[doc = "Field `DR3` writer - desc DR3"]
pub type Dr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0R4` reader - desc DL0R4"]
pub type Dl0r4R = crate::BitReader;
#[doc = "Field `DL0R4` writer - desc DL0R4"]
pub type Dl0r4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1R5` reader - desc DL1R5"]
pub type Dl1r5R = crate::BitReader;
#[doc = "Field `DL1R5` writer - desc DL1R5"]
pub type Dl1r5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL2R6` reader - desc DL2R6"]
pub type Dl2r6R = crate::BitReader;
#[doc = "Field `DL2R6` writer - desc DL2R6"]
pub type Dl2r6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL3R7` reader - desc DL3R7"]
pub type Dl3r7R = crate::BitReader;
#[doc = "Field `DL3R7` writer - desc DL3R7"]
pub type Dl3r7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL4R8` reader - desc DL4R8"]
pub type Dl4r8R = crate::BitReader;
#[doc = "Field `DL4R8` writer - desc DL4R8"]
pub type Dl4r8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL5R9` reader - desc DL5R9"]
pub type Dl5r9R = crate::BitReader;
#[doc = "Field `DL5R9` writer - desc DL5R9"]
pub type Dl5r9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL6R10` reader - desc DL6R10"]
pub type Dl6r10R = crate::BitReader;
#[doc = "Field `DL6R10` writer - desc DL6R10"]
pub type Dl6r10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL7R11` reader - desc DL7R11"]
pub type Dl7r11R = crate::BitReader;
#[doc = "Field `DL7R11` writer - desc DL7R11"]
pub type Dl7r11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL8` reader - desc DL8"]
pub type Dl8R = crate::BitReader;
#[doc = "Field `DL8` writer - desc DL8"]
pub type Dl8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL9` reader - desc DL9"]
pub type Dl9R = crate::BitReader;
#[doc = "Field `DL9` writer - desc DL9"]
pub type Dl9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL10` reader - desc DL10"]
pub type Dl10R = crate::BitReader;
#[doc = "Field `DL10` writer - desc DL10"]
pub type Dl10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL11` reader - desc DL11"]
pub type Dl11R = crate::BitReader;
#[doc = "Field `DL11` writer - desc DL11"]
pub type Dl11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc DR0"]
    #[inline(always)]
    pub fn dr0(&self) -> Dr0R {
        Dr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc DR1"]
    #[inline(always)]
    pub fn dr1(&self) -> Dr1R {
        Dr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc DR2"]
    #[inline(always)]
    pub fn dr2(&self) -> Dr2R {
        Dr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc DR3"]
    #[inline(always)]
    pub fn dr3(&self) -> Dr3R {
        Dr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc DL0R4"]
    #[inline(always)]
    pub fn dl0r4(&self) -> Dl0r4R {
        Dl0r4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DL1R5"]
    #[inline(always)]
    pub fn dl1r5(&self) -> Dl1r5R {
        Dl1r5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DL2R6"]
    #[inline(always)]
    pub fn dl2r6(&self) -> Dl2r6R {
        Dl2r6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DL3R7"]
    #[inline(always)]
    pub fn dl3r7(&self) -> Dl3r7R {
        Dl3r7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc DL4R8"]
    #[inline(always)]
    pub fn dl4r8(&self) -> Dl4r8R {
        Dl4r8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DL5R9"]
    #[inline(always)]
    pub fn dl5r9(&self) -> Dl5r9R {
        Dl5r9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc DL6R10"]
    #[inline(always)]
    pub fn dl6r10(&self) -> Dl6r10R {
        Dl6r10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc DL7R11"]
    #[inline(always)]
    pub fn dl7r11(&self) -> Dl7r11R {
        Dl7r11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc DL8"]
    #[inline(always)]
    pub fn dl8(&self) -> Dl8R {
        Dl8R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc DL9"]
    #[inline(always)]
    pub fn dl9(&self) -> Dl9R {
        Dl9R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc DL10"]
    #[inline(always)]
    pub fn dl10(&self) -> Dl10R {
        Dl10R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DL11"]
    #[inline(always)]
    pub fn dl11(&self) -> Dl11R {
        Dl11R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DADR2")
            .field("dr0", &self.dr0())
            .field("dr1", &self.dr1())
            .field("dr2", &self.dr2())
            .field("dr3", &self.dr3())
            .field("dl0r4", &self.dl0r4())
            .field("dl1r5", &self.dl1r5())
            .field("dl2r6", &self.dl2r6())
            .field("dl3r7", &self.dl3r7())
            .field("dl4r8", &self.dl4r8())
            .field("dl5r9", &self.dl5r9())
            .field("dl6r10", &self.dl6r10())
            .field("dl7r11", &self.dl7r11())
            .field("dl8", &self.dl8())
            .field("dl9", &self.dl9())
            .field("dl10", &self.dl10())
            .field("dl11", &self.dl11())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc DR0"]
    #[inline(always)]
    pub fn dr0(&mut self) -> Dr0W<'_, Dadr2Spec> {
        Dr0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc DR1"]
    #[inline(always)]
    pub fn dr1(&mut self) -> Dr1W<'_, Dadr2Spec> {
        Dr1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc DR2"]
    #[inline(always)]
    pub fn dr2(&mut self) -> Dr2W<'_, Dadr2Spec> {
        Dr2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc DR3"]
    #[inline(always)]
    pub fn dr3(&mut self) -> Dr3W<'_, Dadr2Spec> {
        Dr3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc DL0R4"]
    #[inline(always)]
    pub fn dl0r4(&mut self) -> Dl0r4W<'_, Dadr2Spec> {
        Dl0r4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc DL1R5"]
    #[inline(always)]
    pub fn dl1r5(&mut self) -> Dl1r5W<'_, Dadr2Spec> {
        Dl1r5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc DL2R6"]
    #[inline(always)]
    pub fn dl2r6(&mut self) -> Dl2r6W<'_, Dadr2Spec> {
        Dl2r6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc DL3R7"]
    #[inline(always)]
    pub fn dl3r7(&mut self) -> Dl3r7W<'_, Dadr2Spec> {
        Dl3r7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc DL4R8"]
    #[inline(always)]
    pub fn dl4r8(&mut self) -> Dl4r8W<'_, Dadr2Spec> {
        Dl4r8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc DL5R9"]
    #[inline(always)]
    pub fn dl5r9(&mut self) -> Dl5r9W<'_, Dadr2Spec> {
        Dl5r9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc DL6R10"]
    #[inline(always)]
    pub fn dl6r10(&mut self) -> Dl6r10W<'_, Dadr2Spec> {
        Dl6r10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc DL7R11"]
    #[inline(always)]
    pub fn dl7r11(&mut self) -> Dl7r11W<'_, Dadr2Spec> {
        Dl7r11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc DL8"]
    #[inline(always)]
    pub fn dl8(&mut self) -> Dl8W<'_, Dadr2Spec> {
        Dl8W::new(self, 12)
    }
    #[doc = "Bit 13 - desc DL9"]
    #[inline(always)]
    pub fn dl9(&mut self) -> Dl9W<'_, Dadr2Spec> {
        Dl9W::new(self, 13)
    }
    #[doc = "Bit 14 - desc DL10"]
    #[inline(always)]
    pub fn dl10(&mut self) -> Dl10W<'_, Dadr2Spec> {
        Dl10W::new(self, 14)
    }
    #[doc = "Bit 15 - desc DL11"]
    #[inline(always)]
    pub fn dl11(&mut self) -> Dl11W<'_, Dadr2Spec> {
        Dl11W::new(self, 15)
    }
}
#[doc = "desc DADR2\n\nYou can [`read`](crate::Reg::read) this register and get [`dadr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dadr2Spec;
impl crate::RegisterSpec for Dadr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dadr2::R`](R) reader structure"]
impl crate::Readable for Dadr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dadr2::W`](W) writer structure"]
impl crate::Writable for Dadr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DADR2 to value 0"]
impl crate::Resettable for Dadr2Spec {}
