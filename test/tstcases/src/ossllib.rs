#[allow(unused_imports)]
use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence,asn1_int_choice};
#[allow(unused_imports)]
use asn1obj::base::{Asn1Object,Asn1Integer,Asn1BigNum,Asn1Any,Asn1Time,Asn1Boolean,Asn1PrintableString,Asn1BitString,Asn1Null,Asn1OctData,Asn1BitData,Asn1String};
#[allow(unused_imports)]
use asn1obj::complex::{Asn1Set,Asn1ImpSet,Asn1Seq,Asn1Opt,Asn1Imp,Asn1Ndef,Asn1SeqSelector,Asn1BitSeq};
#[allow(unused_imports)]
use asn1obj::strop::{asn1_format_line};
#[allow(unused_imports)]
use asn1obj::asn1impl::{Asn1Op,Asn1Selector};
#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

use std::error::Error;
use std::boxed::Box;
use std::io::{Write};

use super::asn1def::*;

#[derive(Clone)]
#[asn1_int_choice(debug=3,unicode=0,ascii=1,selector=stype)]
pub struct SpcString {
	pub stype :i32,
	pub unicode : Asn1Imp<Asn1OctData,0>,
	pub ascii :Asn1Imp<Asn1OctData,1>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct SpcSerializedObject {
	pub classid :Asn1OctData,
	pub serializeddata : Asn1OctData,
}

#[derive(Clone)]
#[asn1_int_choice(debug=3,selector=stype,url=0,moniker=1,file=2)]
pub struct SpcLink {
	pub stype :i32,
	pub url :Asn1Imp<Asn1OctData,0>,
	pub moniker :Asn1Imp<SpcSerializedObject,1>,
	pub file :Asn1Imp<SpcString,2>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct SpcSpOpusInfo {
	pub programname :SpcString,
	pub moreinfo : SpcLink,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct SpcAttributeTypeAndOptionalValueElem {
	pub otype  :Asn1Object,
	pub value :Asn1Any,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct SpcAttributeTypeAndOptionalValue {
	pub elem :Asn1Seq<SpcAttributeTypeAndOptionalValueElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct AlgorithmIdentifierElem {
	pub algorithm : Asn1Object,
	pub parameters : Asn1Any,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct AlgorithmIdentifier {
	pub elem : Asn1Seq<AlgorithmIdentifierElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct DigestInfoElem {
	pub digestalgorithm :AlgorithmIdentifier,
	pub digest :Asn1OctData,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct DigestInfo {
	pub elem : Asn1Seq<DigestInfoElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct SpcIndirectDataContentElem {
	pub data :SpcAttributeTypeAndOptionalValue,
	pub messagedigest :DigestInfo,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct SpcIndirectDataContent {
	pub elem :Asn1Seq<SpcIndirectDataContentElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct CatalogAuthAttrElem {
	pub otype :Asn1Object,
	pub contents : Asn1Opt<Asn1Any>,	
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct CatalogAuthAttr {
	pub elem :Asn1Seq<CatalogAuthAttrElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct CatalogInfoElem {
	pub digest : Asn1OctData,
	pub attributes :Asn1Set<CatalogAuthAttr>,	
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct CatalogInfo {
	pub elem : Asn1Seq<CatalogInfoElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct MsCtlContentElem {
	pub stype :SpcAttributeTypeAndOptionalValue,
	pub identifier : Asn1OctData,
	pub time :Asn1Time,
	pub version :SpcAttributeTypeAndOptionalValue,
	pub header_attributes : Asn1Seq<CatalogInfo>,
	pub filename :Asn1Opt<Asn1Any>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct MsCtlContent {
	pub elem :Asn1Seq<MsCtlContentElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct SpcPeImageDataElem {
	pub flags : Asn1BitData,
	pub file :Asn1Opt<SpcLink>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct SpcPeImageData {
	pub elem :Asn1Seq<SpcPeImageDataElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct SpcSipInfoElem {
	pub a :Asn1Integer,
	pub stringv :Asn1OctData,
	pub b :Asn1Integer,
	pub c :Asn1Integer,
	pub d :Asn1Integer,
	pub e :Asn1Integer,
	pub f :Asn1Integer,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct SpcSipInfo {
	pub elem :Asn1Seq<SpcSipInfoElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct MessageImprintElem {
	pub digestalgorithm :AlgorithmIdentifier,
	pub digest : Asn1OctData,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct MessageImprint {
	pub elem :Asn1Seq<MessageImprintElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct TimeStampRequestBlobElem {
	pub otype :Asn1Object,
	pub signature :Asn1OctData,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct TimeStampRequestBlob {
	pub elem :Asn1Seq<TimeStampRequestBlobElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct TimeStampRequestElem {
	pub otype :Asn1Object,
	pub blob :TimeStampRequestBlob,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct TimeStampRequest {
	pub elem :Asn1Seq<TimeStampRequestElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct PKIStatusInfoElem {
	pub status :Asn1Integer,
	pub statusstring :Asn1Opt<Asn1Seq<Asn1String>>,
	pub failinfo :Asn1Opt<Asn1BitData>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct PKIStatusInfo {
	pub elem :Asn1Seq<PKIStatusInfoElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct TimeStampRespElem {
	pub status :PKIStatusInfo,
	pub token :Asn1Opt<Asn1Pkcs7>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct TimeStampResp {
	pub elem :Asn1Seq<TimeStampRespElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct TimeStampReqElem {
	pub version : Asn1Integer,
	pub msgimpprint :MessageImprint,
	pub reqpolicy :Asn1Opt<Asn1Object>,
	pub nonce :Asn1Opt<Asn1Integer>,
	pub certreq :Asn1Boolean,
	pub extensions :Asn1Opt<Asn1ImpSet<Asn1X509Extension,0>>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct TimeStampReq {
	pub elem :Asn1Seq<TimeStampReqElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct TimeStampAccuracyElem {
	pub seconds :Asn1Integer,
	pub millis :Asn1Integer,
	pub micros :Asn1Integer,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct TimeStampAccuracy {
	pub elem :Asn1Seq<TimeStampAccuracyElem>,
}