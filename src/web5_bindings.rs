use neon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Web5DID {
    id: String,
}

fn generate_web5_did(mut cx: FunctionContext) -> JsResult<JsString> {
    let did = cx.execute_script("
        const { Web5 } = require('@web5/api');
        const web5 = new Web5();
        return web5.did.create('key');
    ")?;

    let did_obj: Web5DID = serde_json::from_str(&did.value(&mut cx).to_string(&mut cx)?)?;
    Ok(cx.string(did_obj.id))
}

fn validate_web5_did(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let did = cx.argument::<JsString>(0)?.value(&mut cx);
    let is_valid = cx.execute_script(&format!("
        const {{ DID }} = require('@web5/api');
        try {{
            DID.parse('{}');
            return true;
        }} catch (e) {{
            return false;
        }}
    ", did))?;

    Ok(cx.boolean(is_valid.value(&mut cx).to_boolean(&mut cx)?))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateWeb5DID", generate_web5_did)?;
    cx.export_function("validateWeb5DID", validate_web5_did)?;
    Ok(())
}
