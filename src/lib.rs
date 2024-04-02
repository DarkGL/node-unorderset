use neon::prelude::*;
use std::collections::HashSet;
use std::cell::RefCell;

struct UnorderedSet {
    inner: RefCell<HashSet<String>>,
}

impl UnorderedSet {
    fn new() -> Self {
        UnorderedSet {
            inner: RefCell::new(HashSet::new()),
        }
    }

    fn insert(&self, value: String) {
        self.inner.borrow_mut().insert(value);
    }

    fn has(&self, value: &str) -> bool {
        self.inner.borrow().contains(value)
    }

    fn iterate(&self) -> Vec<String> {
        self.inner.borrow().iter().cloned().collect()
    }
}

impl Finalize for UnorderedSet {}

impl UnorderedSet {
    fn js_new(mut cx: FunctionContext) -> JsResult<JsBox<UnorderedSet>> {
        let set = UnorderedSet::new();

        Ok(cx.boxed(set))
    }

    fn js_add(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let value = cx.argument::<JsString>(0)?.value(&mut cx);

        let this = cx.this::<JsBox<UnorderedSet>>()?;

        this.insert(value);

        Ok(cx.undefined())
    }

    fn js_has(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let value = cx.argument::<JsString>(0)?.value(&mut cx);
        let result = cx.this::<JsBox<UnorderedSet>>()?.has(&value);
        Ok(cx.boolean(result))
    }

    fn js_iterate(mut cx: FunctionContext) -> JsResult<JsArray> {
        let result = {
            let iter = cx.this::<JsBox<UnorderedSet>>()?.iterate();
            let js_array = JsArray::new(&mut cx, iter.len());
            for (i, obj) in iter.into_iter().enumerate() {
                let js_str = cx.string(obj);
                js_array.set(&mut cx, i as u32, js_str).unwrap();
            }
            js_array
        };
        Ok(result)
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("unorderedSetNew", UnorderedSet::js_new)?;
    cx.export_function("unorderedSetAdd", UnorderedSet::js_add)?;
    cx.export_function("unorderedSetHas", UnorderedSet::js_has)?;
    cx.export_function("unorderedSetIterate", UnorderedSet::js_iterate)?;

    Ok(())
}
