use std::collections::HashMap;
use crate::config;

pub fn tr(text: &str) -> String {
    if config::ui().language == "cn" {
        return text.to_string();
    }

    let mut items: HashMap<&str, &str> = HashMap::new();
    items.insert("原因", "Reason");
    items.insert("新建成功", "New Session Success");
    items.insert("删除成功", "Delete Success");
    items.insert("删除失败", "Delete Failed");
    items.insert("复制失败", "Copy Failed");
    items.insert("复制成功", "Copy Success");
    items.insert("设置默认会话库失败", "Set Default Session Failed");
    items.insert("保存默认会话到数据库失败", "Save Default Session Failed");
    items.insert("保存到数据库失败", "Save to Database Failed");

    items.insert("不允许删除默认会话", "Not Allow To Delete Default Session");
    items.insert("删除会话失败", "Delete Session Failed");
    items.insert("删除会话成功", "Delete Session Success");
    items.insert("重置成功", "Reset Success");
    items.insert("收藏成功", "Marked Success");
    items.insert("取消收藏成功", "Unmarked Success");
    items.insert("保存会话失败", "Save Session Failed");
    items.insert("保存会话成功", "Save Session Success");
    items.insert("保存会话配置失败", "Save Session Configure Failed");
    items.insert("保存会话配置成功", "Save Session Configure Success");
    items.insert("保存失败", "Save Failed");
    items.insert("保存成功", "Save Success");
    items.insert("隐藏程序失败", "Hide Window Failed");
    items.insert("清除缓存失败", "Clean Cache Failed");
    items.insert("清除缓存成功", "Clean Cache Success");
    items.insert("请进行音频配置", "Please Configure The Audio Setting");
    items.insert("播放音频失败", "Play Audio Failed");
    items.insert("获取归档文件失败", "Get Archive Files Failed");
    items.insert("没有可归档的对话", "No Chats TO Be Archived");

    if let Some(txt) = items.get(text) {
        return txt.to_string();
    }

    text.to_string()
}
