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

use mini_redis::{client, Result};

// fn main()
// {
//    mod_202310151::fn_1();
// }
//
// // https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
// mod mod_202310151
// {
//    // `block_on` blocks the current thread until the provided future has run to
//    // completion. Other executors provide more complex behavior, like scheduling
//    // multiple futures onto the same thread.
//    use ::futures::executor::block_on;
//
//    pub fn fn_1()
//    {
//       let future = hello_world(); // Nothing is printed
//       block_on(future); // `future` is run and "hello, world!" is printed
//       // future.await;
//    }
//
//    async fn hello_world()
//    {
//       println!("hello, world!");
//    }
// }

#[tokio::main]
async fn main() -> Result<()>
{
   // Open a connection to the mini-redis address.
   let mut client = client::connect("127.0.0.1:6379").await?;

   // Set the key "hello" with value "world"
   client.set("hello", "world".into()).await?;

   // Get key "hello"
   let result = client.get("hello").await?;

   println!("got value from the server; result={:?}", result);

   Ok(())
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

// mod mod_
// {
//    pub fn fn_1()
//    {
//    }
// }
