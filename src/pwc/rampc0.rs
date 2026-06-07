#[doc = "Register `RAMPC0` reader"]
pub type R = crate::R<Rampc0Spec>;
#[doc = "Register `RAMPC0` writer"]
pub type W = crate::W<Rampc0Spec>;
#[doc = "Field `RAMPDC0` reader - desc RAMPDC0"]
pub type Rampdc0R = crate::BitReader;
#[doc = "Field `RAMPDC0` writer - desc RAMPDC0"]
pub type Rampdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPDC10` reader - desc RAMPDC10"]
pub type Rampdc10R = crate::BitReader;
#[doc = "Field `RAMPDC10` writer - desc RAMPDC10"]
pub type Rampdc10W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc RAMPDC0"]
    #[inline(always)]
    pub fn rampdc0(&self) -> Rampdc0R {
        Rampdc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - desc RAMPDC10"]
    #[inline(always)]
    pub fn rampdc10(&self) -> Rampdc10R {
        Rampdc10R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMPC0")
            .field("rampdc0", &self.rampdc0())
            .field("rampdc10", &self.rampdc10())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc RAMPDC0"]
    #[inline(always)]
    pub fn rampdc0(&mut self) -> Rampdc0W<'_, Rampc0Spec> {
        Rampdc0W::new(self, 0)
    }
    #[doc = "Bit 10 - desc RAMPDC10"]
    #[inline(always)]
    pub fn rampdc10(&mut self) -> Rampdc10W<'_, Rampc0Spec> {
        Rampdc10W::new(self, 10)
    }
}
#[doc = "desc RAMPC0\n\nYou can [`read`](crate::Reg::read) this register and get [`rampc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rampc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rampc0Spec;
impl crate::RegisterSpec for Rampc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rampc0::R`](R) reader structure"]
impl crate::Readable for Rampc0Spec {}
#[doc = "`write(|w| ..)` method takes [`rampc0::W`](W) writer structure"]
impl crate::Writable for Rampc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAMPC0 to value 0"]
impl crate::Resettable for Rampc0Spec {}
