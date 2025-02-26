use chumsky::chain::Chain;
use inkwell::{
    context::{Context, ContextRef},
    llvm_sys::prelude::LLVMModuleRef,
    memory_buffer::MemoryBuffer,
    module::{Linkage, Module},
};

use crate::parser::{self, ParseError};

#[derive(Debug)]
pub enum CompileError<'a> {
    ParseError(ParseError<'a>),
}

pub struct CompiledModule<'a> {
    module: Module<'a>,
    context: &'a Context,
}

// pub fn compile_file<'a>(
//     module_name: &str,
//     file: &'a str,
// ) -> Result<(Context, MemoryBuffer), CompileError<'a>> {
//     let file = parser::parse(file).map_err(|err| CompileError::ParseError(err))?;
//     let context = Context::create();
//     let module = {
//         let module = context.create_module(module_name);
//         let builder = context.create_builder();
//         // let mut compiled_module = CompiledModule { module, context };
//
//         // let test_fn_type = context.i8_type().fn_type(&[], false);
//         // let main_fn = module.add_function("main", main_fn_type, Some(Linkage::External));
//         // let main_fn_block = context.append_basic_block(main_fn, "entry");
//
//         // let main_fn_type = context.i8_type().fn_type(&[], false);
//         // let main_fn = module.add_function("main", main_fn_type, None);
//         // let main_fn_block = context.append_basic_block(main_fn, "entry");
//
//         for definition in file.definitions {
//             // builder.position_at_end(main_fn_block);
//             compile_definition(&module, &builder, definition);
//             // let sum = builder
//             //     .build_int_add(
//             //         context.i8_type().const_int(15, false),
//             //         context.i8_type().const_int(15, false),
//             //         "sum",
//             //     )
//             //     .unwrap();
//             // builder.build_return(Some(&sum));
//         }
//         module.write_bitcode_to_memory()
//     };
//
//     Ok((context, module))
// }

// fn compile_definition<'a, 'b>(
//     module: &Module<'a>,
//     builder: &inkwell::builder::Builder<'b>,
//     definition: crate::ast::TopLevelDefinition,
// ) -> Result<(), CompileError<'a>> {
//     let context = module.get_context();
//     match definition {
//         crate::ast::TopLevelDefinition::Binding { lhs, rhs } => match rhs {
//             crate::ast::Literal::String(_) => todo!(),
//             crate::ast::Literal::Int(_) => todo!(),
//             crate::ast::Literal::Float(_) => todo!(),
//             crate::ast::Literal::Bool(_) => todo!(),
//             crate::ast::Literal::Function {
//                 args,
//                 ret_type,
//                 body,
//             } => {
//                 let fn_type = context.i64_type().fn_type(
//                     &args
//                         .iter()
//                         .map(|arg| context.i64_type().into())
//                         .collect::<Vec<_>>()[..],
//                     false,
//                 );
//                 let r#fn = module.add_function(&lhs.name, fn_type, Some(Linkage::External));
//                 let fn_block = context.append_basic_block(r#fn, "entry");
//                 builder.position_at_end(fn_block);
//                 for (i, expr) in body.iter().enumerate() {
//                     let value = compile_expr(&module, &builder, expr)?;
//                     if i == body.len() - 1 {
//                         builder.build_return(Some(value));
//                     }
//                 }
//
//                 builder.build_return(Some(&context.i64_type().const_zero()));
//             }
//         },
//         crate::ast::TopLevelDefinition::Extern { name, rhs } => todo!(),
//     }
//     Ok(())
// }
