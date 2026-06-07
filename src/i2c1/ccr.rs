#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `SLOWW` reader - desc SLOWW"]
pub type SlowwR = crate::FieldReader;
#[doc = "Field `SLOWW` writer - desc SLOWW"]
pub type SlowwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHIGHW` reader - desc SHIGHW"]
pub type ShighwR = crate::FieldReader;
#[doc = "Field `SHIGHW` writer - desc SHIGHW"]
pub type ShighwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKDIV` reader - desc CKDIV"]
pub type CkdivR = crate::FieldReader;
#[doc = "Field `CKDIV` writer - desc CKDIV"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - desc SLOWW"]
    #[inline(always)]
    pub fn sloww(&self) -> SlowwR {
        SlowwR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - desc SHIGHW"]
    #[inline(always)]
    pub fn shighw(&self) -> ShighwR {
        ShighwR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - desc CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("sloww", &self.sloww())
            .field("shighw", &self.shighw())
            .field("ckdiv", &self.ckdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - desc SLOWW"]
    #[inline(always)]
    pub fn sloww(&mut self) -> SlowwW<'_, CcrSpec> {
        SlowwW::new(self, 0)
    }
    #[doc = "Bits 8:15 - desc SHIGHW"]
    #[inline(always)]
    pub fn shighw(&mut self) -> ShighwW<'_, CcrSpec> {
        ShighwW::new(self, 8)
    }
    #[doc = "Bits 16:18 - desc CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CkdivW<'_, CcrSpec> {
        CkdivW::new(self, 16)
    }
}
#[doc = "desc CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR to value 0x1f1f"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0x1f1f;
}
