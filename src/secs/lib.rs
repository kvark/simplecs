#![feature(plugin_registrar, quote, rustc_private)]

extern crate rustc;
extern crate syntax;

use std::ops::Deref;
use syntax::{ast, ext, codemap};
use syntax::abi::Abi;
use syntax::owned_slice::OwnedSlice;
use syntax::parse::token;
use syntax::ptr::P;

#[plugin_registrar]
pub fn registrar(reg: &mut rustc::plugin::Registry) {
    reg.register_syntax_extension(token::intern("secs"),
        ext::base::Decorator(Box::new(SyntaxEcs))
    );
}

#[derive(Copy)]
pub struct SyntaxEcs;

impl ext::base::ItemDecorator for SyntaxEcs {
    fn expand(&self, context: &mut ext::base::ExtCtxt, span: codemap::Span,
              meta_item: &ast::MetaItem, item: &ast::Item,
              push: &mut FnMut(P<ast::Item>))
    {
        use syntax::ext::build::AstBuilder;
        //1: extract the path to `id` crate
        let id_path = match meta_item.node {
            ast::MetaList(_, ref list) if list.len() == 1 => {
                match list[0].node {
                    ast::MetaWord(ref path) => context.ident_of(path.deref()),
                    _ => {
                        context.span_err(span, "the `id` path should be a word");
                        return
                    }
                }
            },
            _ => {
                context.span_err(span, "use as `#[secs(id_path)]`");
                return
            }
        };
        //2: extract the prototype definition
        let (definition, generics) = match item.node {
            ast::ItemStruct(ref def, ref gen) => (def.deref(), gen),
            _ => {
                context.span_err(span, "#[secs] only works with structs");
                return
            }
        };
        let gen_types: Vec<_> = generics.ty_params.as_slice().iter().map(|typaram|
            context.ty_ident(typaram.span, typaram.ident)
        ).collect();
        //3: generate `struct Entity`
        let entity_ident = context.ident_of("Entity");
        let entity_ty = context.ty_path(context.path_all(span, false,
            vec![entity_ident], Vec::new(), gen_types.clone(), Vec::new()
        ));
        push(P(ast::Item {
            ident: entity_ident,
            attrs: Vec::new(),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemStruct(
                P(ast::StructDef {
                    fields: definition.fields.iter().map(|field| {
                        let ref ty = field.node.ty;
                        let mut f = field.clone();
                        f.node.attrs.clear();
                        f.node.ty = quote_ty!(context, Option<$id_path::Id<$ty>>);
                        f
                    }).collect(),
                    ctor_id: None,
                }),
                generics.clone()
            ),
            vis: item.vis,
            span: span,
        }));
        //4a: generate `struct Components`
        let comp_ident = context.ident_of("Components");
        let comp_ty = context.ty_path(context.path_all(span, false,
            vec![comp_ident], Vec::new(), gen_types.clone(), Vec::new()
        ));
        push(P(ast::Item {
            ident: comp_ident,
            attrs: Vec::new(),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemStruct(
                P(ast::StructDef {
                    fields: definition.fields.iter().map(|field| {
                        let ref ty = field.node.ty;
                        let mut f = field.clone();
                        f.node.attrs.clear();
                        f.node.ty = quote_ty!(context, $id_path::Array<$ty>>);
                        f
                    }).collect(),
                    ctor_id: None,
                }),
                generics.clone()
            ),
            vis: item.vis,
            span: span,
        }));
        //4b: generate `impl Components`
        let new_array = quote_expr!(context, $id_path::Array::new());
        fn make_generics() -> ast::Generics {
            ast::Generics {
                lifetimes: Vec::new(),
                ty_params: OwnedSlice::empty(),
                where_clause: ast::WhereClause {
                    id: ast::DUMMY_NODE_ID,
                    predicates: Vec::new(),
                },
            }
        }
        push(P(ast::Item {
            ident: comp_ident,
            attrs: Vec::new(),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemImpl(
                ast::Unsafety::Normal,
                ast::ImplPolarity::Positive,
                generics.clone(),
                None,   //TraitRef
                comp_ty.clone(),
                //method ::new() -> Components
                Some(P(ast::ImplItem {
                    id: ast::DUMMY_NODE_ID,
                    ident: context.ident_of("new"),
                    vis: item.vis,
                    attrs: Vec::new(),
                    node: ast::MethodImplItem(
                        ast::MethodSig {
                            unsafety: ast::Unsafety::Normal,
                            abi: Abi::Rust,
                            decl: context.fn_decl(Vec::new(), comp_ty.clone()),
                            generics: make_generics(),
                            explicit_self: codemap::Spanned {
                                node: ast::SelfStatic,
                                span: span,
                            },
                        },
                        context.block_expr(context.expr_struct_ident(
                            span, comp_ident, definition.fields.iter().map(|field|
                                context.field_imm(field.span, field.node.ident().unwrap(), new_array.clone())
                            ).collect()
                        ))
                    ),
                    span: span,
                })).into_iter()
                //TODO: add_X, get_X, mut_X, etc
                .collect()
            ),
            vis: item.vis,
            span: span,
        }));
        //5a: generate `struct World`
        let world_ident = context.ident_of("World");
        let world_ty = context.ty_path(context.path_all(span, false,
            vec![world_ident], Vec::new(), gen_types.clone(), Vec::new()
        ));
        push(P(ast::Item {
            ident: world_ident,
            attrs: Vec::new(),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemStruct(
                P(ast::StructDef {
                    fields: vec![
                        codemap::Spanned {
                            node: ast::StructField_ {
                                kind: ast::StructFieldKind::NamedField(
                                    context.ident_of("data"),
                                    ast::Visibility::Public,
                                ),
                                id: ast::DUMMY_NODE_ID,
                                ty: comp_ty,
                                attrs: Vec::new(),
                            },
                            span: span,
                        },
                        codemap::Spanned {
                            node: ast::StructField_ {
                                kind: ast::StructFieldKind::NamedField(
                                    context.ident_of("entities"),
                                    ast::Visibility::Public,
                                ),
                                id: ast::DUMMY_NODE_ID,
                                ty: quote_ty!(context, Vec<$entity_ty>),
                                attrs: Vec::new(),
                            },
                            span: span,
                        },
                    ],
                    ctor_id: None,
                }),
                generics.clone()
            ),
            vis: item.vis,
            span: span,
        }));
        //5b: generate `impl World`
        push(P(ast::Item {
            ident: world_ident,
            attrs: Vec::new(),
            id: ast::DUMMY_NODE_ID,
            node: ast::ItemImpl(
                ast::Unsafety::Normal,
                ast::ImplPolarity::Positive,
                generics.clone(),
                None,   //TraitRef
                world_ty.clone(),
                vec![
                    P(ast::ImplItem {
                        id: ast::DUMMY_NODE_ID,
                        ident: context.ident_of("new"),
                        vis: item.vis,
                        attrs: Vec::new(),
                        node: ast::MethodImplItem(
                            ast::MethodSig {
                                unsafety: ast::Unsafety::Normal,
                                abi: Abi::Rust,
                                decl: context.fn_decl(Vec::new(), world_ty.clone()),
                                generics: make_generics(),
                                explicit_self: codemap::Spanned {
                                    node: ast::SelfStatic,
                                    span: span,
                                },
                            },
                            context.block_expr(quote_expr!(context,
                                World {
                                    data: Components::new(),
                                    entities: Vec::new(),
                                }
                            ))
                        ),
                        span: span,
                    })
                ]
            ),
            vis: item.vis,
            span: span,
        }));
    }
}
