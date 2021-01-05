// inner parser for trace! macro: defines new syntax

use syn::{self, Token, punctuated::Punctuated, Error};
use syn::parse::{Parse, ParseStream, Result};


// fields are sink id labels expressions
pub struct TraceStatement (pub syn::Expr, pub syn::Ident, pub Vec<String>, pub Vec<syn::Expr>);

impl Parse for TraceStatement {
    fn parse (input: ParseStream) -> Result<Self> {
        let args: Punctuated<TraceArg, Token![,]> = input.parse_terminated(TraceArg::parse)?;

        if args.len() < 3 {
            return Err(Error::new(input.span(), "trace! macro requires at least 3 parameters"));
        }

        let mut iter = args.into_iter();

        // first parameter is a rust expression representing the sink
        let sink = iter.next().unwrap().expr;

        let id_arg = iter.next().unwrap();
        let id = match id_arg.get_ident() {
            Some(i) => i.clone(),
            None => return Err(Error::new_spanned(id_arg.expr, "trace! macro requires that the trace id be a valid rust identifier"))
        };

        
        let (labels, exprs) : (Vec<_>, Vec<_>) = iter.map(|arg| (arg.label, arg.expr)).unzip();

        for (label, expr) in labels.iter().zip(exprs.iter()) {
            if label.is_none() {
                return Err(Error::new_spanned(expr, "trace! macro requires that all logged expressions be tagged with a label for example, trace!(_,_, my_label_x : 1 + 1); or be a simple expression, for example, trace!(_,_,myvarname) "));
            }
        }
        // convert get rid of options since we have checked them all
        let labels = labels.into_iter().flatten().collect();

        Ok(TraceStatement (sink, id, labels, exprs))

    }
}

// represents new syntax for an expression with a label:
// lablel  : expr
//
// examples:
//
// a : 1
// x : i+1
pub struct LabeledExpr {
    label: syn::Ident,
    _sep: Token![:],
    expr: syn::Expr,
}

impl Parse for LabeledExpr {
    fn parse (input: ParseStream) -> Result<Self> {
        Ok( LabeledExpr {
            label: input.parse()?,
            _sep: input.parse()?,
            expr: input.parse()?,
        })
    }
}

pub struct TraceArg {
    pub label : Option<String>,
    pub expr : syn::Expr,
}

impl TraceArg {
    // get this expr as an ident, if possible
    fn get_ident(&self) -> Option<&syn::Ident> {
        get_ident(&self.expr)
    }
}

impl Parse for TraceArg {
    fn parse (input: ParseStream) -> Result<Self> {
        // if the input has for ident : * then assume it is a labeled param
        if input.peek(syn::Ident) && input.peek2(Token![:]) {
            let le : LabeledExpr = input.parse()?;
            Ok(TraceArg {label : Some(le.label.to_string()),  expr : le.expr})
        } else  {
            // we assume it is an expr. All idents are also exprs
            let expr : syn::Expr = input.parse()?;
            // if the expr is a plain ident, then we use that as its label
            let label = get_ident(&expr).map(|i| i.to_string());
            Ok(TraceArg {label, expr})
        }
    }
}

fn get_ident(expr : &syn::Expr) -> Option<&syn::Ident> {
    match expr {
        syn::Expr::Path(e) => e.path.get_ident(),
        _ => None
    }
}

#[test]
fn test_get_ident() -> Result<()> {
    let test_vec = [
        ("asdf", Some("asdf")),
        ("&asdf", None),
        ("1+1", None),
        ("self", Some("self")),
    ];

    for (input, expected) in test_vec.iter() {
        let expr : syn::Expr = syn::parse_str(input)?;
        let actual = get_ident(&expr).as_ref().map(|&i| i.to_string());
        assert_eq!(expected, &actual.as_deref());
    }
    Ok(())
}

#[test]
fn ident() -> Result<()> {
//    let idents = ["a", "b", "c"];
    let TraceStatement(_,id,labels,_) = syn::parse_str("a,b,c")?;
    assert_eq!(id.to_string(), "b");

    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0], "c");
    Ok(())
}

#[test]
fn labeled_expr() -> Result<()>{
    let le = syn::parse_str::<LabeledExpr>("abc : 1+2+3")?;
    assert_eq!(le.label.to_string(), "abc");
    Ok(())
}
