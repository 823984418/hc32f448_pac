#[doc = "Register `CKCR` reader"]
pub type R = crate::R<CkcrSpec>;
#[doc = "Register `CKCR` writer"]
pub type W = crate::W<CkcrSpec>;
#[doc = "Field `PYOAD` reader - desc PYOAD"]
pub type PyoadR = crate::BitReader;
#[doc = "Field `PYOAD` writer - desc PYOAD"]
pub type PyoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCOAD` reader - desc ECCOAD"]
pub type EccoadR = crate::BitReader;
#[doc = "Field `ECCOAD` writer - desc ECCOAD"]
pub type EccoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BECCOAD` reader - desc BECCOAD"]
pub type BeccoadR = crate::BitReader;
#[doc = "Field `BECCOAD` writer - desc BECCOAD"]
pub type BeccoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCMOD` reader - desc ECCMOD"]
pub type EccmodR = crate::FieldReader;
#[doc = "Field `ECCMOD` writer - desc ECCMOD"]
pub type EccmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BECCMOD` reader - desc BECCMOD"]
pub type BeccmodR = crate::FieldReader;
#[doc = "Field `BECCMOD` writer - desc BECCMOD"]
pub type BeccmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc PYOAD"]
    #[inline(always)]
    pub fn pyoad(&self) -> PyoadR {
        PyoadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - desc ECCOAD"]
    #[inline(always)]
    pub fn eccoad(&self) -> EccoadR {
        EccoadR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc BECCOAD"]
    #[inline(always)]
    pub fn beccoad(&self) -> BeccoadR {
        BeccoadR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - desc ECCMOD"]
    #[inline(always)]
    pub fn eccmod(&self) -> EccmodR {
        EccmodR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - desc BECCMOD"]
    #[inline(always)]
    pub fn beccmod(&self) -> BeccmodR {
        BeccmodR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKCR")
            .field("pyoad", &self.pyoad())
            .field("eccoad", &self.eccoad())
            .field("beccoad", &self.beccoad())
            .field("eccmod", &self.eccmod())
            .field("beccmod", &self.beccmod())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc PYOAD"]
    #[inline(always)]
    pub fn pyoad(&mut self) -> PyoadW<'_, CkcrSpec> {
        PyoadW::new(self, 0)
    }
    #[doc = "Bit 16 - desc ECCOAD"]
    #[inline(always)]
    pub fn eccoad(&mut self) -> EccoadW<'_, CkcrSpec> {
        EccoadW::new(self, 16)
    }
    #[doc = "Bit 17 - desc BECCOAD"]
    #[inline(always)]
    pub fn beccoad(&mut self) -> BeccoadW<'_, CkcrSpec> {
        BeccoadW::new(self, 17)
    }
    #[doc = "Bits 24:25 - desc ECCMOD"]
    #[inline(always)]
    pub fn eccmod(&mut self) -> EccmodW<'_, CkcrSpec> {
        EccmodW::new(self, 24)
    }
    #[doc = "Bits 26:27 - desc BECCMOD"]
    #[inline(always)]
    pub fn beccmod(&mut self) -> BeccmodW<'_, CkcrSpec> {
        BeccmodW::new(self, 26)
    }
}
#[doc = "desc CKCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkcrSpec;
impl crate::RegisterSpec for CkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckcr::R`](R) reader structure"]
impl crate::Readable for CkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ckcr::W`](W) writer structure"]
impl crate::Writable for CkcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKCR to value 0"]
impl crate::Resettable for CkcrSpec {}
