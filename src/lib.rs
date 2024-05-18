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
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("unorderedSetNew", UnorderedSet::js_new)?;
    cx.export_function("unorderedSetAdd", UnorderedSet::js_add)?;
    cx.export_function("unorderedSetHas", UnorderedSet::js_has)?;

    Ok(())
}
