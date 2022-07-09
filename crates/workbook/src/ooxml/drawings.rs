use super::defaults::default_false;
use super::defaults::default_string_empty;
use super::defaults::default_true;
use super::defaults::default_zero_i32;
use super::simple_types::StBlackWhiteMode;
use xmlserde::Unparsed;

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub enum EgObjectChoices {
    #[xmlserde(name = b"sp")]
    Sp(CtShape),
    #[xmlserde(name = b"grpSp")]
    GrpSp(CtGroupShape),
    #[xmlserde(name = b"graphicFrame")]
    GraphicFrame(CtGroupShape),
    #[xmlserde(name = b"cxnSp")]
    CxnSp(CtConnector),
    #[xmlserde(name = b"pic")]
    Pic(CtPicture),
    #[xmlserde(name = b"contentPart")]
    ContentPart(CtRel),
}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtShape {
    #[xmlserde(name = b"nvSpPr", ty = "child")]
    pub nv_sp_pr: CtShapeNoneVisual,
    #[xmlserde(name = b"spPr", ty = "child")]
    pub sp_pr: CtShapeProperties,
    #[xmlserde(name = b"style", ty = "child")]
    pub style: Option<CtShapeStyle>,
    #[xmlserde(name = b"txBody", ty = "child")]
    pub tx_body: Option<CtTextBody>,
    #[xmlserde(name = b"macro", ty = "attr")]
    pub r#macro: Option<String>,
    #[xmlserde(name = b"textlink", ty = "attr")]
    pub textlink: Option<String>,
    #[xmlserde(name = b"extLst", ty = "child")]
    pub ext_lst: Option<Unparsed>,
    #[xmlserde(name = b"fLocksText", ty = "attr", default = "default_true")]
    pub f_locks_text: bool,
    #[xmlserde(name = b"fPublished", ty = "attr", default = "default_false")]
    pub f_published: bool,
}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtShapeNoneVisual {
    #[xmlserde(name = b"cNvPr", ty = "child")]
    pub c_nv_pr: CtNonVisualDrawingProps,
    #[xmlserde(name = b"cNvSpPr", ty = "child")]
    pub c_nv_sp_pr: CtNonVisualDrawingShapeProps,
}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtNonVisualDrawingProps {
    #[xmlserde(name = b"id", ty = "attr")]
    pub id: u32,
    #[xmlserde(name = b"name", ty = "attr")]
    pub name: String,
    #[xmlserde(name = b"descr", ty = "attr", default = "default_string_empty")]
    pub descr: String,
    #[xmlserde(name = b"hidden", ty = "attr", default = "default_false")]
    pub hidden: bool,
    #[xmlserde(name = b"title", ty = "attr", default = "default_string_empty")]
    pub title: String,
    #[xmlserde(name = b"hlinkClick", ty = "child")]
    pub hlink_click: Option<Unparsed>,
    #[xmlserde(name = b"hlinkHover", ty = "child")]
    pub hlink_hover: Option<Unparsed>,
    #[xmlserde(name = b"extLst", ty = "child")]
    pub ext_lst: Option<Unparsed>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtNonVisualDrawingShapeProps {
    #[xmlserde(name = b"spLocks", ty = "child")]
    pub sp_locks: Option<CtShapeLocking>,
    #[xmlserde(name = b"extLst", ty = "child")]
    pub ext_lst: Option<Unparsed>,
    #[xmlserde(name = b"txBox", ty = "attr", default = "default_false")]
    pub tx_box: bool,
}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtShapeLocking {
    #[xmlserde(name = b"noTextEdit", ty = "attr", default = "default_false")]
    pub no_text_edit: bool,
    #[xmlserde(name = b"noGrp", ty = "attr", default = "default_false")]
    pub no_grp: bool,
    #[xmlserde(name = b"noSelect", ty = "attr", default = "default_false")]
    pub no_select: bool,
    #[xmlserde(name = b"noRot", ty = "attr", default = "default_false")]
    pub no_rot: bool,
    #[xmlserde(name = b"noChangeAspect", ty = "attr", default = "default_false")]
    pub no_change_aspect: bool,
    #[xmlserde(name = b"noMove", ty = "attr", default = "default_false")]
    pub no_move: bool,
    #[xmlserde(name = b"noResize", ty = "attr", default = "default_false")]
    pub no_resize: bool,
    #[xmlserde(name = b"noEditPoints", ty = "attr", default = "default_false")]
    pub no_edit_points: bool,
    #[xmlserde(name = b"noAdjustHandles", ty = "attr", default = "default_false")]
    pub no_adjust_handles: bool,
    #[xmlserde(name = b"noChangeArrowHeads", ty = "attr", default = "default_false")]
    pub no_change_arrow_heads: bool,
    #[xmlserde(name = b"noChangeShapeType", ty = "attr", default = "default_false")]
    pub no_change_shape_type: bool,
    #[xmlserde(name = b"extLst", ty = "child")]
    pub ext_lst: Option<Unparsed>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtShapeProperties {
    #[xmlserde(name = b"xfrm", ty = "child")]
    pub xfrm: Option<CtTransform2D>,
    // ref EgGeometry
    // ref FillProperties
    #[xmlserde(name = b"bwMode", ty = "attr")]
    pub bw_mode: Option<StBlackWhiteMode>,
}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtGroupShape {}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtGraphicalObjectFrame {}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtConnector {}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtPicture {}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtRel {
    #[xmlserde(name = b"r:id", ty = "attr")]
    pub id: String,
}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtTransform2D {
    #[xmlserde(name = b"off", ty = "child")]
    pub off: Option<CtPoint2D>,
    #[xmlserde(name = b"ext", ty = "child")]
    pub ext: Option<CtPositiveSize2D>,
    #[xmlserde(name = b"rot", ty = "attr", default = "default_zero_i32")]
    pub rot: i32,
    #[xmlserde(name = b"flipH", ty = "attr", default = "default_false")]
    pub flip_h: bool,
    #[xmlserde(name = b"flipV", ty = "attr", default = "default_false")]
    pub flip_v: bool,
}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtPoint2D {
    #[xmlserde(name = b"x", ty = "attr")]
    pub x: i64,
    #[xmlserde(name = b"y", ty = "attr")]
    pub y: i64,
}

#[derive(Debug, XmlSerialize, XmlDeserialize)]
pub struct CtPositiveSize2D {
    #[xmlserde(name = b"cx", ty = "attr")]
    pub cx: u64,
    #[xmlserde(name = b"cy", ty = "attr")]
    pub cy: u64,
}
