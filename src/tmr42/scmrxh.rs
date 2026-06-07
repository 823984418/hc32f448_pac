#[doc = "Register `SCMRXH` reader"]
pub type R = crate::R<ScmrxhSpec>;
#[doc = "Register `SCMRXH` writer"]
pub type W = crate::W<ScmrxhSpec>;
#[doc = "Field `AMC` reader - desc AMC"]
pub type AmcR = crate::FieldReader;
#[doc = "Field `AMC` writer - desc AMC"]
pub type AmcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MZCE` reader - desc MZCE"]
pub type MzceR = crate::BitReader;
#[doc = "Field `MZCE` writer - desc MZCE"]
pub type MzceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCE` reader - desc MPCE"]
pub type MpceR = crate::BitReader;
#[doc = "Field `MPCE` writer - desc MPCE"]
pub type MpceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc AMC"]
    #[inline(always)]
    pub fn amc(&self) -> AmcR {
        AmcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - desc MZCE"]
    #[inline(always)]
    pub fn mzce(&self) -> MzceR {
        MzceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc MPCE"]
    #[inline(always)]
    pub fn mpce(&self) -> MpceR {
        MpceR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCMRXH")
            .field("amc", &self.amc())
            .field("mzce", &self.mzce())
            .field("mpce", &self.mpce())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - desc AMC"]
    #[inline(always)]
    pub fn amc(&mut self) -> AmcW<'_, ScmrxhSpec> {
        AmcW::new(self, 0)
    }
    #[doc = "Bit 6 - desc MZCE"]
    #[inline(always)]
    pub fn mzce(&mut self) -> MzceW<'_, ScmrxhSpec> {
        MzceW::new(self, 6)
    }
    #[doc = "Bit 7 - desc MPCE"]
    #[inline(always)]
    pub fn mpce(&mut self) -> MpceW<'_, ScmrxhSpec> {
        MpceW::new(self, 7)
    }
}
#[doc = "desc SCMRXH\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrxh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrxh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmrxhSpec;
impl crate::RegisterSpec for ScmrxhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`scmrxh::R`](R) reader structure"]
impl crate::Readable for ScmrxhSpec {}
#[doc = "`write(|w| ..)` method takes [`scmrxh::W`](W) writer structure"]
impl crate::Writable for ScmrxhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMRXH to value 0xff00"]
impl crate::Resettable for ScmrxhSpec {
    const RESET_VALUE: u16 = 0xff00;
}
