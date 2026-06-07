#[doc = "Register `POCRX` reader"]
pub type R = crate::R<PocrxSpec>;
#[doc = "Register `POCRX` writer"]
pub type W = crate::W<PocrxSpec>;
#[doc = "Field `DIVCK` reader - desc DIVCK"]
pub type DivckR = crate::FieldReader;
#[doc = "Field `DIVCK` writer - desc DIVCK"]
pub type DivckW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PXMMD` reader - desc PXMMD"]
pub type PxmmdR = crate::FieldReader;
#[doc = "Field `PXMMD` writer - desc PXMMD"]
pub type PxmmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LVLS` reader - desc LVLS"]
pub type LvlsR = crate::FieldReader;
#[doc = "Field `LVLS` writer - desc LVLS"]
pub type LvlsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - desc DIVCK"]
    #[inline(always)]
    pub fn divck(&self) -> DivckR {
        DivckR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - desc PXMMD"]
    #[inline(always)]
    pub fn pxmmd(&self) -> PxmmdR {
        PxmmdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc LVLS"]
    #[inline(always)]
    pub fn lvls(&self) -> LvlsR {
        LvlsR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POCRX")
            .field("divck", &self.divck())
            .field("pxmmd", &self.pxmmd())
            .field("lvls", &self.lvls())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - desc DIVCK"]
    #[inline(always)]
    pub fn divck(&mut self) -> DivckW<'_, PocrxSpec> {
        DivckW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc PXMMD"]
    #[inline(always)]
    pub fn pxmmd(&mut self) -> PxmmdW<'_, PocrxSpec> {
        PxmmdW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc LVLS"]
    #[inline(always)]
    pub fn lvls(&mut self) -> LvlsW<'_, PocrxSpec> {
        LvlsW::new(self, 6)
    }
}
#[doc = "desc POCRX\n\nYou can [`read`](crate::Reg::read) this register and get [`pocrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pocrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PocrxSpec;
impl crate::RegisterSpec for PocrxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pocrx::R`](R) reader structure"]
impl crate::Readable for PocrxSpec {}
#[doc = "`write(|w| ..)` method takes [`pocrx::W`](W) writer structure"]
impl crate::Writable for PocrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POCRX to value 0xff00"]
impl crate::Resettable for PocrxSpec {
    const RESET_VALUE: u16 = 0xff00;
}
