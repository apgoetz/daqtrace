// use syn;
// use quote;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
//use proc_macro2;
mod parser;

#[proc_macro]
pub fn trace(input: TokenStream) -> TokenStream {
    
    let ts = parse_macro_input!(input as parser::TraceStatement);
    let parser::TraceStatement(sink, id, _labels, exprs) = ts;

    let e_name = id.to_string();
    
    quote! ({

        use daqtrace::Sink;
        
        #[link_section = ".daqtrace"]
        #[export_name = #e_name ]
        static DAQTRACE_PLACEHOLDER: u8 = 0;

        let id = &DAQTRACE_PLACEHOLDER as *const u8 as u16;

        let sink = &#sink;

        let ts = sink.timestamp();

        let msg = (id,ts,#(#exprs),*);

        sink.write(id,&msg);

    }).into()
}

