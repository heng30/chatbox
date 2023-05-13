#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextType {
    EnUs,
    ZhCn,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AzureTextItem {
    pub text_type: TextType,
    pub text: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Speech2Text {
    #[serde(rename = "RecognitionStatus" )]
    pub recognition_status: String,

    #[serde(rename = "Offset" )]
    pub offset: u64,

    #[serde(rename = "Duration" )]
    pub duration: u64,

    #[serde(rename = "DisplayText" )]
    pub display_text: String,
}
