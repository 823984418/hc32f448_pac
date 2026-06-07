#[doc = "Register `CANCKCFGR` reader"]
pub type R = crate::R<CanckcfgrSpec>;
#[doc = "Register `CANCKCFGR` writer"]
pub type W = crate::W<CanckcfgrSpec>;
#[doc = "Field `MCAN1CKS` reader - desc MCAN1CKS"]
pub type Mcan1cksR = crate::FieldReader;
#[doc = "Field `MCAN1CKS` writer - desc MCAN1CKS"]
pub type Mcan1cksW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCAN2CKS` reader - desc MCAN2CKS"]
pub type Mcan2cksR = crate::FieldReader;
#[doc = "Field `MCAN2CKS` writer - desc MCAN2CKS"]
pub type Mcan2cksW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc MCAN1CKS"]
    #[inline(always)]
    pub fn mcan1cks(&self) -> Mcan1cksR {
        Mcan1cksR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc MCAN2CKS"]
    #[inline(always)]
    pub fn mcan2cks(&self) -> Mcan2cksR {
        Mcan2cksR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANCKCFGR")
            .field("mcan1cks", &self.mcan1cks())
            .field("mcan2cks", &self.mcan2cks())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - desc MCAN1CKS"]
    #[inline(always)]
    pub fn mcan1cks(&mut self) -> Mcan1cksW<'_, CanckcfgrSpec> {
        Mcan1cksW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc MCAN2CKS"]
    #[inline(always)]
    pub fn mcan2cks(&mut self) -> Mcan2cksW<'_, CanckcfgrSpec> {
        Mcan2cksW::new(self, 4)
    }
}
#[doc = "desc CANCKCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`canckcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canckcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanckcfgrSpec;
impl crate::RegisterSpec for CanckcfgrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`canckcfgr::R`](R) reader structure"]
impl crate::Readable for CanckcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`canckcfgr::W`](W) writer structure"]
impl crate::Writable for CanckcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CANCKCFGR to value 0xdd"]
impl crate::Resettable for CanckcfgrSpec {
    const RESET_VALUE: u16 = 0xdd;
}
