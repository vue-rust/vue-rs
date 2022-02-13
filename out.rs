#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use vue::Vue;
fn main() {}
struct MyProps {
    #[vue(prop)]
    id: String,
    #[vue(sync)]
    b: bool,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for MyProps {
    #[inline]
    fn default() -> MyProps {
        MyProps {
            id: ::core::default::Default::default(),
            b: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
pub struct JsMyProps(
    #[doc(hidden)] ReactiveMyProps,
    #[doc(hidden)] vue::macros_utils::ReactiveDrop<MyProps>,
);
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribe for JsMyProps {
    fn describe() {
        use wasm_bindgen::__wbindgen_if_not_std;
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(7u32);
        inform(77u32);
        inform(121u32);
        inform(80u32);
        inform(114u32);
        inform(111u32);
        inform(112u32);
        inform(115u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::IntoWasmAbi for JsMyProps {
    type Abi = u32;
    fn into_abi(self) -> u32 {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::WasmRefCell;
        Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::FromWasmAbi for JsMyProps {
    type Abi = u32;
    unsafe fn from_abi(js: u32) -> Self {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<JsMyProps>;
        assert_not_null(ptr);
        let js = Box::from_raw(ptr);
        (*js).borrow_mut();
        js.into_inner()
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::core::convert::From<JsMyProps> for wasm_bindgen::JsValue {
    fn from(value: JsMyProps) -> Self {
        let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __wbg_myprops_new(_: u32) -> u32 {
            {
                ::std::rt::begin_panic("cannot convert to JsValue outside of the wasm target")
            }
        }
        unsafe {
            <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                __wbg_myprops_new(ptr),
            )
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::RefFromWasmAbi for JsMyProps {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::Ref<'static, JsMyProps>;
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<JsMyProps>;
        wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow()
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::RefMutFromWasmAbi for JsMyProps {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RefMut<'static, JsMyProps>;
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<JsMyProps>;
        wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow_mut()
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionIntoWasmAbi for JsMyProps {
    #[inline]
    fn none() -> Self::Abi {
        0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionFromWasmAbi for JsMyProps {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        *abi == 0
    }
}
#[automatically_derived]
impl JsMyProps {
    pub fn new() -> Self {
        #[automatically_derived]
        const __wasm_bindgen_generated_JsMyProps_new__const: () = {
            pub extern "C" fn __wasm_bindgen_generated_JsMyProps_new(
            ) -> <JsMyProps as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                let _ret = { JsMyProps::new() };
                <JsMyProps as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
            }
            ()
        };
        let reactive = vue::Reactive::<MyProps>::new();
        vue::macros_utils::reactive_fut(
            my_props_update(ReactiveMyProps(reactive.clone())),
            reactive.clone(),
        );
        Self(
            ReactiveMyProps(reactive.clone()),
            vue::macros_utils::ReactiveDrop(reactive),
        )
    }
    pub fn set_component_active(&mut self, value: bool) {
        #[automatically_derived]
        const __wasm_bindgen_generated_JsMyProps_set_component_active__const: () = {
            pub extern "C" fn __wasm_bindgen_generated_JsMyProps_set_component_active(
                me: u32,
                arg1: <bool as wasm_bindgen::convert::FromWasmAbi>::Abi,
            ) -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                let _ret = {
                    let mut me = unsafe {
                        <JsMyProps as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                            me,
                        )
                    };
                    let me = &mut *me;
                    let arg1 =
                        unsafe { <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1) };
                    me.set_component_active(arg1)
                };
                <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
            }
            ()
        };
        self.0.macro_set_active(value);
    }
    pub fn set_data(&mut self, data: wasm_bindgen::JsValue) {
        #[automatically_derived]
        const __wasm_bindgen_generated_JsMyProps_set_data__const: () = {
            pub extern "C" fn __wasm_bindgen_generated_JsMyProps_set_data(
                me: u32,
                arg1: <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi,
            ) -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                let _ret = {
                    let mut me = unsafe {
                        <JsMyProps as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                            me,
                        )
                    };
                    let me = &mut *me;
                    let arg1 = unsafe {
                        <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            arg1,
                        )
                    };
                    me.set_data(arg1)
                };
                <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
            }
            ()
        };
        self.0.macro_set_target(data);
    }
    pub fn id(&self) -> String {
        #[automatically_derived]
        const __wasm_bindgen_generated_JsMyProps_id__const: () = {
            pub extern "C" fn __wasm_bindgen_generated_JsMyProps_id(
                me: u32,
            ) -> <String as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                let _ret = {
                    let me = unsafe {
                        <JsMyProps as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(me)
                    };
                    let me = &*me;
                    me.id()
                };
                <String as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
            }
            ()
        };
        self.0.map(|p| p.id.clone())
    }
    pub fn set_id(&mut self, value: String) {
        #[automatically_derived]
        const __wasm_bindgen_generated_JsMyProps_set_id__const: () = {
            pub extern "C" fn __wasm_bindgen_generated_JsMyProps_set_id(
                me: u32,
                arg1: <String as wasm_bindgen::convert::FromWasmAbi>::Abi,
            ) -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                let _ret = {
                    let mut me = unsafe {
                        <JsMyProps as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                            me,
                        )
                    };
                    let me = &mut *me;
                    let arg1 =
                        unsafe { <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1) };
                    me.set_id(arg1)
                };
                <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
            }
            ()
        };
        self.0.set_id(value)
    }
    pub fn b(&self) -> bool {
        #[automatically_derived]
        const __wasm_bindgen_generated_JsMyProps_b__const: () = {
            pub extern "C" fn __wasm_bindgen_generated_JsMyProps_b(
                me: u32,
            ) -> <bool as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                let _ret = {
                    let me = unsafe {
                        <JsMyProps as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(me)
                    };
                    let me = &*me;
                    me.b()
                };
                <bool as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
            }
            ()
        };
        self.0.map(|p| p.b.clone())
    }
    pub fn set_b(&mut self, value: bool) {
        #[automatically_derived]
        const __wasm_bindgen_generated_JsMyProps_set_b__const: () = {
            pub extern "C" fn __wasm_bindgen_generated_JsMyProps_set_b(
                me: u32,
                arg1: <bool as wasm_bindgen::convert::FromWasmAbi>::Abi,
            ) -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
                let _ret = {
                    let mut me = unsafe {
                        <JsMyProps as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                            me,
                        )
                    };
                    let me = &mut *me;
                    let arg1 =
                        unsafe { <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1) };
                    me.set_b(arg1)
                };
                <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
            }
            ()
        };
        self.0.set_b(value)
    }
}
#[automatically_derived]
pub fn js_my_props_create() -> wasm_bindgen::JsValue {
    wasm_bindgen::JsValue::from_serde(&vue::macros_utils::CreateConfig {
        props: <[_]>::into_vec(box ["id", "b"]),
    })
    .expect("CreateConfig")
}
#[automatically_derived]
const __wasm_bindgen_generated_myPropsCreate__const: () = {
    #[automatically_derived]
    pub extern "C" fn __wasm_bindgen_generated_myPropsCreate(
    ) -> <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = { js_my_props_create() };
        <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
    ()
};
#[automatically_derived]
struct ReactiveMyProps(#[doc(hidden)] vue::Reactive<MyProps>);
#[automatically_derived]
impl ReactiveMyProps {
    pub fn set_id(&mut self, value: String) {
        self.0.macro_set(
            |p| &mut p.id,
            value,
            "id",
            vue::macros_utils::ReactiveTy::Prop,
        );
    }
    pub fn set_b(&mut self, value: bool) {
        self.0.macro_set(
            |p| &mut p.b,
            value,
            "b",
            vue::macros_utils::ReactiveTy::Sync,
        );
    }
}
impl std::ops::Deref for ReactiveMyProps {
    type Target = vue::Reactive<MyProps>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
async fn my_props_update(props: ReactiveMyProps) {
    props.map(|props| if props.id.is_empty() {})
}
