#[doc = "Register `BWSR2` reader"]
pub type R = crate::R<Bwsr2Spec>;
#[doc = "Register `BWSR2` writer"]
pub type W = crate::W<Bwsr2Spec>;
#[doc = "Field `MSKW` reader - desc MSKW"]
pub type MskwR = crate::FieldReader;
#[doc = "Field `MSKW` writer - desc MSKW"]
pub type MskwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TWEG` reader - desc TWEG"]
pub type TwegR = crate::FieldReader;
#[doc = "Field `TWEG` writer - desc TWEG"]
pub type TwegW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - desc MSKW"]
    #[inline(always)]
    pub fn mskw(&self) -> MskwR {
        MskwR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - desc TWEG"]
    #[inline(always)]
    pub fn tweg(&self) -> TwegR {
        TwegR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BWSR2")
            .field("mskw", &self.mskw())
            .field("tweg", &self.tweg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - desc MSKW"]
    #[inline(always)]
    pub fn mskw(&mut self) -> MskwW<'_, Bwsr2Spec> {
        MskwW::new(self, 0)
    }
    #[doc = "Bits 8:9 - desc TWEG"]
    #[inline(always)]
    pub fn tweg(&mut self) -> TwegW<'_, Bwsr2Spec> {
        TwegW::new(self, 8)
    }
}
#[doc = "desc BWSR2\n\nYou can [`read`](crate::Reg::read) this register and get [`bwsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bwsr2Spec;
impl crate::RegisterSpec for Bwsr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bwsr2::R`](R) reader structure"]
impl crate::Readable for Bwsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`bwsr2::W`](W) writer structure"]
impl crate::Writable for Bwsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BWSR2 to value 0"]
impl crate::Resettable for Bwsr2Spec {}
