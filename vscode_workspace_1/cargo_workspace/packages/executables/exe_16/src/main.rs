// region: My Region 1

// include!("../../../sared/lints.rs");

// #![allow(unknown_lints)]

#![warn(absolute_paths_not_starting_with_crate)]
// #![warn(box_pointers)]
#![warn(deprecated_in_future)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(ffi_unwind_calls)]
// #![warn(fuzzy_provenance_casts)]
#![warn(invalid_reference_casting)]
#![warn(keyword_idents)]
#![warn(let_underscore_drop)]
// #![warn(lossy_provenance_casts)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
// #![warn(missing_copy_implementations)]
// #![warn(missing_debug_implementations)]
// #![warn(missing_docs)]
// #![warn(multiple_supertrait_upcastable)]
// #![warn(must_not_suspend)]
#![warn(non_ascii_idents)]
// #![warn(non_exhaustive_omitted_patterns)]
#![warn(noop_method_call)]
#![warn(pointer_structural_match)]
// #![warn(private_bounds)]
// #![warn(private_interfaces)]
#![warn(rust_2021_incompatible_closure_captures)]
#![warn(rust_2021_incompatible_or_patterns)]
#![warn(rust_2021_prefixes_incompatible_syntax)]
#![warn(rust_2021_prelude_collisions)]
#![warn(single_use_lifetimes)]
// #![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
// #![warn(unnameable_types)]
// #![warn(unreachable_pub)]
// #![warn(unsafe_code)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unstable_features)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_macro_rules)]
// #![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(unused_tuple_struct_fields)]
#![warn(variant_size_differences)]

// #![warn(unknown_lints)]

// #![warn(clippy::pedantic)]
// // #![warn(clippy::nursery)]
// #![warn(clippy::cargo)]
//
// // #![allow(clippy::blanket_clippy_restriction_lints)]
// // #![warn(clippy::restriction)]
// // // #![allow(clippy::blanket_clippy_restriction_lints)]
// // #![warn(clippy::blanket_clippy_restriction_lints)]

// #![warn(clippy::absolute_paths)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::allow_attributes)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::arithmetic_side_effects)]
// #![warn(clippy::as_conversions)]
#![warn(clippy::as_underscore)]
#![warn(clippy::assertions_on_result_states)]
#![warn(clippy::big_endian_bytes)]
#![warn(clippy::bool_to_int_with_if)]
// #![warn(clippy::borrow_as_ptr)]
// #![warn(clippy::cargo_common_metadata)]
#![warn(clippy::case_sensitive_file_extension_comparisons)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::cast_precision_loss)]
#![warn(clippy::cast_ptr_alignment)]
#![warn(clippy::cast_sign_loss)]
#![warn(clippy::checked_conversions)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cloned_instead_of_copied)]
#![warn(clippy::copy_iterator)]
#![warn(clippy::create_dir)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::decimal_literal_representation)]
// #![warn(clippy::default_numeric_fallback)]
#![warn(clippy::default_trait_access)]
#![warn(clippy::default_union_representation)]
#![warn(clippy::deref_by_slicing)]
#![warn(clippy::disallowed_script_idents)]
#![warn(clippy::doc_link_with_quotes)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::else_if_without_else)]
#![warn(clippy::empty_drop)]
#![warn(clippy::empty_enum)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::enum_glob_use)]
// #![warn(clippy::error_impl_error)]
#![warn(clippy::exhaustive_enums)]
// #![warn(clippy::exhaustive_structs)]
#![warn(clippy::exit)]
#![warn(clippy::expect_used)]
#![warn(clippy::expl_impl_clone_on_copy)]
#![warn(clippy::explicit_deref_methods)]
// #![warn(clippy::explicit_into_iter_loop)]
// #![warn(clippy::explicit_iter_loop)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::filter_map_next)]
#![warn(clippy::flat_map_option)]
#![warn(clippy::float_arithmetic)]
#![warn(clippy::float_cmp)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_params_excessive_bools)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::from_iter_instead_of_collect)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::host_endian_bytes)]
#![warn(clippy::if_not_else)]
#![warn(clippy::if_then_some_else_none)]
// #![warn(clippy::ignored_unit_patterns)]
#![warn(clippy::impl_trait_in_params)]
#![warn(clippy::implicit_clone)]
#![warn(clippy::implicit_hasher)]
// #![warn(clippy::implicit_return)]
#![warn(clippy::inconsistent_struct_constructor)]
#![warn(clippy::index_refutable_slice)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::inline_always)]
#![warn(clippy::inline_asm_x86_att_syntax)]
#![warn(clippy::inline_asm_x86_intel_syntax)]
#![warn(clippy::integer_division)]
// #![warn(clippy::into_iter_without_iter)]
#![warn(clippy::invalid_upcast_comparisons)]
#![warn(clippy::items_after_statements)]
#![warn(clippy::iter_not_returning_iterator)]
// #![warn(clippy::iter_without_into_iter)]
#![warn(clippy::large_digit_groups)]
#![warn(clippy::large_futures)]
#![warn(clippy::large_include_file)]
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::large_types_passed_by_value)]
#![warn(clippy::let_underscore_must_use)]
#![warn(clippy::let_underscore_untyped)]
#![warn(clippy::linkedlist)]
#![warn(clippy::little_endian_bytes)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::manual_assert)]
#![warn(clippy::manual_instant_elapsed)]
#![warn(clippy::manual_let_else)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::manual_string_new)]
#![warn(clippy::many_single_char_names)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::match_bool)]
#![warn(clippy::match_on_vec_items)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::match_wild_err_arm)]
#![warn(clippy::match_wildcard_for_single_variants)]
#![warn(clippy::maybe_infinite_iter)]
#![warn(clippy::mem_forget)]
// #![warn(clippy::min_ident_chars)]
#![warn(clippy::mismatching_type_param_order)]
// #![warn(clippy::missing_assert_message)]
// #![warn(clippy::missing_asserts_for_indexing)]
// #![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::missing_enforced_import_renames)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_fields_in_debug)]
#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_trait_methods)]
#![warn(clippy::mixed_read_write_in_expression)]
// #![warn(clippy::mod_module_files)]
#![warn(clippy::module_name_repetitions)]
// #![warn(clippy::modulo_arithmetic)]
#![warn(clippy::multiple_crate_versions)]
#![warn(clippy::multiple_inherent_impl)]
// #![warn(clippy::multiple_unsafe_ops_per_block)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::mut_mut)]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::naive_bytecount)]
#![warn(clippy::needless_bitwise_bool)]
#![warn(clippy::needless_continue)]
#![warn(clippy::needless_for_each)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::needless_raw_string_hashes)]
#![warn(clippy::needless_raw_strings)]
#![warn(clippy::negative_feature_names)]
#![warn(clippy::no_effect_underscore_binding)]
#![warn(clippy::no_mangle_with_rust_abi)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::option_option)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::partial_pub_fields)]
#![warn(clippy::pattern_type_mismatch)]
#![warn(clippy::print_stderr)]
// #![warn(clippy::print_stdout)]
// #![warn(clippy::ptr_as_ptr)]
#![warn(clippy::ptr_cast_constness)]
#![warn(clippy::pub_use)]
#![warn(clippy::pub_with_shorthand)]
#![warn(clippy::pub_without_shorthand)]
#![warn(clippy::question_mark_used)]
#![warn(clippy::range_minus_one)]
#![warn(clippy::range_plus_one)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::redundant_closure_for_method_calls)]
#![warn(clippy::redundant_else)]
#![warn(clippy::redundant_feature_names)]
#![warn(clippy::redundant_type_annotations)]
#![warn(clippy::ref_binding_to_reference)]
#![warn(clippy::ref_option_ref)]
#![warn(clippy::ref_patterns)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::return_self_not_must_use)]
#![warn(clippy::same_functions_in_if_condition)]
#![warn(clippy::same_name_method)]
#![warn(clippy::self_named_module_files)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::semicolon_inside_block)]
// #![warn(clippy::semicolon_outside_block)]
// #![warn(clippy::separated_literal_suffix)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::shadow_same)]
#![warn(clippy::shadow_unrelated)]
// #![warn(clippy::should_panic_without_expect)]
#![warn(clippy::similar_names)]
// #![warn(clippy::single_call_fn)]
// #![warn(clippy::single_char_lifetime_names)]
#![warn(clippy::single_match_else)]
#![warn(clippy::stable_sort_primitive)]
// #![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
// #![warn(clippy::str_to_string)]
#![warn(clippy::string_add)]
#![warn(clippy::string_add_assign)]
// #![warn(clippy::string_lit_chars_any)]
#![warn(clippy::string_slice)]
// #![warn(clippy::string_to_string)]
#![warn(clippy::struct_excessive_bools)]
#![warn(clippy::suspicious_xor_used_as_pow)]
#![warn(clippy::tests_outside_test_module)]
#![warn(clippy::todo)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::transmute_ptr_to_ptr)]
// #![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::try_err)]
#![warn(clippy::unchecked_duration_subtraction)]
// #![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unicode_not_nfc)]
#![warn(clippy::unimplemented)]
// #![warn(clippy::uninlined_format_args)]
#![warn(clippy::unnecessary_box_returns)]
#![warn(clippy::unnecessary_join)]
#![warn(clippy::unnecessary_safety_comment)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unnecessary_wraps)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unnested_or_patterns)]
#![warn(clippy::unreachable)]
// #![warn(clippy::unreadable_literal)]
#![warn(clippy::unsafe_derive_deserialize)]
// #![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unused_async)]
#![warn(clippy::unused_self)]
#![warn(clippy::unwrap_in_result)]
// #![warn( clippy::unwrap_used)]
// #![warn(clippy::use_debug)]
#![warn(clippy::used_underscore_binding)]
#![warn(clippy::verbose_bit_mask)]
#![warn(clippy::verbose_file_reads)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::wildcard_enum_match_arm)]
// #![warn(clippy::wildcard_imports)]
#![warn(clippy::zero_sized_map_values)]

#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// #![allow(unused_imports)]
#![allow(unused_assignments)]

#![allow(clippy::vec_init_then_push)]
#![allow(clippy::assertions_on_constants)]
// #![allow(clippy::missing_const_for_fn)]
// #![allow(clippy::redundant_clone)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::explicit_auto_deref)]

// endregion

// use std::prelude;

// use std::fs::create_dir;

// use ::std::collections::HashMap;

// // This is already listed in "Cargo.toml".
// // This kind of declaration is unnecessary in case Rust edition is greater than 2015.
// extern crate rand;

mod mod_1;
mod mod_2;

fn main()
{
   mod_202309288::fn_1();

   crate::mod_1::fn_202309155();
   self::mod_1::fn_202309155();
   mod_2::fn_202309156();
   mod_2::submod::fn_202309172();

   println!("exe_16 done");
}

mod mod_202309151
{
   pub struct MySruct
   {
      pub field_1: i32,
   }

   pub fn fn_1()
   {
      use ::core::str::FromStr;

      {
         let n: i16 = 2;
         let mut box_1 = Box::new(n);
         let i = ( * box_1 );
         let j = & mut ( * box_1 );
         // let k = (box_1 as dyn std::ops::Deref<i16>).deref();
      }

      {
         let mut box_1 = Box::new(MySruct { field_1: 5 });

         {
            let i = ( & mut * box_1 );
            i.field_1 = 81;
         }

         let j = ( & * box_1 );
         let f = j.field_1;

         assert!(true);
      }

      // 'Box::drop' doesn't seem to null its internally contained pointer.
      // That's not due to optimization, right?
      {
         // use ::core::ops::Deref;

         let box_1 = Box::new(0x123);
         let box_1_ptr_1 = ::core::ops::Deref::deref( & box_1 ) as ( * const i32 );
         let box_1_ptr_2 = ( & * box_1 ) as ( * const i32 );
         let box_1_ptr_3 = ( & box_1 ) as ( * const Box<_> ) as ( * const usize );
         let box_1_ptr_4 = ( & box_1 ) as ( * const Box<_> ) as ( * const * const i32 );
         let box_1_value_1 = unsafe { * box_1_ptr_3 };
         drop(box_1);
         let box_1_value_2 = unsafe { * box_1_ptr_3 };

         assert!(true);
      }

      {
         let box_1 = Box::new(String::from_str("a"));

         // This moves the boxed content.
         // After this, 'box_1' is inaccessible.
         let srting_1 = * box_1;

         // println!("{}", * box_1)
      }
   }
}

mod mod_202309153
{
   pub fn fn_1()
   {
      let mut a = -11111111i64;
      a = a.abs();
      let ptr1 = ( & a ) as ( * const i64 ) as ( * const i16 );
      println!("Data is here: {}", ptr1 as usize);
      let mut ptr2 = ( & mut a ) as ( * mut i64 ) as ( * mut i16 );
   
      unsafe
      {
         // ( * ptr1 ) = 9999i16;
         ( * ptr2 ) = 9999i16;
      }
   
      println!("Updated: {}", a);
      // ptr2 = ptr2 + 1usize;
      ptr2 = 123 as ( * mut i16 );
      println!("{}", ptr2 as (usize));
   
      // let ptr_3 = 0 as ( * mut i16 );
   }
}

mod mod_202309161
{
   pub fn fn_1()
   {
      let s1 /* : & str */ = "Hel";
      // drop(s1);
      let s2: String = String::from(s1);
      let s3 = s2.replace("a1", "b2");
      let s33 = ( & s2 ).replace("a1", "b2");
      let s34 = ( * s2 ).replace("a1", "b2");
      let s4 = s2.to_string();
      let s5 = s2.as_str();
      let s6 = "Hello".to_string();
      drop(s6);
      // let s66 = s6;
      let i1 = 0;
      // drop(i1);
      // let i1copy = i1;
   
      for token in s2.split(',')
      {
         println!("token is {}", token);
      }
   }
}

mod mod_202309162
{
   pub fn fn_1()
   {
      let data = [10,20,30,40,50];
      use_slice(&data[1..4]);
      use_slice(&data);
      use_array(&data);
      use_array_by_value(data);
   }

   fn use_slice(slice:&[i32])
   { 
      println!("length of slice is {}",slice.len());
      println!("{:?}",slice);
   }

   fn use_array(array: & [i32; 5])
   { 
      println!("length of slice is {:?}", array.len());
      println!("{:?}", array);
   }

   fn use_array_by_value(array: [i32; 5])
   { 
   }
}

mod mod_202309163
{
   struct Employee
   {
      name:String,
      company:String,
      age:u32
   }
   
   fn who_is_elder<'a> (emp1: & 'a Employee,emp2: & 'a  Employee) -> & 'a Employee
   {
      if emp1.age>emp2.age {
         emp1
      } else {
         emp2
      }
   }

   pub fn fn_1()
   {
      let emp1 = Employee{
         company:String::from("TutorialsPoint"),
         name:String::from("Mohtashim"),
         age:50
      };
   
      {
         let emp2 = Employee {
            company:String::from("TutorialsPoint"),
            name:String::from("Kannan"),
            age:32
         };
         let elder = who_is_elder(&emp1,&emp2);   
      }
   }
}

mod mod_202309171
{
   #[repr(u8)]
   enum MyEnum { Val1 = 0, Val2, Val3, }
   
   pub fn fn_1()
   {
      let n1 = MyEnum::Val1;
      // let n2 = (MyEnum::A as i32 - 1i32) as MyEnum;
   }
}

mod mod_202309173
{
   #[allow(clippy::arithmetic_side_effects)]
   pub fn fn_1()
   {
      let mut v = Vec::new();
      v.push(20);
      v.push(30);
   
      for i in v.iter_mut()
      {
         println!("{}", i);
         ( * i ) += 1;
      }
   
      // Using of the reference prevents 'v' consumption.
      // for i in ( * ( & v ) )
      for i in ( & v )
   
      {
         println!("{}", i);
      }
   
      println!("{:?}", v);
      println!("{:?}", v);
   
      // This doesn't actually consume 'v'.
      #[allow(clippy::into_iter_on_ref)]
      let v_iter_1 = ( & v ).into_iter();
   
      for i in v_iter_1
      {
         println!("{}", i);
         // ( * i ) += 1;
      }
   
      let v_iter_2 = v.into_iter();
   
      for i in v_iter_2
      {
         println!("{}", i);
         // ( * i ) += 1;
      }
   }
}

mod mod_202309174
{
   pub fn fn_1()
   {
      let mut hs1 = ::std::collections::HashSet::new();
      let bool1 = hs1.insert(789_u32);

      for i in ( & hs1 )
      {
         println!("{}", i);
      }

      // // for i in ( & mut hs1 )
      // for i in ( & mut hs1 ).iter_mut()
      // {
      //    println!("{}", i);
      // }

      for i in ( & hs1 ).iter()
      {
         println!("{}", i);
      }

      for i in hs1.into_iter()
      {
         println!("{}", i);
      }
   }
}

mod mod_202309182
{
   pub fn fn_1()
   {
      {
         let rand_num = ::rand::random::<i32>();
         println!("{}", rand_num);
      }

      {
         let rand_num = ::rand::random::<u128>();
         println!("{}", rand_num);
      }
   }
}

mod mod_202309183
{
   pub fn fn_1()
   {
      let is_even =
         |x: i32|
         {
            x%2==0
         };

      // fn is_even(x: i32) -> bool
      // {
      //    x%2==0
      // }

      // let no = 13i16;
      // println!("{} is even ? {}",no,is_even(no));

      let no2 = 13i32;
      println!("{} is even ? {}",no2,is_even(no2));
   }
}

mod mod_202309184
{
   pub fn fn_1()
   {
      let mut state_codes = std::collections::HashMap::new();
      let option1 = state_codes.insert("KL","Kerala");
      let option2 = state_codes.insert("MH","Maharashtra");
      println!("{:?}", state_codes);
   }
}

mod mod_202309191
{
   pub fn fn_1()
   {
      let cmd_line = std::env::args();
      println!("No of elements in arguments is :{}",cmd_line.len());

      for arg in cmd_line
      {
         println!("[{}]", arg);
      }
   }
}

mod mod_202309201
{
   pub fn fn_1()
   {
      let z = u16::try_from(100000).unwrap();
   }
}

mod mod_202309202
{
   pub fn fn_1()
   {
      // https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
      // The Option<T> enum is so useful that it’s even included in the prelude;
      // you don’t need to bring it into scope explicitly. Its variants are also included in the prelude:
      // you can use Some and None directly without the Option:: prefix.
      let n = None::<i32>;
      let m = Option::<i32>::None;
      let k = Some(5);
      // let k_value = k.unwrap();
      unsafe
      {
         let k_value = k.unwrap_unchecked();
      }

      // let config_max = Some(3u8);
      // let Some(max) = config_max;
      // let max2 = max;
   }
}

mod mod_202309211
{
   pub fn fn_1()
   {
      {
         let x = Struct1 { field_2: 4, ..Default::default() };
         assert!(x.field_1 == -1);
         assert!(x.field_2 == 4);
      }

      {
         let x = Struct1 { field_2: 4, ..Struct1::default() };
         assert!(x.field_1 == -1);
         assert!(x.field_2 == 4);
      }

      {
         let x = Struct2 { field_2: 4, ..Default::default() };
         assert!(x.field_1 == 0);
         assert!(x.field_2 == 4);
      }

      {
         let x = Struct2 { field_2: 4, ..Struct2::default() };
         assert!(x.field_1 == 0);
         assert!(x.field_2 == 4);
      }

      {
         let x = Struct2::default();
         assert!(x.field_1 == 0);
         assert!(x.field_2 == 0);
      }

      {
         let n1 = i32::default();
         // let n2 = i32::new();
      }
   }

   struct Struct1
   {
      field_1: i32,
      field_2: i32,
   }

   impl Default for Struct1
   {
      fn default() -> Self
      {
         return
            Self
            {
               field_1: -1,
               field_2: -1,
            };
      }
   }

   #[derive(Default)]
   struct Struct2
   {
      field_1: i32,
      field_2: i32,
   }
}

mod mod_202309241
{
   use ::std::env;
   // use ::std::error::Error;
   // use ::std::fs;

   pub fn fn_1()
   {
   }

   pub struct Config {
      pub query: String,
      pub file_path: String,
      pub ignore_case: bool,
   }

   impl Config
   {
      pub fn build(
         mut args: impl Iterator<Item = String>,
      ) -> Result<Self, &'static str> {
         let dummy = args.next();

         let query = match args.next() {
               Some(arg) => arg,
               None => return Err("Didn't get a query string"),
         };

         let file_path = match args.next() {
               Some(arg) => arg,
               None => return Err("Didn't get a file path"),
         };

         let ignore_case = env::var("IGNORE_CASE").is_ok();

         Ok(Self {
               query,
               file_path,
               ignore_case,
         })
      }
   }
}

mod mod_202309251
{
   pub fn fn_1()
   {
      #[allow(clippy::std_instead_of_alloc)]
      let a = ::std::rc::Rc::new(123);
      #[allow(clippy::std_instead_of_alloc)]
      let b = ::std::rc::Rc::clone( & a );
      let c = Clone::clone( & a );

      // This is not recommended.
      #[allow(clippy::clone_on_ref_ptr)]
      let d = a.clone();
  }      
}

// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
mod mod_202309252
{
   pub fn fn_1()
   {
      // use ::std::ops::DerefMut;

      let immut_var: ::core::cell::RefCell<Vec<String>> = ::core::cell::RefCell::new(vec![]);

      {
         let mut mut_ref = immut_var.borrow_mut();
         let mut_ref_2 = ( & mut ( * mut_ref ) );
         // let mut_ref_3 = mut_ref.deref_mut();
         mut_ref_2.push(String::from("a1"));
         mut_ref_2.push(String::from("a2"));
         // mut_ref_3.push(String::from("a3"));
         // drop(mut_ref_3);
         // drop(mut_ref_4);
         drop(mut_ref);
         // mut_ref_2.push(String::from("a4"));
         // immut_var.borrow_mut();
      }
      
      // This looks better.
      {
         let mut mut_ref = immut_var.borrow_mut();
         ( * mut_ref ).push(String::from("a5"));
         ( * mut_ref ).push(String::from("a6"));
         drop(mut_ref);
      }
      
      drop(immut_var);
   }
}

// https://doc.rust-lang.org/book/ch16-02-message-passing.html
mod mod_202309261
{
   use ::std::sync::mpsc;
   use ::std::thread;
   use ::core::time::Duration;

   pub fn fn_1()
   {
      let (tx, rx) = mpsc::channel();

      let jh1 =
         thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
         });
  
      let received = rx.recv().unwrap();
      drop(rx);
      println!("Got: {}", received);
   }

   pub fn fn_2()
   {
      let (tx, rx) = mpsc::channel();

      let tx1 = tx.clone();
      let jh1 =
         thread::spawn(move || {
            let vals = vec![
               String::from("hi"),
               String::from("from"),
               String::from("the"),
               String::from("thread"),
            ];
   
            for val in vals {
               tx1.send(val).unwrap();
               thread::sleep(Duration::from_secs(1));
            }
         });
  
      let jh2 =
         thread::spawn(move || {
            let vals = vec![
               String::from("more"),
               String::from("messages"),
               String::from("for"),
               String::from("you"),
            ];
   
            for val in vals {
               tx.send(val).unwrap();
               thread::sleep(Duration::from_secs(1));
            }
         });
  
      for received in rx {
          println!("Got: {}", received);
      }
   }
}

mod mod_202309282
{
   pub fn fn_1()
   {
      let a = "34".parse();
      let age: Result<u8, _> = a;
   }
}

mod mod_202309283
{
   pub fn fn_1()
   {
      let point = (3, 5,);
      print_coordinates( & point );
   }

   fn print_coordinates( & (x, y) : & (i32, i32) )
   {
      println!("Current location: ({}, {})", x, y);
   }

   pub fn fn_2()
   {
      #[allow(irrefutable_let_patterns)]
      if let x = 5
      {
         println!("{}", x);
      }
   }
}

mod mod_202309284
{
   // https://stackoverflow.com/questions/52172090/bitwise-or-in-match
   // https://github.com/camsteffen/switch-statement-rust
   pub fn fn_1()
   {
      let x = 1 | 2;
      println!("{}", x);

      match x
      {
         1 | 2 => println!("one | two"),
         3 => println!("three"),
         _ => println!("anything"),
      }

      match x
      {
         (1 | 2) => println!("one | two"),
         3 => println!("three"),
         _ => println!("anything"),
      }

      match x
      {
         ((1 | 2)) => println!("one | two"),
         3 => println!("three"),
         _ => println!("anything"),
      }

      match x
      {
         | 1 | 2 => println!("one | two"),
         3 => println!("three"),
         _ => println!("anything"),
      }

      match x
      {
         n @ (1 | 2) => println!("one | two"),
         3 => println!("three"),
         _ => println!("anything"),
      }

      match x
      {
         n if n == 1 | 2 => println!("one | two"),
         3 => println!("three"),
         _ => println!("anything"),
      }

      match x
      {
         _ if x == 1 | 2 => println!("one | two"),
         3 => println!("three"),
         _ => println!("anything"),
      }

      // match x
      // {
      //    1 =>
      //    2 => println!("one | two"),
      //    3 => println!("three"),
      //    _ => println!("anything"),
      // }

      // match x
      // {
      //    { 1 | 2 } => println!("one | two"),
      //    3 => println!("three"),
      //    _ => println!("anything"),
      // }

      // match x
      // {
      //    ( || 1 | 2 ).call() => println!("one | two"),
      //    3 => println!("three"),
      //    _ => println!("anything"),
      // }
   }
}

// Listing 18-29: Using @ to bind to a value in a pattern while also testing it
mod mod_202309285
{
   enum Message
   {
      Hello { id: i32 },
   }

   pub fn fn_1()
   {
      let msg = Message::Hello { id: 10 };

      match msg {
         Message::Hello {
            id: id_variable @ 3..=7,
         } => println!("Found an id in range: {}", id_variable),
         Message::Hello { id: id @ 10..=12 } => {
            // println!("Found an id in another range.")
            println!("Found an id in another range. {}", id);
         }

         Message::Hello { id } => println!("Found some other id: {}", id),
      }

      // Use the if let expression to compare the message variable to the Hello variant
      if let Message::Hello { id: x @ 13..=17 } = msg
      {
         // If the pattern matches, print the id
         println!("Found a Hello message with id: {}", x);
      }
      else
      {
         // If the pattern does not match, print a default message
         println!("Not a Hello message");
      }
   }
}

// Listing 19-6: Using unsafe code in the implementation of the split_at_mut function
mod mod_202309286
{
   use std::slice;

   fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
      let len = values.len();
      let ptr = values.as_mut_ptr();
  
      assert!(mid <= len);
  
      unsafe
      {
         (
            slice::from_raw_parts_mut(ptr, mid),
            #[allow(clippy::arithmetic_side_effects)]
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
         )
      }
   }

   pub fn fn_1()
   {
   }
}

// Listing 19-15: Implementing the Add trait on Millimeters to add Millimeters to Meters
mod mod_202309288
{
   use ::core::ops::Add;

   struct Millimeters(u32,);
   struct Meters(u32,);
   
   impl Add<Meters> for Millimeters
   {
      // type Output = Millimeters;
      type Output = Self;
   
      // fn add(self, other: Meters) -> Millimeters
      fn add(self, other: Meters) -> Self::Output
      {
         // Millimeters(self.0 + (other.0 * 1000))
         #[allow(clippy::arithmetic_side_effects)]
         Self(self.0 + (other.0 * 1000))
      }
   }

   impl Add<&Meters> for &Millimeters
   {
      type Output = Millimeters;
   
      fn add(self, other: &Meters) -> Self::Output
      {
         #[allow(clippy::arithmetic_side_effects)]
         Millimeters(self.0 + (other.0 * 1000))
         // <&Millimeters>::Output(self.0 + (other.0 * 1000))
      }
   }

   pub fn fn_1()
   {
      {
         let mlm = Millimeters(7u32);
         let m = Meters(8u32);
         let mlm2 = mlm.add(m);
         // let mlm3 = mlm.add(mlm2);
         // let mlm4 = mlm.add(3u32);

         assert!(true);
      }

      {
         let mlm = Millimeters(7u32);
         let m = Meters(8u32);
         let mlm5 = (&mlm).add(&m);
         let mlm_ref = &mlm;
         let m_ref = &m;
         let mlm6 = mlm_ref.add(m_ref);

         assert!(true);
      }
   }
}

mod mod_202309291
{
   pub fn fn_1()
   {
      type Thunk = Box<dyn Fn() + Send + 'static>;
      let f: Thunk = Box::new(|| println!("hi"));
      // let f2: Thunk = Thunk::new(|| println!("hi"));
   }
}

mod mod_202310021
{
   pub fn fn_1()
   {
      {
         let n_1 = 7;

         // // This syntax is still experimental.
         // let n_1_ptr = & raw const n_1;
      }

      {
         let c = 'Q';

         // A `ref` borrow on the left side of an assignment is equivalent to
         // an `&` borrow on the right side.
         {
            #[allow(clippy::ref_patterns)]
            #[allow(clippy::toplevel_ref_arg)]
            let ref ref_1_c = c;

            let ref_2_c = &c;
         }
      }
   }
}

// https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
// Rust also provides "pretty printing" with {:#?}
mod mod_202310072
{
   #[derive(Debug)]
   struct Person<'a>
   {
      name: &'a str,
      age: u8
   }

   pub fn fn_1()
   {
      let name = "Peter";

      // This is a `u8`. So always explicitly specify numeric literal type.
      let age = 27;

      let peter = Person { name, age };

      println!("{:?}", peter);

      // Pretty print
      println!("{:#?}", peter);
   }
}

mod mod_202310073
{
   // https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
   pub fn fn_1()
   {
      // Tuples can be tuple members.
      let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

      // Tuples are printable.
      println!("tuple of tuples: {:?}", tuple_of_tuples);

      // // But long Tuples (more than 12 elements) cannot be printed.
      // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
      // println!("Too long tuple: {:?}", too_long_tuple);

      // To create one element tuples, the comma is required to tell them apart
      // from a literal surrounded by parentheses.
      println!("One element tuple: {:?}", (5u32,));
      println!("Just an integer: {:?}", (5u32));

      // Tuples can be destructured to create bindings.
      let tuple = (1, "hello", 4.5, true);

      let (a, b, c, d) = tuple;
      println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
   }
}

mod mod_202310074
{
   pub fn fn_1()
   {
   }

   // https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
   // On that page, find:
   // https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
   fn hello(arg: &Option<String>) {
      match arg {
          &Some(ref name) => println!("Hello {}!", name),
          &None => println!("I don't know who you are."),
      }
   }
}

mod mod_202310075
{
   pub fn fn_1()
   {
      let mut names = vec![String::from("Bob"), "Frank".to_owned(), "Ferris".to_string()];
      // let mut names_dereferenced_1 = names.deref();
      let mut names_dereferenced_1 = ::core::ops::DerefMut::deref_mut( & mut names );
      let ref mut names_dereferenced_2 = names[..];
      let names_dereferenced_3 = & mut * names;
      
      // for name in ( names_dereferenced_3 ).iter_mut()
      for name in ( & mut * names ).iter_mut()
      {
         *name =
            match ( & * name ).as_str()
            {
               "Ferris" => String::from("There is a rustacean among us!"),
               _ => String::from("Hello"),
            };
      
         // // cannot move out of `*name` which is behind a mutable reference
         // let s = *name;

         // // cannot move out of `*name` which is behind a mutable reference
         // drop(*name) ;
      }
      
      println!("names: {:?}", names);

      for name in names.into_iter()
      {
         // // This makes no sense
         // // becase {name} is already a {String} that has already been moved out of {names}.
         // // It's anyway about to be dropped.
         // drop(*name) ;
      }
   }
}

mod mod_202310081
{
   pub fn fn_1()
   {
      let s1 = "abc";
      let s2 = &*"xyz";

      // `to_owned` is faster than `to_string`.
      let mut farewell = "goodbye".to_owned();
      let mut val = s1.to_string();

      // A reference is dropped as soon as it's no longer needed,
      // so we can create multiple `mut` references.
      let val_ref_1 = & mut val;
      let val_ref_2 = & mut val;
   }
}

// https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html
mod mod_202310082
{
   pub fn fn_1()
   {
      let fn_plain = create_fn();
      let mut fn_mut = create_fnmut();
      let fn_once = create_fnonce();

      fn_plain();
      fn_mut();
      fn_once();

      fn_plain();
      fn_mut();
      // fn_once();
   }

   fn create_fn() -> impl Fn()
   {
      let text = "Fn".to_owned();
      move || println!("This is a: {}", text)
   }

   fn create_fnmut() -> impl FnMut()
   {
      let text = "FnMut".to_owned();
      move || println!("This is a: {}", text)
   }
   
   fn create_fnonce() -> impl FnOnce()
   {
      let text = "FnOnce".to_owned();
      move || println!("This is a: {}", text)
   }
}

mod mod_202310091
{
   struct GenVal<T> {
      gen_val: T,
   }

   impl<T> GenVal<T> {
      fn value(&mut self) -> &mut T {
         return & mut ( * self ).gen_val;
      }
   }

   pub fn fn_1()
   {
      let mut y = GenVal { gen_val: 3i32 };
      ( * ( & mut y ).value() ) = 123_i32;
      println!("{}", y.value());
   }
}

mod mod_202310092
{
   fn compare_types<T: ::core::fmt::Debug, U: ::core::fmt::Debug>(t: &T, u: &U) {
      println!("t: `{:?}`", t);
      println!("u: `{:?}`", u);
   }

   trait PrintInOption {
      fn print_in_option(self);
  }
  
  // Because we would otherwise have to express this as `T: Debug` or 
  // use another method of indirect approach, this requires a `where` clause:
  impl<T> PrintInOption for T where
      Option<T>: ::core::fmt::Debug {
      // We want `Option<T>: Debug` as our bound because that is what's
      // being printed. Doing otherwise would be using the wrong bound.
      fn print_in_option(self) {
          println!("{:?}", Some(self));
      }
   }
  
   pub fn fn_1()
   {
   }
}

mod mod_202310093
{
   struct ToDrop;

   impl Drop for ToDrop {
      fn drop(&mut self) {
         println!("ToDrop is being dropped");
      }
   }
   
   pub fn fn_1()
   {
      {
         let x = ToDrop;
         println!("Made a ToDrop 1!");
      }

      {
         // This will be dropped on the same line.
         #[allow(clippy::let_underscore_untyped)]
         #[allow(let_underscore_drop)]
         let _ = ToDrop;

         println!("Made a ToDrop 2!");
      }
  }
}

mod mod_202310101
{
   pub fn fn_1()
   {
      // let mut s = String::from("hello");
      // 
      // let r1 = &s; // no problem
      // let r2 = &s; // no problem
      // println!("{} and {}", r1, r2);
      // 
      // let r3 = &mut s; // error
      // println!("{}", r3);
      // 
      // println!("{}", r1);
   }
}

// https://doc.rust-lang.org/rust-by-example/scope/move/partial_move.html
mod mod_202310102
{
   #[derive(Debug)]
   struct Person {
       name: String,
       age: Box<u8>,
   }

   pub fn fn_1()
   {
      {
         let person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
         };
   
         // `name` is moved out of `person`, but `age` is referenced
         #[allow(clippy::ref_patterns)]
         let Person { name, ref age } = person;
   
         println!("The person's age is {}", age);
         println!("The person's age is {age}");
   
         println!("The person's name is {}", name);
         println!("The person's name is {name}");
   
         // Error! borrow of partially moved value: `person` partial move occurs
         //println!("The person struct is {:?}", person);
   
         // `person` cannot be used but `person.age` can be used as it is not moved
         println!("The person's age from person struct is {}", person.age);
      }

      // See help for the "clippy::ref_patterns" lint.
      // Destructuring by value consumes the value,
      // so in some cases it's better to destructure by reference.
      {
         let opt = Some(5);

         #[allow(clippy::ref_patterns)]
         if let Some(ref foo1) = opt
         {
            println!("{foo1}");
         }

         if let & Some(foo2) = & opt
         {
            println!("{foo2}");
         }
      }
   }
}

mod mod_202310103
{
   struct MyStruct
   {
      pub flag_1: bool,
   }

   impl MyStruct
   {
      pub fn flag_1(&self) -> bool
      {
         return self.flag_1;
      }
   }

   pub fn fn_1()
   {
   }
}

mod mod_202310111
{
   pub fn fn_1()
   {
      if cfg!(panic = "abort")
         { println!("abort"); }
      else
         { println!("unwind"); }

      fn_2();
   }

   #[cfg(panic = "abort")]
   pub fn fn_2()
   {
      println!("abort 2");
   }

   // #[cfg(panic != "abort")]
   #[cfg(panic = "unwind")]
   pub fn fn_2()
   {
      println!("unwind 2");
   }
}

mod mod_202310151
{
   use ::futures::executor::block_on;

   pub fn fn_1()
   {
      let future = hello_world(); // Nothing is printed
      block_on(future); // `future` is run and "hello, world!" is printed
      // future.await;
   }

   async fn hello_world() {
      println!("hello, world!");
  }
}

// mod mod_
// {
//    pub fn fn_1()
//    {
//    }
// }

// mod mod_
// {
//    pub fn fn_1()
//    {
//    }
// }

// mod mod_
// {
//    pub fn fn_1()
//    {
//    }
// }

// mod mod_
// {
//    pub fn fn_1()
//    {
//    }
// }

// mod mod_
// {
//    pub fn fn_1()
//    {
//    }
// }

// mod mod_
// {
//    pub fn fn_1()
//    {
//    }
// }
