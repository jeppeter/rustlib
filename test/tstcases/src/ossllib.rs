#[allow(unused_imports)]
use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence,asn1_int_choice};
#[allow(unused_imports)]
use asn1obj::base::{Asn1Object,Asn1Integer,Asn1BigNum,Asn1Any,Asn1Time,Asn1Boolean,Asn1PrintableString,Asn1BitString,Asn1Null,Asn1OctData,Asn1BitData,Asn1String};
#[allow(unused_imports)]
use asn1obj::complex::{Asn1Set,Asn1ImpSet,Asn1Seq,Asn1Opt,Asn1Imp,Asn1Ndef,Asn1SeqSelector,Asn1BitSeq};
#[allow(unused_imports)]
use asn1obj::strop::{asn1_format_line,asn1_enter_debug,asn1_leave_debug};
#[allow(unused_imports)]
use asn1obj::asn1impl::{Asn1Op,Asn1Selector};
#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};
#[allow(unused_imports)]
use asn1obj::{asn1_format_debug,asn1_format_debug_buffer};

use std::error::Error;
use std::boxed::Box;
use std::io::{Write};

use super::asn1def::*;

#[asn1_int_choice(unicode=0,ascii=1,selector=stype)]
#[derive(Clone)]
pub struct SpcString {
	pub stype :i32,
	pub unicode : Asn1Imp<Asn1OctData,0>,
	pub ascii :Asn1Imp<Asn1OctData,1>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcSerializedObject {
	pub classid :Asn1OctData,
	pub serializeddata : Asn1OctData,
}

#[asn1_int_choice(selector=stype,url=0,moniker=1,file=2)]
#[derive(Clone)]
pub struct SpcLink {
	pub stype :i32,
	pub url :Asn1ImpSet<Asn1OctData,0>,
	pub moniker :Asn1ImpSet<SpcSerializedObject,1>,
	pub file :Asn1ImpSet<SpcString,2>,
}


#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcSpOpusInfo {
	pub programname :SpcString,
	pub moreinfo : SpcLink,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcAttributeTypeAndOptionalValueElem {
	pub otype  :Asn1Object,
	pub value :Asn1Any,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcAttributeTypeAndOptionalValue {
	pub elem :Asn1Seq<SpcAttributeTypeAndOptionalValueElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct AlgorithmIdentifierElem {
	pub algorithm : Asn1Object,
	pub parameters : Asn1Any,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct AlgorithmIdentifier {
	pub elem : Asn1Seq<AlgorithmIdentifierElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct DigestInfoElem {
	pub digestalgorithm :AlgorithmIdentifier,
	pub digest :Asn1OctData,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct DigestInfo {
	pub elem : Asn1Seq<DigestInfoElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcIndirectDataContentElem {
	pub data :SpcAttributeTypeAndOptionalValue,
	pub messagedigest :DigestInfo,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcIndirectDataContent {
	pub elem :Asn1Seq<SpcIndirectDataContentElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct CatalogAuthAttrElem {
	pub otype :Asn1Object,
	pub contents : Asn1Opt<Asn1Any>,	
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct CatalogAuthAttr {
	pub elem :Asn1Seq<CatalogAuthAttrElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct CatalogInfoElem {
	pub digest : Asn1OctData,
	pub attributes :Asn1Set<CatalogAuthAttr>,	
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct CatalogInfo {
	pub elem : Asn1Seq<CatalogInfoElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct MsCtlContentElem {
	pub stype :SpcAttributeTypeAndOptionalValue,
	pub identifier : Asn1OctData,
	pub time :Asn1Time,
	pub version :SpcAttributeTypeAndOptionalValue,
	pub header_attributes : Asn1Seq<CatalogInfo>,
	pub filename :Asn1Opt<Asn1Any>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct MsCtlContent {
	pub elem :Asn1Seq<MsCtlContentElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcPeImageDataElem {
	pub flags : Asn1BitData,
	pub file :Asn1Opt<SpcLink>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcPeImageData {
	pub elem :Asn1Seq<SpcPeImageDataElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcSipInfoElem {
	pub a :Asn1Integer,
	pub stringv :Asn1OctData,
	pub b :Asn1Integer,
	pub c :Asn1Integer,
	pub d :Asn1Integer,
	pub e :Asn1Integer,
	pub f :Asn1Integer,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcSipInfo {
	pub elem :Asn1Seq<SpcSipInfoElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct MessageImprintElem {
	pub digestalgorithm :AlgorithmIdentifier,
	pub digest : Asn1OctData,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct MessageImprint {
	pub elem :Asn1Seq<MessageImprintElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct TimeStampRequestBlobElem {
	pub otype :Asn1Object,
	pub signature :Asn1OctData,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct TimeStampRequestBlob {
	pub elem :Asn1Seq<TimeStampRequestBlobElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct TimeStampRequestElem {
	pub otype :Asn1Object,
	pub blob :TimeStampRequestBlob,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct TimeStampRequest {
	pub elem :Asn1Seq<TimeStampRequestElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct PKIStatusInfoElem {
	pub status :Asn1Integer,
	pub statusstring :Asn1Opt<Asn1Seq<Asn1String>>,
	pub failinfo :Asn1Opt<Asn1BitData>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct PKIStatusInfo {
	pub elem :Asn1Seq<PKIStatusInfoElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct TimeStampRespElem {
	pub status :PKIStatusInfo,
	pub token :Asn1Opt<Asn1Pkcs7>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct TimeStampResp {
	pub elem :Asn1Seq<TimeStampRespElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct TimeStampReqElem {
	pub version : Asn1Integer,
	pub msgimpprint :MessageImprint,
	pub reqpolicy :Asn1Opt<Asn1Object>,
	pub nonce :Asn1Opt<Asn1Integer>,
	pub certreq :Asn1Boolean,
	pub extensions :Asn1Opt<Asn1ImpSet<Asn1X509Extension,0>>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct TimeStampReq {
	pub elem :Asn1Seq<TimeStampReqElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct TimeStampAccuracyElem {
	pub seconds :Asn1Integer,
	pub millis :Asn1Integer,
	pub micros :Asn1Integer,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct TimeStampAccuracy {
	pub elem :Asn1Seq<TimeStampAccuracyElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcAsn1CodeElem {
	pub classid : Asn1OctData,
	pub serializeddata :Asn1Opt<Asn1ImpSet<Asn1Object,0>>,
	pub intval :Asn1ImpSet<Asn1BigNum,1>,
	pub ccval :Asn1BigNum,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct SpcAsn1Code {
	pub elem : Asn1Seq<SpcAsn1CodeElem>,
}
