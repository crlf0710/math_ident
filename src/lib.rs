extern crate proc_macro;

use proc_macro::{Ident, Literal, Span, TokenStream, TokenTree};

fn literal_never_escaped_str_value_polyfill(literal: &Literal) -> Result<String, ()> {
    let mut s = literal.to_string();
    if !s.starts_with('"') {
        return Err(());
    }
    s.remove(0);
    if !s.ends_with('"') {
        return Err(());
    }
    s.pop();
    if s.contains('\\') {
        return Err(());
    }
    Ok(s)
}

fn is_mathematically_valid(c: char, is_start: bool) -> Option<&'static str> {
    let s = match c {
        '\u{221E}' => {
            if is_start {
                "INFINITY_"
            } else {
                "_INFINITY_"
            }
        }
        '\u{2207}' => {
            if is_start {
                "NABLA_"
            } else {
                "_NABLA_"
            }
        }
        '\u{2202}' => {
            if is_start {
                "PARTIALDIFF_"
            } else {
                "_PARTIALDIFF_"
            }
        }
        '\u{1D6C1}' => {
            if is_start {
                "NABLAǀbold_"
            } else {
                "_NABLAǀbold_"
            }
        }
        '\u{1D6DB}' => {
            if is_start {
                "PARTIALDIFFǀbold_"
            } else {
                "_PARTIALDIFFǀbold_"
            }
        }
        '\u{1D6FB}' => {
            if is_start {
                "NABLAǀitalic_"
            } else {
                "_NABLAǀitalic_"
            }
        }
        '\u{1D715}' => {
            if is_start {
                "PARTIALDIFFǀitalic_"
            } else {
                "_PARTIALDIFFǀitalic_"
            }
        }
        '\u{1D735}' => {
            if is_start {
                "NABLAǀbolditalic_"
            } else {
                "_NABLAǀbolditalic_"
            }
        }
        '\u{1D74F}' => {
            if is_start {
                "PARTIALDIFFǀbolditalic_"
            } else {
                "_PARTIALDIFFǀbolditalic_"
            }
        }
        '\u{1D76F}' => {
            if is_start {
                "NABLAǀsansbold_"
            } else {
                "_NABLAǀsansbold_"
            }
        }
        '\u{1D789}' => {
            if is_start {
                "PARTIALDIFFǀsansbold_"
            } else {
                "_PARTIALDIFFǀsansbold_"
            }
        }
        '\u{1D7A9}' => {
            if is_start {
                "NABLAǀsansbolditalic_"
            } else {
                "_NABLAǀsansbolditalic_"
            }
        }
        '\u{1D7C3}' => {
            if is_start {
                "PARTIALDIFFǀsansbolditalic_"
            } else {
                "_PARTIALDIFFǀsansbolditalic_"
            }
        }
        _ => {
            if is_start {
                return None;
            }
            match c {
                '\u{2070}' => "_ZEROǀsuperscript_",
                '\u{00B9}' => "_ONEǀsuperscript_",
                '\u{00B2}' => "_TWOǀsuperscript_",
                '\u{00B3}' => "_THREEǀsuperscript_",
                '\u{2074}' => "_FOURǀsuperscript_",
                '\u{2075}' => "_FIVEǀsuperscript_",
                '\u{2076}' => "_SIXǀsuperscript_",
                '\u{2077}' => "_SEVENǀsuperscript_",
                '\u{2078}' => "_EIGHTǀsuperscript_",
                '\u{2079}' => "_NINEǀsuperscript_",
                '\u{207A}' => "_PLUSǀsuperscript_",
                '\u{207B}' => "_MINUSǀsuperscript_",
                '\u{207C}' => "_EQUALSǀsuperscript_",
                '\u{207D}' => "_LEFTPARENTHESISǀsuperscript_",
                '\u{207E}' => "_RIGHTPARENTHESISǀsuperscript_",

                '\u{2080}' => "_ZEROǀsubscript_",
                '\u{2081}' => "_ONEǀsubscript_",
                '\u{2082}' => "_TWOǀsubscript_",
                '\u{2083}' => "_THREEǀsubscript_",
                '\u{2084}' => "_FOURǀsubscript_",
                '\u{2085}' => "_FIVEǀsubscript_",
                '\u{2086}' => "_SIXǀsubscript_",
                '\u{2087}' => "_SEVENǀsubscript_",
                '\u{2088}' => "_EIGHTǀsubscript_",
                '\u{2089}' => "_NINEǀsubscript_",
                '\u{208A}' => "_PLUSǀsubscript_",
                '\u{208B}' => "_MINUSǀsubscript_",
                '\u{208C}' => "_EQUALSǀsubscript_",
                '\u{208D}' => "_LEFTPARENTHESISǀsubscript_",
                '\u{208E}' => "_RIGHTPARENTHESISǀsubscript_",
                _ => return None,
            }
        }
    };
    Some(s)
}

fn proc_macro_compile_error(message: &str, span: Span) -> TokenStream {
    use proc_macro::{Delimiter, Group, Punct, Spacing};
    TokenStream::from_iter([
        TokenTree::Punct({
            let mut punct = Punct::new(':', Spacing::Joint);
            punct.set_span(span);
            punct
        }),
        TokenTree::Punct({
            let mut punct = Punct::new(':', Spacing::Alone);
            punct.set_span(span);
            punct
        }),
        TokenTree::Ident(Ident::new("core", span)),
        TokenTree::Punct({
            let mut punct = Punct::new(':', Spacing::Joint);
            punct.set_span(span);
            punct
        }),
        TokenTree::Punct({
            let mut punct = Punct::new(':', Spacing::Alone);
            punct.set_span(span);
            punct
        }),
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct({
            let mut punct = Punct::new('!', Spacing::Alone);
            punct.set_span(span);
            punct
        }),
        TokenTree::Group({
            let mut group = Group::new(Delimiter::Brace, {
                TokenStream::from_iter([TokenTree::Literal({
                    let mut string = Literal::string(message);
                    string.set_span(span);
                    string
                })])
            });
            group.set_span(span);
            group
        }),
    ])
}

#[proc_macro]
pub fn math_ident(input: TokenStream) -> TokenStream {
    let mut input_iter = input.into_iter();
    let Some(ident_tt) = input_iter.next() else {
        return proc_macro_compile_error("should use string literal as identifier", Span::call_site());
    };
    let true = input_iter.next().is_none() else {
        return proc_macro_compile_error(
            "should use only one single string literal as identifier",
            ident_tt.span(),
        );
    };
    let TokenTree::Literal(ident_literal) = ident_tt else {
        return proc_macro_compile_error("should use string literal as identifier", ident_tt.span());
    };
    let Ok(ident_str) = literal_never_escaped_str_value_polyfill(&ident_literal) else {
        return proc_macro_compile_error(
            "should use unescaped string literal as identifier",
            ident_literal.span(),
        );
    };
    let true = !ident_str.is_empty() else {
        return proc_macro_compile_error(
            "should use non-empty string literal as identifier",
            ident_literal.span(),
        );
    };
    let mut result_ident_str = String::new();
    result_ident_str.reserve(ident_str.len());
    for (char_idx, char) in ident_str.chars().enumerate() {
        let is_start = char_idx == 0;
        let casually_valid = (if is_start {
            unicode_ident::is_xid_start
        } else {
            unicode_ident::is_xid_continue
        })(char);
        if casually_valid {
            result_ident_str.push(char);
            continue;
        }
        if let Some(replacement) = is_mathematically_valid(char, is_start) {
            result_ident_str.push_str(replacement);
            continue;
        }
        return proc_macro_compile_error(
            "should only use identifier character and math character in string literal as identifier",
            ident_literal.span(),
        );
    }
    let ident = Ident::new(&result_ident_str, ident_literal.span());
    TokenStream::from_iter(Some(TokenTree::Ident(ident)))
}
