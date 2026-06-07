#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `LBCK` reader - desc LBCK"]
pub type LbckR = crate::BitReader;
#[doc = "Field `LBCK` writer - desc LBCK"]
pub type LbckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - desc TX"]
pub type TxR = crate::FieldReader;
#[doc = "Field `TX` writer - desc TX"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX` reader - desc RX"]
pub type RxR = crate::BitReader;
#[doc = "Field `TXBNP` reader - desc TXBNP"]
pub type TxbnpR = crate::FieldReader;
#[doc = "Field `PVAL` reader - desc PVAL"]
pub type PvalR = crate::BitReader;
#[doc = "Field `TXBNS` reader - desc TXBNS"]
pub type TxbnsR = crate::FieldReader;
#[doc = "Field `SVAL` reader - desc SVAL"]
pub type SvalR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - desc LBCK"]
    #[inline(always)]
    pub fn lbck(&self) -> LbckR {
        LbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - desc TX"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - desc RX"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - desc TXBNP"]
    #[inline(always)]
    pub fn txbnp(&self) -> TxbnpR {
        TxbnpR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - desc PVAL"]
    #[inline(always)]
    pub fn pval(&self) -> PvalR {
        PvalR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - desc TXBNS"]
    #[inline(always)]
    pub fn txbns(&self) -> TxbnsR {
        TxbnsR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - desc SVAL"]
    #[inline(always)]
    pub fn sval(&self) -> SvalR {
        SvalR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("lbck", &self.lbck())
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .field("txbnp", &self.txbnp())
            .field("pval", &self.pval())
            .field("txbns", &self.txbns())
            .field("sval", &self.sval())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - desc LBCK"]
    #[inline(always)]
    pub fn lbck(&mut self) -> LbckW<'_, TestSpec> {
        LbckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - desc TX"]
    #[inline(always)]
    pub fn tx(&mut self) -> TxW<'_, TestSpec> {
        TxW::new(self, 5)
    }
}
#[doc = "desc TEST\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {}
