- `.cargo/registry/src/github.com-1ecc6299db9ec823/i-slint-core-1.0.2/model.rs:958` 修改为下面的代码：
```
let mut new_components = Vec::new();
while new_offset > 0 && new_offset_y > -vp_y {
    new_offset -= 1;

    // Start: 添加的代码，进行错误处理
    if model.row_data(new_offset).is_none() {
        continue;
    }
    // End: 添加的代码，进行错误处理

    let new_component = init();
    new_component.update(new_offset, model.row_data(new_offset).unwrap());
    new_offset_y -=
        new_component.as_pin_ref().get_item_ref(0).as_ref().geometry().height_length();
    new_components.push(new_component);
}
```
