#[doc = "Register `PORRC` reader"]
pub type R = crate::R<PorrcSpec>;
#[doc = "Register `PORRC` writer"]
pub type W = crate::W<PorrcSpec>;
#[doc = "Field `POR00` reader - desc POR00"]
pub type Por00R = crate::BitReader;
#[doc = "Field `POR00` writer - desc POR00"]
pub type Por00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR01` reader - desc POR01"]
pub type Por01R = crate::BitReader;
#[doc = "Field `POR01` writer - desc POR01"]
pub type Por01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR02` reader - desc POR02"]
pub type Por02R = crate::BitReader;
#[doc = "Field `POR02` writer - desc POR02"]
pub type Por02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR03` reader - desc POR03"]
pub type Por03R = crate::BitReader;
#[doc = "Field `POR03` writer - desc POR03"]
pub type Por03W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR04` reader - desc POR04"]
pub type Por04R = crate::BitReader;
#[doc = "Field `POR04` writer - desc POR04"]
pub type Por04W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR05` reader - desc POR05"]
pub type Por05R = crate::BitReader;
#[doc = "Field `POR05` writer - desc POR05"]
pub type Por05W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR06` reader - desc POR06"]
pub type Por06R = crate::BitReader;
#[doc = "Field `POR06` writer - desc POR06"]
pub type Por06W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR07` reader - desc POR07"]
pub type Por07R = crate::BitReader;
#[doc = "Field `POR07` writer - desc POR07"]
pub type Por07W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR08` reader - desc POR08"]
pub type Por08R = crate::BitReader;
#[doc = "Field `POR08` writer - desc POR08"]
pub type Por08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR09` reader - desc POR09"]
pub type Por09R = crate::BitReader;
#[doc = "Field `POR09` writer - desc POR09"]
pub type Por09W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR10` reader - desc POR10"]
pub type Por10R = crate::BitReader;
#[doc = "Field `POR10` writer - desc POR10"]
pub type Por10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR11` reader - desc POR11"]
pub type Por11R = crate::BitReader;
#[doc = "Field `POR11` writer - desc POR11"]
pub type Por11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR12` reader - desc POR12"]
pub type Por12R = crate::BitReader;
#[doc = "Field `POR12` writer - desc POR12"]
pub type Por12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR13` reader - desc POR13"]
pub type Por13R = crate::BitReader;
#[doc = "Field `POR13` writer - desc POR13"]
pub type Por13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR14` reader - desc POR14"]
pub type Por14R = crate::BitReader;
#[doc = "Field `POR14` writer - desc POR14"]
pub type Por14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR15` reader - desc POR15"]
pub type Por15R = crate::BitReader;
#[doc = "Field `POR15` writer - desc POR15"]
pub type Por15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POR00"]
    #[inline(always)]
    pub fn por00(&self) -> Por00R {
        Por00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POR01"]
    #[inline(always)]
    pub fn por01(&self) -> Por01R {
        Por01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POR02"]
    #[inline(always)]
    pub fn por02(&self) -> Por02R {
        Por02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc POR03"]
    #[inline(always)]
    pub fn por03(&self) -> Por03R {
        Por03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc POR04"]
    #[inline(always)]
    pub fn por04(&self) -> Por04R {
        Por04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc POR05"]
    #[inline(always)]
    pub fn por05(&self) -> Por05R {
        Por05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc POR06"]
    #[inline(always)]
    pub fn por06(&self) -> Por06R {
        Por06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc POR07"]
    #[inline(always)]
    pub fn por07(&self) -> Por07R {
        Por07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc POR08"]
    #[inline(always)]
    pub fn por08(&self) -> Por08R {
        Por08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc POR09"]
    #[inline(always)]
    pub fn por09(&self) -> Por09R {
        Por09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc POR10"]
    #[inline(always)]
    pub fn por10(&self) -> Por10R {
        Por10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc POR11"]
    #[inline(always)]
    pub fn por11(&self) -> Por11R {
        Por11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc POR12"]
    #[inline(always)]
    pub fn por12(&self) -> Por12R {
        Por12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc POR13"]
    #[inline(always)]
    pub fn por13(&self) -> Por13R {
        Por13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POR14"]
    #[inline(always)]
    pub fn por14(&self) -> Por14R {
        Por14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POR15"]
    #[inline(always)]
    pub fn por15(&self) -> Por15R {
        Por15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORRC")
            .field("por00", &self.por00())
            .field("por01", &self.por01())
            .field("por02", &self.por02())
            .field("por03", &self.por03())
            .field("por04", &self.por04())
            .field("por05", &self.por05())
            .field("por06", &self.por06())
            .field("por07", &self.por07())
            .field("por08", &self.por08())
            .field("por09", &self.por09())
            .field("por10", &self.por10())
            .field("por11", &self.por11())
            .field("por12", &self.por12())
            .field("por13", &self.por13())
            .field("por14", &self.por14())
            .field("por15", &self.por15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc POR00"]
    #[inline(always)]
    pub fn por00(&mut self) -> Por00W<'_, PorrcSpec> {
        Por00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc POR01"]
    #[inline(always)]
    pub fn por01(&mut self) -> Por01W<'_, PorrcSpec> {
        Por01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc POR02"]
    #[inline(always)]
    pub fn por02(&mut self) -> Por02W<'_, PorrcSpec> {
        Por02W::new(self, 2)
    }
    #[doc = "Bit 3 - desc POR03"]
    #[inline(always)]
    pub fn por03(&mut self) -> Por03W<'_, PorrcSpec> {
        Por03W::new(self, 3)
    }
    #[doc = "Bit 4 - desc POR04"]
    #[inline(always)]
    pub fn por04(&mut self) -> Por04W<'_, PorrcSpec> {
        Por04W::new(self, 4)
    }
    #[doc = "Bit 5 - desc POR05"]
    #[inline(always)]
    pub fn por05(&mut self) -> Por05W<'_, PorrcSpec> {
        Por05W::new(self, 5)
    }
    #[doc = "Bit 6 - desc POR06"]
    #[inline(always)]
    pub fn por06(&mut self) -> Por06W<'_, PorrcSpec> {
        Por06W::new(self, 6)
    }
    #[doc = "Bit 7 - desc POR07"]
    #[inline(always)]
    pub fn por07(&mut self) -> Por07W<'_, PorrcSpec> {
        Por07W::new(self, 7)
    }
    #[doc = "Bit 8 - desc POR08"]
    #[inline(always)]
    pub fn por08(&mut self) -> Por08W<'_, PorrcSpec> {
        Por08W::new(self, 8)
    }
    #[doc = "Bit 9 - desc POR09"]
    #[inline(always)]
    pub fn por09(&mut self) -> Por09W<'_, PorrcSpec> {
        Por09W::new(self, 9)
    }
    #[doc = "Bit 10 - desc POR10"]
    #[inline(always)]
    pub fn por10(&mut self) -> Por10W<'_, PorrcSpec> {
        Por10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc POR11"]
    #[inline(always)]
    pub fn por11(&mut self) -> Por11W<'_, PorrcSpec> {
        Por11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc POR12"]
    #[inline(always)]
    pub fn por12(&mut self) -> Por12W<'_, PorrcSpec> {
        Por12W::new(self, 12)
    }
    #[doc = "Bit 13 - desc POR13"]
    #[inline(always)]
    pub fn por13(&mut self) -> Por13W<'_, PorrcSpec> {
        Por13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc POR14"]
    #[inline(always)]
    pub fn por14(&mut self) -> Por14W<'_, PorrcSpec> {
        Por14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc POR15"]
    #[inline(always)]
    pub fn por15(&mut self) -> Por15W<'_, PorrcSpec> {
        Por15W::new(self, 15)
    }
}
#[doc = "desc PORRC\n\nYou can [`read`](crate::Reg::read) this register and get [`porrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PorrcSpec;
impl crate::RegisterSpec for PorrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`porrc::R`](R) reader structure"]
impl crate::Readable for PorrcSpec {}
#[doc = "`write(|w| ..)` method takes [`porrc::W`](W) writer structure"]
impl crate::Writable for PorrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORRC to value 0"]
impl crate::Resettable for PorrcSpec {}
