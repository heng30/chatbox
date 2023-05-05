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
