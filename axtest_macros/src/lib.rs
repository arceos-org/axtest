extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Ident};

#[proc_macro_attribute]
pub fn axtest(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: ItemFn = syn::parse_macro_input!(item as ItemFn);
    let fn_name = &item.sig.ident;
    let fn_name_str = fn_name.to_string();
    
    // 解析属性参数 (目前支持 should_panic, execution_mode)
    let should_panic = if !attr.is_empty() {
        if let Ok(attr_str) = syn::parse::<syn::LitStr>(attr.clone()) {
            attr_str.value().contains("should_panic")
        } else {
            false
        }
    } else {
        false
    };
    
    let execution_mode = if !attr.is_empty() {
        if let Ok(attr_str) = syn::parse::<syn::LitStr>(attr.clone()) {
            let attr_content = attr_str.value();
            if attr_content.contains("ignore") {
                quote!(axtest::AxTestExecutionMode::Ignore)
            } else if attr_content.contains("custom") {
                quote!(axtest::AxTestExecutionMode::Custom)
            } else if attr_content.contains("user") {
                quote!(axtest::AxTestExecutionMode::User)
            } else {
                quote!(axtest::AxTestExecutionMode::Standard)
            }
        } else {
            quote!(axtest::AxTestExecutionMode::Standard)
        }
    } else {
        quote!(axtest::AxTestExecutionMode::Standard)
    };
    
    // 生成唯一的静态变量名
    let static_name = Ident::new(&format!("__axtest_descriptor_{}", fn_name), fn_name.span());
    
    // 生成增强后的函数代码
    let expanded = quote! {
        #item
        
        #[used]
        #[unsafe(link_section = ".axtest")]
        #[allow(non_upper_case_globals)]
        static #static_name: axtest::AxTestDescriptor = axtest::AxTestDescriptor::new(
            #fn_name_str,
            module_path!(),
            #fn_name,
            #should_panic,
            #execution_mode,
        );
    };
    
    TokenStream::from(expanded)
}
