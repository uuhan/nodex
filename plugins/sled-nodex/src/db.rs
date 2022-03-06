use nodex::prelude::*;
use sled::*;
use std::sync::Arc;

type SledInner = Arc<Db>;

pub fn class(env: NapiEnv) -> NapiResult<JsClass> {
    JsClass::new(
        env,
        "sled",
        move |mut this, path: JsString| {
            let path = path.get()?;

            let db = sled::Config::new()
                .path(path)
                .open().unwrap();

            this.wrap(Arc::new(db), move |_, _| Ok(()))?;

            this.set("insert", env.func(move |this, (key, value): (JsArrayBuffer, JsArrayBuffer)| {
                let env = this.env();
                let db = if let Some(inner) = this.unwrap::<SledInner>()? {
                    inner.clone()
                } else {
                    env.throw_error("database missing")?;
                    return env.undefined().map(|u| u.value());
                };

                if let Some(value) = db.insert(
                    key.buffer()?,
                    value.buffer()?).unwrap()
                {
                    env.arraybuffer(value).map(|buffer| buffer.value())
                } else {
                    env.null().map(|u| u.value())
                }
            })?)?;

            this.set("get", env.func(move |this, key: JsArrayBuffer| {
                let env = this.env();
                let db = if let Some(inner) = this.unwrap::<SledInner>()? {
                    inner.clone()
                } else {
                    env.throw_error("database missing")?;
                    return env.undefined().map(|u| u.value());
                };

                let key = key.buffer()?;
                if let Some(value) = db.get(key).unwrap() {
                    env.arraybuffer(value).map(|buffer| buffer.value())
                } else {
                    env.null().map(|u| u.value())
                }
            })?)?;

            this.undefined()
        },
        &[],
    )
}
