#[doc = "Register `WLOCK` reader"]
pub type R = crate::R<WlockSpec>;
#[doc = "Register `WLOCK` writer"]
pub type W = crate::W<WlockSpec>;
#[doc = "Field `WLOCK0` reader - desc WLOCK0"]
pub type Wlock0R = crate::BitReader;
#[doc = "Field `WLOCK0` writer - desc WLOCK0"]
pub type Wlock0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc WLOCK0"]
    #[inline(always)]
    pub fn wlock0(&self) -> Wlock0R {
        Wlock0R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WLOCK")
            .field("wlock0", &self.wlock0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc WLOCK0"]
    #[inline(always)]
    pub fn wlock0(&mut self) -> Wlock0W<'_, WlockSpec> {
        Wlock0W::new(self, 0)
    }
}
#[doc = "desc WLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`wlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WlockSpec;
impl crate::RegisterSpec for WlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wlock::R`](R) reader structure"]
impl crate::Readable for WlockSpec {}
#[doc = "`write(|w| ..)` method takes [`wlock::W`](W) writer structure"]
impl crate::Writable for WlockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WLOCK to value 0"]
impl crate::Resettable for WlockSpec {}
