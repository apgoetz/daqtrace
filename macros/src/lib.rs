// use syn;
// use quote;
use proc_macro::TokenStream;
use quote::{quote,quote_spanned};
use syn::parse_macro_input;
use syn::spanned::Spanned;
//use proc_macro2;
mod parser;

#[proc_macro]
pub fn trace(input: TokenStream) -> TokenStream {
    
    let ts = parse_macro_input!(input as parser::TraceStatement);
    let parser::TraceStatement(sink, key, _labels, exprs) = ts;

    let e_name = key.to_string();

    // some parts of the macro can fail, and we want the span to be
    // set to the first parameter, since we know that is what causes
    // the problems
    let sink_span = sink.span();

    // first we make a sink
    let make_sink =
        quote_spanned!(sink_span=>
                       
                       let sink = &#sink;
                       let ts = ::daqtrace::Sink::timestamp(sink);
    );

    // at the end, we write the message
    let write_msg =
        quote_spanned!(sink_span=>
                       ::daqtrace::Sink::write(sink, key, &msg);      
    );
    
    quote! ({

        use ::daqtrace::Sink;
        
        #[link_section = ".daqtrace"]
        #[export_name = #e_name ]
        static DAQTRACE_PLACEHOLDER: u8 = 0;

        let key = &DAQTRACE_PLACEHOLDER as *const u8 as u16;

        #make_sink

        let msg = (key,ts,#(#exprs),*);

        #write_msg

    }).into()
}
