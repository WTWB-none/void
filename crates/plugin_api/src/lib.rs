use gpui::{Div, ParentElement, SharedString, Styled, div, prelude::FluentBuilder, rgba};
use mlua::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum LuaNode {
    Div(LuaDiv),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct LuaDiv {
    children: Vec<LuaNode>,
    flex: bool,
    bg: Option<String>,
}

#[allow(dead_code)]
impl LuaNode {
    fn as_gpui(&self) -> Div {
        match &self {
            LuaNode::Div(d) => d.as_gpui(),
            LuaNode::Text(s) => div().child(SharedString::from(s)),
        }
    }
}

#[allow(dead_code)]
impl LuaDiv {
    fn as_gpui(&self) -> Div {
        div()
            .when_some(self.bg.clone(), |el, val| {
                el.bg(rgba(val.clone().trim().parse().unwrap()))
            })
            .when(self.flex, |el| el.flex())
            .when(!self.children.is_empty(), |el| {
                el.children(self.children.iter().map(|node| node.as_gpui()))
            })
    }
}

impl FromLua for LuaNode {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => {
                if let Ok(node) = ud.take::<LuaNode>() {
                    return Ok(node);
                }
                Err(mlua::Error::FromLuaConversionError {
                    from: "userdata",
                    to: "LuaNode".to_string(),
                    message: Some("unknown userdata".into()),
                })
            }
            mlua::Value::String(s) => Ok(LuaNode::Text(s.to_str()?.to_string())),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LuaNode".to_string(),
                message: None,
            }),
        }
    }
}

impl LuaUserData for LuaDiv {}
impl LuaUserData for LuaNode {}

#[allow(dead_code)]
fn test_lua(lua: &Lua) -> LuaTable {
    let ui = lua.create_table().unwrap();
    let func = lua
        .create_function(|_, text: String| Ok(LuaNode::Text(text)))
        .unwrap();
    ui.set("text", func).unwrap();
    ui
}

#[cfg(test)]
mod tests {
    use mlua::Lua;

    use crate::{LuaNode, test_lua};

    #[test]
    fn test_lua_api() {
        let lua = Lua::new();
        let table = test_lua(&lua);
        lua.globals().set("ui", table).unwrap();
        let res: LuaNode = lua.load(r#"return ui.text("penis")"#).eval().unwrap();
        assert_eq!(LuaNode::Text("penis".to_string()), res);
    }
}
