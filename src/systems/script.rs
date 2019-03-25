#![allow(clippy::module_name_repetitions)]

use std::sync::{Arc, Mutex};

use amethyst::ecs::{ReadExpect, Resources, SystemData};

use crate::settings::Context;

#[derive(Default)]
pub struct ScriptRuntime {
    pub lua: Arc<Mutex<rlua::Lua>>,
}

#[derive(Default)]
pub struct System;

impl<'s> amethyst::ecs::System<'s> for System {
    type SystemData = (ReadExpect<'s, Context>,);

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, _: Self::SystemData) {}
}

#[cfg(test)]
mod tests {
    use rlua::{Function, Lua};

    #[test]
    fn rlua_test_1() {
        let lua = Lua::new();
        lua.context(|lua_ctx| -> rlua::Result<()> {
            lua_ctx
                .load(
                    r#"
                 print("Executed");
                 function add (a, b)
                    return a + b
                 end
                 "#,
                )
                .eval::<()>()
                .unwrap();

            let globals = lua_ctx.globals();
            let add: Function = globals.get("add")?;

            let print: Function = globals.get("print")?;
            print.call::<_, ()>("hello from rust")?;

            let res = add.call::<_, i32>((1, 2))?;
            println!("Got res = {}", res);

            Ok(())
        })
        .unwrap();
    }
}
